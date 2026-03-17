# 定期出力・設定永続化問題の調査サマリー

---
作成: Session 229 (2026-03-17)
対象: Session 199-229
---

## 概要

Phase 3（複数台同時対応）の実機テスト中に発生した問題の調査経緯と解決策をまとめる。

---

## 背景（Session 194-198）

**目的**: 複数台のF9Pを同時接続して検査する機能の実装

**達成事項**:
- MultiDeviceManager実装（Session 194-196）
- 2台同時接続成功（Session 197）
- パス指定API追加（Session 198）

**発覚した問題**: 古い機（RTK基準局として使用されていた）でパースエラー発生

---

## 問題1: 定期出力が止まらない（Session 199-210）

### 状況

- `disable_periodic_output` で定期出力を無効化しようとしても、古い機では大量のメッセージが流れ続ける
- パーサーが追いつかずパースエラーが発生

### 対処の変遷

| Session | 対応 | キー数 | 結果 |
|---------|------|--------|------|
| 199-201 | NAV-TIMEQZSS等追加 | 24 | 不十分 |
| 209 | 5メッセージ追加 | 34 | 不十分 |
| 210 | 8メッセージ追加 | 50 | まだ3種類出力 |

### 結論

**個別メッセージ追加では際限がない** → 根本対処（CFG-CFGによるBBRクリア）を検討

---

## 問題2: CFG-CFGが効かない（Session 211-216）

### 状況

- CFG-CFG（clearMask）でBBRクリアを実装
- 実機テストで効果なし（定期出力が消えない）

### 原因

1. **deviceMask不備**: BBRのみ対象 → Flashにも設定が保存されていた
2. **loadMask問題**: loadMask=ALLでボーレートがROMデフォルト（9600bps）に戻る

### 対処

- deviceMaskをBBR+Flashに変更（Session 214）
- loadMask=NONEに変更（Session 215）
- ADR-015: loadMask使用禁止を記録

### 確認結果

Session 216で確認 → 定期出力は既に0件（以前のテストで消えていた）

---

## 問題3: 設定がUSB抜き差しで消える（Session 218-229）

### 状況

- `set-periodic-output` で定期出力を有効化
- USB抜き差し後、`message-scan` で0件（設定が消えている）

### 調査の流れ

| Session | 調査内容 | 発見 |
|---------|----------|------|
| 218 | set-periodic-output API作成 | UART1キー不足 |
| 219 | USB抜き差しテスト | BBRが消える？ |
| 220 | 仕様確認 | Layer::RamBbrFlash追加 |
| 221-222 | Flash書き込みテスト | Flashに書いてもUSB抜き差しで消える |
| 223 | CFG-VALGET API作成 | レイヤー別に値を確認可能に |
| 224 | **真因特定** | **BBRに0が残っていてFlashの1より優先** |
| 225 | 深掘り | BBRの「値の存在」、USB vs UART1 |
| 226-227 | 対策実装 | disable_periodic_outputをRAMのみに変更 |
| 227 | 仕様確認 | CFG-VALDELでBBR削除可能 |
| 228 | CFG-VALDEL実装 | BBR削除成功 |
| 229 | **解決確認** | **USB抜き差し後もFlash値有効** |

### 真因（Session 224で特定）

**u-bloxレイヤー優先順位**: RAM > BBR > Flash > Default

**問題**:
1. `disable_periodic_output` が `Layer::RamAndBbr` に0を書き込んでいた
2. BBRに0が記録される
3. Flashに1を書いても、起動時にBBRの0が優先される
4. 結果: Flashの設定が無視される

### 解決策

**方法A: disable_periodic_outputをRAMのみに変更**（Session 226で実装済み）
- BBRに0を書き込まない
- 新しいデバイスでは問題なし

**方法B: CFG-VALDELでBBRを削除**（Session 228で実装済み）
- 既存のBBR設定を削除
- Flashの値が参照されるようになる

### 確認済み手順

```bash
# 1. Flashに定期出力設定を書き込み
make set-periodic-output-flash

# 2. BBRから設定を削除
make cfg-valdel-bbr

# 3. USB抜き差し後、BE再起動
make dev-backend

# 4. 確認（定期出力無効化をスキップして接続）
make connect-raw
make message-scan  # NAV-PVT等が検出されればOK
```

---

## 作成したAPI・ツール

| セッション | API/ツール | 用途 |
|------------|-----------|------|
| 202 | `GET /api/devices/{path}/message-scan` | 定期出力スキャン |
| 212 | `POST /api/devices/{path}/reset-config` | 設定リセット（BBR+Flashクリア） |
| 218 | `POST /api/devices/{path}/set-periodic-output` | 定期出力有効化 |
| 221 | `GET /api/devices/{path}/mon-ver` | モジュール情報取得 |
| 223 | `GET /api/devices/{path}/cfg-valget` | 設定値読み出し |
| 228 | `DELETE /api/devices/{path}/cfg-valdel` | 設定値削除 |

---

## 重要な学び

### 1. u-bloxレイヤー優先順位

```
起動時の設定読み込み順序（高→低）:
RAM > BBR > Flash > Default
```

- **BBRに値があるとFlashは無視される**
- 値が「0」でも「存在する」ことが重要
- BBRを削除すると次のレイヤー（Flash）が参照される

### 2. USB vs UART1

- PC接続はUSBだが、**組み込みボードは内部的にUART1として動作**
- NAV_PVT_USB=0でもNAV_PVT_UART1=1なら出力される
- 両方のキーを設定する必要がある

### 3. 仕様書確認の重要性

- 推測で進めると遠回りする
- 仕様書（Interface Description）を確認してから実装

---

## 関連ドキュメント

| ドキュメント | 内容 |
|-------------|------|
| [38-ublox-config-management.md](../../docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md) | 設定管理の仕組み（レイヤー、コマンド） |
| [32-cfg-msgout-periodic-output.md](../../docs/missions/m1-sensor-evaluation/gnss/32-cfg-msgout-periodic-output.md) | 定期出力キー一覧 |
| [ADR-015](../../docs/adr/m1/ADR-015-cfg-cfg-loadmask.md) | loadMask使用禁止の理由 |
