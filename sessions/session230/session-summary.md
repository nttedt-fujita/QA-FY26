# Session 230 サマリー

**日付**: 2026-03-17

**目的**: Living Documentation方針の確定 + 断捨離計画の策定

---

## 実施内容

### 1. 現状調査

- リポジトリ全体のドキュメント構造を確認
- docs/: 43ファイル（gnss/）、ADR、技術調査資料
- sessions/: 657個のMarkdownファイル
- 重複ファイル: 10件以上（sessions/ と docs/ に同一）

### 2. 問題の特定

| 問題 | 具体例 |
|------|--------|
| 📝 sessionマークが移動されない | CFG-CFG(session211), CFG-VALGET(session214) |
| 同じ仕様を再抽出 | Configuration layers を Session 220 と 227 で2回抽出 |
| 重複ファイル | sessions/session5/*.md と docs/technical-research/*.md が同一 |

### 3. Living Documentation方針の確定

**原則**:
1. 仕様書（PDF）が1次情報
2. コードが実装の真実（出典をコードに書く）
3. 抽出ドキュメントはClaude用に必要（PDFを直接読めないため）
4. 抽出は docs/ に正式配置、sessions/ に放置しない
5. 目次 + 抽出状態テーブルで管理

**出典**: [Living Documentation (Cyrille Martraire, 2019)](https://www.oreilly.com/library/view/living-documentation-continuous/9780134689418/)

### 4. 断捨離計画の策定

| フェーズ | 内容 |
|----------|------|
| 2-1 | 重複ファイル削除（10件） |
| 2-2 | 📝 sessionマーク移動（2件） |
| 2-3 | 未掲載抽出物の確認・判断 |
| 3 | ルールファイル改善 |

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [living-documentation-concept.md](living-documentation-concept.md) | 方針と断捨離計画（次セッション用背景情報） |

---

## 結論

- **方針確定**: Living Documentationの考え方をこのプロジェクトに適用する方針を定めた
- **次のステップ**: 断捨離作業（重複削除、📝session移動）→ ルール改善
- **背景情報**: [living-documentation-concept.md](living-documentation-concept.md) に詳細をまとめた

---

## 次セッションでやること

1. 重複ファイル削除（10件）
2. 📝 sessionマーク移動（CFG-CFG, CFG-VALGET）
3. 未掲載抽出物の確認・判断
4. ルールファイル改善（14-document-management.md）

---

*作成: Session 230 (2026-03-17)*
