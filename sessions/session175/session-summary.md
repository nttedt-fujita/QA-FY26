# Session 175 サマリー

**日時**: 2026-03-13
**目的**: 生データ保存機能の動作確認 → 競合問題の調査・方針検討

---

## 実施内容

### 1. 屋外テスト実施

- DBクリーン後に屋外でテスト実施
- 検査10件、スナップショット15件が保存された
- **問題発覚**: 30秒の検査で1-2回しかスナップショットが取れない

### 2. 問題の特定

**BEログから判明した事実**:
- 1回の`/api/gnss-state`呼び出しに16秒かかっている
- 各メッセージで2秒タイムアウトが連発
- NTRIPとのロック競合: `⚠️ ロック待機が長い！(14849ms)`

**根本原因**:
- gnss_state_apiとntrip_apiが同じ`std::sync::Mutex<DeviceManager>`を取り合い
- FEの30秒タイマーとBEの実態が乖離

### 3. 解決方針の検討（AとD）

| 案 | 内容 | 検討結果 |
|----|------|----------|
| **A. 読み書き分離** | シリアルポートを分離して競合解消 | **有力** - serialport v4の`try_clone()`で実現可能 |
| **D. タイムアウト短縮** | 2秒→短縮 | 未検討（根拠確認が必要） |

**案A2（try_clone）の設計**:
```
DeviceManager (読み込み用) ─┬─ receive_ubx, send_ubx
                            │
                            └─ try_clone() → NTRIP Writer (書き込み専用)
```

---

## 確認した1次情報

| ファイル | 内容 |
|----------|------|
| [gnss_state_api.rs:63](../../../prototype/m1-gnss/backend/src/web/gnss_state_api.rs#L63) | タイムアウト2秒の設定箇所 |
| [gnss_state_api.rs:184](../../../prototype/m1-gnss/backend/src/web/gnss_state_api.rs#L184) | ロック取得（読み込み側） |
| [ntrip_api.rs:502](../../../prototype/m1-gnss/backend/src/web/ntrip_api.rs#L502) | ロック取得（書き込み側） |
| [ADR-012](../../../docs/adr/m1/ADR-012-periodic-output-and-unified-api.md) | 統合API採用の経緯、定期出力は無効化済み |

**外部ドキュメント**:
- [serialport docs](https://docs.rs/serialport/4/serialport/trait.SerialPort.html) - `try_clone()`サポート確認
- [serial2 crate](https://docs.rs/serial2) - 代替案（concurrent read/writeサポート）

---

## 次セッションでやること

### 優先度高（KISS原則に従った順序）
1. **案D（タイムアウト短縮）を先に検討**
   - タイムアウト2秒の根拠をUBX仕様書で確認
   - 短縮だけで問題解決できないか確認
   - **最もシンプルな解決策**

2. **案Dで不十分なら案A（try_clone）**
   - try_clone()の実装箇所特定
   - DeviceManagerの変更範囲
   - 注意: try_cloneドキュメントに「真の非同期ならmio-serial/tokio-serialを検討」と記載あり

3. **async化（mio-serial/tokio-serial）は最後の手段**
   - 複雑度が大幅に上がる
   - 本当に必要か慎重に判断

### 優先度中
3. **ログ分析**（時間があれば）
   - 1.4MB/12175行のログを時間帯別に分割
   - 正常動作していた時間帯の特定

---

## 参照

- [session-plan.md](session-plan.md) - 当初の計画
- [session174/session-summary.md](../session174/session-summary.md) - 前セッション
- [raw-data-storage-plan.md](../session172/raw-data-storage-plan.md) - 生データ保存設計
