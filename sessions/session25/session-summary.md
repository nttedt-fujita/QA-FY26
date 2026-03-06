# Session 25 サマリー

**実施日**: 2026-03-06
**主な作業**: 品質管理フレームワーク調査、プロトタイプ方針策定、ヒアリング項目統合

---

## 実施内容

1. **午前打ち合わせ内容の整理**
   - 宇枝さんの要望: 「良くなったか見たい」「原因確認したい」「現状把握したい」
   - 目標: 受入検査月300時間 → 150時間
   - 課題: 品質協定書（ナカヨ、渡辺製作所）未締結

2. **品質管理フレームワーク調査**
   - IQC/PQC/OQCの関係
   - ロット管理・トレーサビリティ
   - AQL・抜取検査（JIS Z 9015）
   - 8Dレポート（問題解決フレームワーク）
   - 品質協定書の役割
   - QC7つ道具

3. **M3/M4プロトタイプ方針策定**
   - Phase 1: 分析・可視化（今できる）
   - Phase 2: 入力のデジタル化（ヒアリング後）
   - Phase 3: M3/M4統合 + トレーサビリティ（協定書締結後）

4. **ヒアリング項目統合**
   - Session 5, 8, 24の項目を統合
   - Session 25の調査結果から5項目追加
   - docs/missions/m3-incoming-inspection-db/ に配置

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [sessions/session25/meeting-notes-am.md](meeting-notes-am.md) | 午前打ち合わせメモ |
| [sessions/session25/quality-framework-research.md](quality-framework-research.md) | 品質管理フレームワーク調査レポート |
| [sessions/session25/prototype-approach.md](prototype-approach.md) | M3/M4プロトタイプ方針 |
| [docs/missions/m3-incoming-inspection-db/hearing-items.md](../../docs/missions/m3-incoming-inspection-db/hearing-items.md) | ヒアリング項目統合版（更新） |

---

## 主な発見

### 品質管理フレームワーク

| トピック | 発見 | M3/M4への示唆 |
|---------|------|--------------|
| IQC/PQC/OQC | 品質管理は工程の流れに沿って構成 | M3=IQC、M4=IPQC/PQC、両者は「部品」で紐づく |
| ロット/トレーサビリティ | ロット番号が追跡の基本単位 | 現行Excelにはロット概念がない（To-Beで導入） |
| AQL/抜取検査 | AQL基準があれば全数→抜取で効率化可能 | プロポ検査の効率化に使えるか要確認 |
| 8Dレポート | 問題解決の標準フレームワーク | 宇枝さんの「原因確認」「効果確認」と一致 |
| 品質協定書 | 委託先管理の法的根拠 | **協定書なしではM4ツール導入が困難** |

### プロトタイプ方針

| 判断 | 根拠 |
|------|------|
| Phase 1（分析）から始める | データ構造確定済み、ツール完成済み |
| 入力フォームは後回し | As-Isモデル未確定、ヒアリングが必要 |
| M4は協定書締結を待つ | 協定書なしでは委託先にツール使用を強制できない |

---

## 残った課題

- [ ] ヒアリング実施（P0〜P2）
- [ ] Phase 1プロトタイプ（分析ダッシュボード）の設計・実装
- [ ] 品質協定書締結の支援（M4の前提）

---

## 次セッション（Session 26）でやること

1. **Phase 1プロトタイプの検討**
   - 分析ダッシュボードの設計（Streamlit or Jupyter）
   - 既存ツール（tools/incoming_inspection/）の拡張

2. **（余裕があれば）M1-B GNSS関連**
   - 合格基準のエビデンス収集

---

## 参考資料

- [Session 24 summary](../session24/session-summary.md)
- [Session 8: hearing-items-integrated.md](../session8/hearing-items-integrated.md)
