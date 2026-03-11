# Session 105 サマリー

**日付**: 2026-03-11
**目的**: 屋外検査項目の調査・設計

---

## 実施内容

### 1. セッション履歴の修正

Session 104の履歴に「200回連続テストPass」の記載漏れを発見・修正。

### 2. 屋外検査項目の調査

過去のセッション資料を確認し、元々のExcel（小峰無線GPS確認.xlsx）で確認していた情報を整理:

| シート | 確認内容 | ツール化 |
|--------|---------|---------|
| 内部設定 | FW/PROTVER/パラメータ | **Phase 1で実装済み** |
| 受信感度 | C/N0（3回測定、仰角別） | NAV-SIG |
| スペアナ | RF波形（L1/L2別） | MON-SPAN |
| 飛行試験 | RTK FIX時間、精度、衛星数 | NAV-STATUS, NAV-PVT |
| MAG確認 | 磁気コンパスずれ | — |
| 電池確認 | バックアップ電池 | — |

### 3. 発見した問題

1. **NAV-SAT vs NAV-SIG**: 07-cross-sheet-findings.mdでは**NAV-SIG**が推奨されている（L1/L2別）
2. **MON-SPANの仕様未確認**: 128MHz/256点のスペクトラム取得方法が不明
3. **要求整理が先**: 合格基準・ヒアリング結果を確認してから設計すべき

---

## 作成ファイル

| ファイル | 内容 | 備考 |
|----------|------|------|
| [outdoor-inspection-design.md](outdoor-inspection-design.md) | 屋外検査設計（ドラフト） | **不完全** — 要求再整理後に見直し |

---

## 次セッション（Session 106）でやること

1. **u-blox仕様書確認**: MON-SPAN (p.134) の詳細仕様
   - メッセージ構造（Class/ID、ペイロード形式）
   - RF1/RF2の選択方法
   - 256点が128MHzをどうカバーするか
2. **要求再整理**: 屋外検査で何をテストするか確定
   - 小板橋さん・末永さんヒアリング結果の確認
   - 合格基準の確定状況

---

## 参照資料

- [07-cross-sheet-findings.md](../../docs/missions/m1-sensor-evaluation/gnss/07-cross-sheet-findings.md) — 全シート横断の発見事項
- [03-spectrum-analyze.md](../../docs/missions/m1-sensor-evaluation/gnss/03-spectrum-analyze.md) — スペアナ分析
- [08-ubx-protocol-index.md](../../docs/missions/m1-sensor-evaluation/gnss/08-ubx-protocol-index.md) — UBXプロトコル索引

---

*作成: 2026-03-11 Session 105終了時*
