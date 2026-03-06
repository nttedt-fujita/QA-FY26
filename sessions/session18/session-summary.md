# Session 18 サマリー

**実施日**: 2026-03-06
**主な作業**: Excel画像の読み取り・分析 → Session 17テキスト分析との統合ドキュメント作成

---

## 実施内容

1. Excel画像29枚の読み取り・分析（受信感度14枚、スペアナ5枚、飛行ログ6枚、MAG確認4枚）
2. Session 17のテキスト分析と画像分析を統合し、`docs/missions/m1-sensor-evaluation/gnss/` 配下にシートごとのドキュメントを作成
3. 横断的な発見事項・合格基準の叩き台・末永さんヒアリング項目10問を整理

---

## 主な発見

### No.5の異常が全データソースで裏付けられた

| データソース | No.5の異常 | 定量値 |
|------------|-----------|--------|
| 受信感度 | Q2 L1が突出して低い | 13dBHz（他: 30〜48） |
| 受信感度 | L2欠損が多い | GPS衛星のL2が多数「–」 |
| スペアナ | RF2（L2帯）PGA異常 + 波形異常 | 38dB（他: 54dB）、スパイク状 |
| 飛行試験 | RTK FIX不可、衛星数少、ジャミング大 | 5分以上NG、衛星19（他: 25〜27） |

**結論**: L2帯の受信不良がRTK FIX不可の根本原因。L1帯は正常。

### その他の発見

- **No.02のMAGが45度ずれ** → 2/8にキャリブで解消済み
- **QZSS L2は全機体（Ref含む）で受信不可** → 設定/仕様の問題
- **Ref機と実機のC/N0差は5〜15dBHz** → アンテナ取付環境の影響か
- **ノイズ・ジャミングスパイクは全機体で発生** → 環境要因

---

## 作成ファイル

### 統合ドキュメント（docs/missions/m1-sensor-evaluation/gnss/）

| ファイル | 内容 |
|---------|------|
| [README.md](../../docs/missions/m1-sensor-evaluation/gnss/README.md) | インデックス |
| [01-internal-settings.md](../../docs/missions/m1-sensor-evaluation/gnss/01-internal-settings.md) | 内部設定（FW/PROTVER/パラメータ） |
| [02-reception-sensitivity.md](../../docs/missions/m1-sensor-evaluation/gnss/02-reception-sensitivity.md) | 受信感度 全3回データ + u-center画像分析 + 仰角別整理 |
| [03-spectrum-analyze.md](../../docs/missions/m1-sensor-evaluation/gnss/03-spectrum-analyze.md) | スペアナ波形分析（PGA値・波形形状） |
| [04-flight-test.md](../../docs/missions/m1-sensor-evaluation/gnss/04-flight-test.md) | 飛行試験テーブル + 飛行中ログ時系列 |
| [05-mag-check.md](../../docs/missions/m1-sensor-evaluation/gnss/05-mag-check.md) | MAG確認（GCSスクリーンショット分析） |
| [06-battery-check.md](../../docs/missions/m1-sensor-evaluation/gnss/06-battery-check.md) | バックアップ電池確認 |
| [07-cross-sheet-findings.md](../../docs/missions/m1-sensor-evaluation/gnss/07-cross-sheet-findings.md) | 横断発見事項・合格基準叩き台・ヒアリング項目10問 |
| [08-ubx-protocol-index.md](../../docs/missions/m1-sensor-evaluation/gnss/08-ubx-protocol-index.md) | UBX仕様書索引（Phase 2用） |

### セッション作業ファイル（sessions/session18/）

| ファイル | 内容 |
|---------|------|
| [reception-sensitivity-analysis.md](reception-sensitivity-analysis.md) | 受信感度画像分析（作業用、統合済み） |
| [spectrum-analyze-analysis.md](spectrum-analyze-analysis.md) | スペアナ画像分析（作業用、統合済み） |
| [flight-log-20260218-analysis.md](flight-log-20260218-analysis.md) | 飛行ログ画像分析（作業用、統合済み） |
| [mag-check-analysis.md](mag-check-analysis.md) | MAG確認画像分析（作業用、統合済み） |

---

## 未完了・スキップした項目

- 電池確認シートの画像（意味が薄いと判断しスキップ）
- No.02シートの画像（同上）
- スペアナ4, 5.png（小さく不鮮明。HORIBA参考品との比較データ。大きいスクショで補完可能）
- 受信感度1.png（全体俯瞰のため詳細不可）

---

## 次セッション（Session 19）でやること

→ [session19/session-plan.md](../session19/session-plan.md) 参照
