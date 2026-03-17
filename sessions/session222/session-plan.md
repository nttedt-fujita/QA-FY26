# Session 222 計画

**目的**: 評価ボードのFlash有無調査 + 対応策検討

**前提**: Session 221でset-periodic-outputがUSB抜き差しで消える問題を確認

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | 評価ボードの種類・仕様確認 | - | - |
| 2 | ZED-F9P Integration ManualでFlash要件確認 | docs/missions/.../gnss/README.md | - |
| 3 | 対応策の検討・決定 | - | - |

---

## 詳細

### 1. 評価ボードの種類・仕様確認

- 評価ボードのメーカー・型番を確認
- 外部Flashメモリの有無を確認
- バッテリーバックアップの有無を確認

### 2. ZED-F9P Integration ManualでFlash要件確認

- Integration Manual（IM）からFlash関連の仕様を抽出
- 外部Flashメモリの接続要件を確認

### 3. 対応策の検討

**Flashがない場合の代替案**:
1. 毎回接続時にset-periodic-outputを実行（ソフトウェア対応）
2. 評価ボードにFlashを追加（ハードウェア変更）
3. バッテリーバックアップを追加（ハードウェア変更）
4. reset-config機能の運用変更（接続時に自動設定）

---

## 参照

- [Session 221 summary](../session221/session-summary.md)
- [config-layers-spec.md](../session220/config-layers-spec.md) - BBR/Flash仕様
