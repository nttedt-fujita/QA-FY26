# M3レビュー・AI見積もり調査 計画書

**作成日**: 2026-03-18（Session 238）
**目的**: M3プロトタイプの現状確認、AI調査資料との整合性確認、説明用スライド作成

---

## 背景

- Session 185から「AI組み合わせ見積もり調査」が放置中
- Session 236にAI調査資料（M3M4tools-AI-research/）が配置された
- M3プロトタイプはSession 42-47で完成、Session 52で一旦ストップ
- Living Documentation導入後のメンテナンスができていない

---

## 現状マップ（Session 238で確認済み）

### プロトタイプ状態

| 項目 | 状態 |
|------|------|
| Phase 2プロトタイプ | ✅ 完成（Session 42-47） |
| アーキテクチャドキュメント | ✅ prototype/m3/docs/ARCHITECTURE.md |
| 技術スタック | Go + Next.js + PostgreSQL（確定） |
| ADR | 3件承認済み（ADR-001〜003） |

### 方針状態

| 項目 | 状態 | 課題 |
|------|------|------|
| M3/M4優先順位 | ⏸️ M3ストップ、M4優先 | Session 52で決定済み |
| platform-comparison.md | 📝 古い（Session 5） | 要更新 |
| prototype-approach.md | 📝 古い（Session 25） | 要更新 |

### AI調査資料（session236/M3M4tools-AI-research/）

| ファイル | 内容 |
|----------|------|
| 01_define_phase_report | DMAIC Defineフェーズ |
| 06_ai_visual_inspection | AIサービス比較 |
| 07_ai_integration_and_cost | AI連携設計・コスト分析 |
| 08_lean_improvement_proposals | 4つの提案（推奨: A+B） |
| 09_updates_and_supplements | 補足・修正 |

---

## セッション別計画

### Session 239: 古いドキュメント整理

**目的**: Living Documentation準拠のドキュメント整理

**やること**:
1. `docs/missions/m3-incoming-inspection-db/to-be/` のドキュメント更新
   - `platform-comparison.md` — 現在の方針（M4優先、プロトタイプ完成済み）を反映
   - `prototype-approach.md` — Phase進捗と優先順位変更を反映
2. Session 52の方針変更をADR化（必要なら）
3. `hearing/` ディレクトリの状態確認・整理

**成果物**:
- 更新されたto-be/ドキュメント
- M3方針の現状整理

**読むべきファイル**:
- `docs/missions/m3-incoming-inspection-db/to-be/platform-comparison.md`
- `docs/missions/m3-incoming-inspection-db/to-be/prototype-approach.md`
- `sessions/session52/session-summary.md`（方針変更の詳細確認）

---

### Session 240: AI連携要件の確認

**目的**: 現プロトタイプがAI連携設計（Must/Should/Could）を満たしているか確認

**やること**:
1. `prototype/m3/docs/ARCHITECTURE.md` を読む
2. `session236/M3M4tools-AI-research/07_ai_integration_and_cost_analysis.md` のMust/Shouldと照合
3. ギャップがあれば記録

**チェック項目（07より）**:

| 分類 | 項目 | 確認 |
|------|------|------|
| Must | 検査画像の保存機能 | ? |
| Must | 画像と検査メタデータの紐付け | ? |
| Must | 不良モードの統一分類コード | ? |
| Must | 良品/不良品のラベル付け | ? |
| Must | データエクスポート機能 | ? |
| Should | S3互換ストレージの採用 | ? |
| Should | AI判定結果の記録フィールド | ? |
| Should | フィードバックUIの設計 | ? |

**成果物**:
- AI連携要件チェック結果
- 現プロトタイプのギャップ分析

**読むべきファイル**:
- `prototype/m3/docs/ARCHITECTURE.md`
- `session236/M3M4tools-AI-research/07_ai_integration_and_cost_analysis.md`

---

### Session 241: AI見積もり具体化

**目的**: 導入判断のためのコスト試算・ROI計算

**やること**:
1. 現行コスト（人件費）の確認
   - 月間検査工数: 300h → 実際の数値を確認
   - 時給想定の確認
2. AI構成別コスト試算（07の数値をベースに）
   - 構成C（マルチモーダルLLM）が最も現実的
3. ROIシミュレーション
4. 導入判断の材料整理

**成果物**:
- AI導入コスト試算書
- 導入判断の推奨

**参照資料**:
- `session236/M3M4tools-AI-research/07_ai_integration_and_cost_analysis.md`
- `session236/M3M4tools-AI-research/08_lean_improvement_proposals.md`

---

### Session 242: Marpスライド作成

**目的**: 説明用スライドの作成

**やること**:
1. スライド構成の決定
   - 現状（M3プロトタイプ完成、M4優先）
   - AI調査結果サマリー
   - コスト試算・ROI
   - 推奨アクション
2. Marp形式でスライド作成
3. 画像・図の配置

**成果物**:
- Marp形式スライド（.md）
- PDF出力（必要なら）

**スキル参照**:
- `~/.claude/skills/marp-slides/SKILL.md`

---

## 全体スケジュール

```
Session 238（今回）: 状況把握・計画立て ✅
    │
Session 239: 古いドキュメント整理
    │
Session 240: AI連携要件の確認
    │
Session 241: AI見積もり具体化
    │
Session 242: Marpスライド作成
```

---

## 関連資料

- [M3 README](../../docs/missions/m3-incoming-inspection-db/README.md)
- [M3M4tools-AI-research/](./M3M4tools-AI-research/)
- [prototype/m3/docs/ARCHITECTURE.md](../../prototype/m3/docs/ARCHITECTURE.md)

---

*作成: Session 238 (2026-03-18)*
