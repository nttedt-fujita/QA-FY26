# Session 172 サマリー

**日付**: 2026-03-13
**目的**: RTK屋外テスト予定 → 生データ保存機能の計画立案に変更

---

## 実施内容

### 1. 問題の認識

ユーザーからの指摘:
- RTKフィックス状態がFE経由でしかわからない
- BEだけでテストしても結果を可視化できない
- DBに保存されたデータからスカイプロット等を再表示したい

### 2. 現状調査

**確認した資料・コード**:
- ドメインモデル: 19-gnss-unified-domain-model.md
- DBスキーマ: schema.sql
- 設計資料: session105/outdoor-inspection-design.md、session106/outdoor-inspection-needs.md
- BE実装: repository/types.rs、outdoor_inspection_api.rs
- FE実装: useOutdoorInspection.ts、outdoor-inspection.ts、api.ts

**発見**:
- 既存スキーマに詳細データ用テーブル（outdoor_measurements、satellites等）が存在
- しかし実装されておらず、集計結果（outdoor_inspection_results）のみ使用中
- スキーマと実装の乖離がある

### 3. 設計計画書の作成

3つのオプションを検討:
- A: 新規スナップショットテーブル（シンプル）
- B: 既存outdoor_measurements系を活用（整合性高い）
- C: ハイブリッド（推奨）

**推奨案（オプションC）**:
- outdoor_inspection_snapshotsテーブルを新規追加
- GnssStateResponseをJSON形式で保存
- 既存outdoor_inspection_resultsとの後方互換性を維持

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [raw-data-storage-plan.md](raw-data-storage-plan.md) | 生データ保存機能の設計計画書 |

---

## 次セッション（Session 173）でやること

1. 計画書のレビュー・精査
2. オプションCで進めてよいか確認
3. 実装開始（Phase 1: BE）

---

## hooks振り返り

今回のセッションでは特になし。
設計・計画フェーズが中心だった。
