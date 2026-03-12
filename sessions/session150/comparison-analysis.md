# Session 130 vs 現在の比較分析

**作成**: Session 150 (2026-03-12)

---

## 1. Session 130の状態（安定版）

コミット: `4c53ecf`

### 実装内容
- 屋内検査（Connectivity, FwVersion, SerialNumber, OutputRate, PortConfig）
- 屋外検査（NAV-PVT/NAV-SATポーリング、30秒モニタリング）
- DB保存

### API構成
- `/api/devices` - デバイス一覧・接続・切断
- `/api/inspections` - 屋内検査
- `/api/outdoor-inspection-results` - 屋外検査結果
- `/api/nav-sat` - NAV-SAT個別API
- `/api/nav-sig` - NAV-SIG個別API
- `/api/mon-span` - MON-SPAN個別API

### 特徴
- **定期出力制御なし** - F9Pのデフォルト設定のまま
- **NMEA制御なし** - NMEAも出力されている可能性
- **個別API** - FEが各APIを順番に呼ぶ

---

## 2. Session 131-138: NTRIP追加

### 追加内容
- `/api/ntrip/connect` - NTRIP接続
- `/api/ntrip/disconnect` - NTRIP切断
- `/api/ntrip/settings` - NTRIP設定
- GGA送信機能
- device_id（F9Pシリアル番号）紐付け

### 影響
- `ntrip_api.rs` (884行) 追加
- デバイス接続時にF9Pシリアル番号取得
- CurrentPosition構造体追加（GGA送信用）

---

## 3. Session 139-144: 統合API実装

### 背景
- **API並行問題**: 複数APIが同時にUBXコマンドを送ると競合
- FEのポーリングが重なるとシリアル通信が破綻

### 解決策
- 統合API (`/api/gnss-state`) を実装
- 1回のAPIで全データを順次ポーリング: NAV-PVT → NAV-STATUS → NAV-SAT → NAV-SIG → MON-SPAN → MON-RF

### 追加ファイル
- `gnss_state_api.rs` (337行)

---

## 4. Session 145-149: 定期出力制御

### 背景
- 統合API実行中に定期出力が混入 → タイムアウト・パースエラー

### 対策
- `disable_periodic_output()` - 定期出力を無効化
- `send_disable_nmea_output()` - NMEA出力を無効化
- Layer::Ram → Layer::Bbr（不揮発性）

### 無効化対象メッセージ（USB + UART1）
| メッセージ | キー |
|-----------|------|
| NAV-PVT | CFG-MSGOUT-UBX_NAV_PVT_* |
| NAV-STATUS | CFG-MSGOUT-UBX_NAV_STATUS_* |
| NAV-SAT | CFG-MSGOUT-UBX_NAV_SAT_* |
| NAV-SIG | CFG-MSGOUT-UBX_NAV_SIG_* |
| MON-SPAN | CFG-MSGOUT-UBX_MON_SPAN_* |
| MON-RF | CFG-MSGOUT-UBX_MON_RF_* |

---

## 5. 現在の問題

### 症状
1. 「不明データあり」が頻発（500バイト前後）
2. タイムアウトが発生
3. でもAPIは200 OKを返す（時々）

### 疑問点
1. **定期出力無効化は効いているのか？**
   - Layer::Bbrに設定しても、電源リセットするまで効かない？
   - ACKは受信しているが、実際に設定が反映されているか不明

2. **「不明データ」の正体は？**
   - バイト列を見ると、スペクトラムデータ（dBHz値）のように見える
   - MON-SPANの一部がバッファに残っている可能性

3. **なぜSession 130では問題なかったのか？**
   - 定期出力が来ていても、個別APIは影響を受けにくかった？
   - 統合APIは連続ポーリングするため、定期出力と干渉しやすい？

---

## 6. 選択肢

### A. Session 130に戻す + NTRIPのみ追加

**メリット**:
- 安定していた実装に戻る
- シンプル

**デメリット**:
- API並行問題が再発する可能性
- 統合APIの利点（1回で全データ取得）を失う

**確認事項**:
- Session 130で本当に競合問題が起きていたか？
- 屋外検査中のFEポーリング頻度は？

### B. 統合APIを修正（定期出力無効化を正しく動作させる）

**メリット**:
- 根本解決
- 統合APIの利点を維持

**デメリット**:
- 根本原因の特定が必要
- 追加調査に時間がかかる

**調査事項**:
- u-centerで定期出力設定を確認
- Layer::Bbrが本当に効いているか
- 他に定期出力されているメッセージがないか

### C. 統合APIを使いつつ、定期出力制御をやめる

**メリット**:
- 中間的なアプローチ
- 定期出力との共存を試みる

**デメリット**:
- 根本解決ではない
- パフォーマンス問題が残る可能性

---

## 7. 次のアクション

1. **u-centerで確認**: 定期出力設定の現状を確認
2. **Session 130の競合問題を再確認**: 本当に問題があったか
3. **選択肢を決定**: A/B/Cのどれで進めるか
