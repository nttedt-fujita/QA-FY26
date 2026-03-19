# Session 270 サマリー

**日付**: 2026-03-19
**目的**: Linear運用方針の策定とスキル実装

---

## 実施内容

### 1. Linear運用方針の整理

**背景**:
- Session 269でLinear導入完了
- 進捗を他のメンバーと共有したい
- 予算制約（お金かかりそうになったら上司に相談）

**検討した観点**:
- Issue粒度（どこまで細かく管理するか）
- アーカイブ運用（アクティブissue数を抑える）
- 課金判断基準（150件で相談、200件で移行）
- 監視の仕組み（セッション開始時に自動確認）

**決定**:
- **Freeプランで運用開始**（250 active issues）
- **Issue粒度**: 中程度（作業単位）
- **アーカイブ**: Issue完了時に即座に実施
- **監視**: セッション開始時に自動確認（linear-managementスキル）

### 2. Living Documentationの視点でドキュメント配置を決定

**判断**: ADRとして記録（Transient Knowledge = 意思決定の経緯）

**理由**:
- プロジェクト管理のアプローチは「設計判断」
- 将来見直す可能性がある
- 他のADRと同じフォーマットで記録

**配置先**: `docs/adr/common/ADR-017-linear-free-plan-operation.md`

### 3. linear-managementスキル作成

**場所**: `~/.claude/skills/linear-management/SKILL.md`

**責務**:
- セッション開始時のアクティブissue数確認
- 監視基準に基づいて警告
- アーカイブ運用ガイド
- Issue作成ガイド

**監視基準**:
| アクティブissue数 | 状態 | 対応 |
|-----------------|------|------|
| 0-100 | ✅ 余裕 | 継続 |
| 100-150 | 🟡 注意 | 監視強化 |
| 150-200 | 🟠 警告 | 上司に相談 |
| 200-250 | 🔴 限界 | 移行必須 |

### 4. session-managementスキル更新

**追加内容**: 「0.5. Linear管理（該当プロジェクトのみ）」

**フロー**:
```
セッション開始
  ↓
energy-management（体調チェック）
  ↓
プロジェクトCLAUDE.mdを確認
  ↓
「Linear運用」セクションあり？
  ↓ Yes
linear-management（issue数確認・警告）
  ↓
session-management（履歴・前セッション確認）
  ↓
作業開始
```

### 5. ADR-017作成

**ファイル**: `docs/adr/common/ADR-017-linear-free-plan-operation.md`

**Status**: Accepted（2026-03-19、Session 270）

**Decision**:
- Freeプランで運用開始
- 150件到達時に上司に相談
- Issue粒度は中程度（作業単位）
- 完了時に即座にアーカイブ

### 6. QA-FY26/CLAUDE.md更新

**追加内容**:
- 「Linear運用」セクション（Workspace情報、監視基準）
- ADR一覧にADR-017追加

### 7. Linearに実装

**作成内容**:
- **Project**: M3+M4: 検査プロセス改善
- **Issues**: 3件作成
  - QA-5: SIPOC作成と現場レビュー準備
  - QA-6: プレート調査（暗黙知外部化）
  - QA-7: 梱包変更作業・隠れコストの定量調査

**現在のアクティブissue数**: 7件（デフォルト 4 + 新規 3）

---

## 主な発見

### 1. 方針を先に決めてからドキュメント化

- 「とりあえずやってみる」ではなく、方針を整理してからドキュメント化
- Living Documentationの視点で配置先を決定（ADR vs 運用ルール）

### 2. スキルとルールの責務分離

- session-management: 汎用的なセッション管理
- linear-management: ツール固有の運用（独立したスキル）
- 将来的に他ツール（GitHub Projects等）に切り替える可能性を考慮

### 3. 推測禁止の原則違反

**ユーザーの指摘**: 「GraphQL使うならAPIの仕様をきちんとWebで調べてからやればいいんじゃないの」

**問題点**:
- Linear GraphQL APIを推測で使用
- 公式ドキュメントを確認せずに実装
- エラーが出たら試行錯誤で対応

**教訓**: API実装時は必ず公式ドキュメントを確認してから実装

---

## 残った課題

### 1. Linear API公式ドキュメント調査（次セッション）

- GraphQL Schemaの確認
- Issue作成、Project作成、Milestone作成の正式な仕様
- ガントチャート（Roadmap view）の設定方法
- 状態管理（state）の仕様

### 2. メンバー招待の手順（次セッション）

- Guest vs Member権限の違い
- 招待手順のドキュメント化
- 宇枝さん、小笠原さん、石川さんへの招待

### 3. Issueの追加・アーカイブ（次セッション）

- 現在の進捗を全てLinearに反映
- 完了分をアーカイブ
- ガントチャート表示の確認

---

## 成果物

### 作成ファイル

| ファイル | 内容 |
|----------|------|
| `~/.claude/skills/linear-management/SKILL.md` | Linear管理スキル |
| `~/.claude/skills/session-management/SKILL.md`（更新） | Linear管理呼び出しを追加 |
| [docs/adr/common/ADR-017-linear-free-plan-operation.md](../../docs/adr/common/ADR-017-linear-free-plan-operation.md) | Linear運用方針（ADR） |
| [CLAUDE.md](../../CLAUDE.md)（更新） | Linear運用セクション追加、ADR-017追加 |

### Linearに作成

- Project: M3+M4: 検査プロセス改善（ID: 1cff36fe-12d1-4cd8-89c6-b2bb66db991d）
- Issue: QA-5, QA-6, QA-7（3件）

---

## 次セッションでやること

1. **Linear API公式ドキュメント調査**（最優先）
   - GraphQL Schemaの確認
   - 正式な仕様に基づいた実装

2. **メンバー招待手順の調査・実施**
   - Guest vs Member権限の違い
   - 宇枝さん、小笠原さん、石川さんへの招待方法

3. **Issueの追加・アーカイブ**
   - 現在の進捗を全てLinearに反映
   - 完了分をアーカイブ
   - ガントチャート表示の確認

4. **宇枝さん説明資料作成**（Session 270計画の本来の目的）
   - AI導入の現実を伝える資料
   - 検査プロセス改善の進捗報告

---

## 参照

- [session269/session-summary.md](../session269/session-summary.md) — 前セッションサマリー
- [docs/adr/common/ADR-017-linear-free-plan-operation.md](../../docs/adr/common/ADR-017-linear-free-plan-operation.md) — Linear運用方針（ADR）
- [docs/tools/linear/issue-estimate.md](../../docs/tools/linear/issue-estimate.md) — Issue数見積もり（Session 268）
- `~/.claude/skills/linear-management/SKILL.md` — Linear管理スキル

---

**次セッション**: [session271/session-plan.md](../session271/session-plan.md)
