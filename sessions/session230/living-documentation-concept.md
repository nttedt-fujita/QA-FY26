# Living Documentation 方針と断捨離計画

---
作成: Session 230 (2026-03-17)
目的: Living Documentationの方針を確定し、断捨離作業の背景情報を整理
---

## 出典

| 資料 | 内容 |
|------|------|
| [Living Documentation (Cyrille Martraire, 2019)](https://www.oreilly.com/library/view/living-documentation-continuous/9780134689418/) | 原典。DDDの考え方を取り入れたドキュメンテーション手法 |
| [InfoQ Q&A with Cyrille Martraire](https://www.infoq.com/articles/book-review-living-documentation/) | 著者インタビュー。4原則の解説 |

---

## 背景

### なぜこの議論が必要になったか

1. **ルール導入前からプロジェクトが開始**されていた
2. sessions/にドキュメントが**657個**蓄積
3. **重複ファイルが10件以上**存在（sessions/ と docs/ に同一）
4. **📝 sessionマーク**がsessions/に放置されている
5. **同じ仕様を何度も再抽出**する問題が発生

### 組み込み開発特有の制約

```
Claude Code は PDF を直接読めない（セッションが止まる）
    ↓
仕様書（PDF）から抽出した Markdown が必要
    ↓
抽出ドキュメントの管理が重要
```

---

## 確定した方針

### 原則

| # | 原則 | 説明 |
|---|------|------|
| 1 | **仕様書（PDF）が1次情報** | 変更不可の外部情報源 |
| 2 | **コードが実装の真実** | コードに出典（ページ番号）を書く |
| 3 | **抽出ドキュメントはClaude用に必要** | PDFを直接読めないため |
| 4 | **抽出は docs/ に正式配置** | sessions/ に放置しない |
| 5 | **目次 + 抽出状態テーブルで管理** | 再抽出を防ぐ |

### 情報の階層

```
┌─────────────────────────────────────────────────┐
│  外部1次情報（変更不可）                          │
│  - u-blox仕様書（PDF）                           │
│  - 標準仕様（RFC等）                              │
└─────────────────────────────────────────────────┘
                    ↓ 参照して実装
┌─────────────────────────────────────────────────┐
│  内部の真実（Single Source of Truth）            │
│  - コード（仕様を実装したもの）                   │
│  - テスト（振る舞いを記述したもの）               │
│  - コード内に「出典: IF p.93」と書く             │
└─────────────────────────────────────────────────┘
                    ↓ 補足（最小限）
┌─────────────────────────────────────────────────┐
│  派生情報                                        │
│  - 索引（どこを見ればいいか）                    │
│  - 抽出ドキュメント（Claude用）                  │
│  - ADR（設計判断の記録）                         │
│  - 作業記録（session-summary）                   │
└─────────────────────────────────────────────────┘
```

### 断捨離の判断基準

| 対象 | 判断 | 理由 |
|------|------|------|
| sessions/にある重複ファイル | **削除** | docs/に同一がある |
| 📝 sessionマークの仕様抽出 | **docs/に移動** | Claudeが参照するため必要 |
| session-summary/plan | **残す** | 作業記録として必要 |
| 古い作業メモ（1回限り） | **削除検討** | 再利用価値なければ削除 |

---

## 既存の仕組み（活用すべき）

### 目次ファイル

```
docs/missions/m1-sensor-evaluation/gnss/33-toc-ublox-f9-interface-description.md
```

- PDF全体の目次を抽出済み
- 「どこに何があるか」の全体マップ

### 抽出状態テーブル

```
docs/missions/m1-sensor-evaluation/gnss/README.md
  → 「PDF仕様抽出状態」セクション
```

| ステータス | 意味 |
|-----------|------|
| ✅ 済 | docs/に正式配置済み |
| 📝 session | sessions/に抽出済み（未移動）← これが問題 |
| ❌ 未抽出 | PDFから抽出していない |

### 問題点

- 📝 sessionマークが**移動されずに蓄積**
- 抽出状態テーブルに**載っていない抽出物**がある
- 結果: Claudeが見つけられず**再抽出してしまう**

---

## 断捨離作業の詳細（次セッション以降）

### Phase 2-1: 重複ファイル削除

発見済みの重複（sessions/ と docs/ に同一）:

| sessions/ | docs/ |
|-----------|-------|
| session52/m2-obstacle-detection-report.md | docs/missions/m2-.../m2-obstacle-detection-report.md |
| session52/m2-confirmation-checklist.md | docs/missions/m2-.../m2-confirmation-checklist.md |
| session14/closed-questions-m3m4.md | docs/missions/m3-.../hearing/closed-questions-m3m4.md |
| session14/mockup-concepts.md | docs/missions/m3-.../to-be/mockup-concepts.md |
| session5/platform-comparison.md | docs/technical-research/platform-comparison.md |
| session5/maintenance-strategy.md | docs/technical-research/maintenance-strategy.md |
| session5/kintone-evaluation.md | docs/technical-research/kintone-evaluation.md |
| session5/aws-cost-estimation.md | docs/technical-research/aws-cost-estimation.md |
| session9/typescript-vs-go-report.md | docs/technical-research/typescript-vs-go-report.md |
| session9/quicksight-report.md | docs/technical-research/quicksight-report.md |

**作業**: sessions/側を削除

### Phase 2-2: 📝 sessionマーク移動

README.mdに📝 sessionとして載っているもの:

| 対象 | 現在地 | 移動先 |
|------|--------|--------|
| CFG-CFG | session211/cfg-cfg-spec.md | docs/gnss/39-cfg-cfg-spec.md |
| CFG-VALSET/VALGET | session214/cfg-valget-spec.md | docs/gnss/40-cfg-valget-spec.md |

**作業**: docs/に移動 → README索引更新 → sessions/から削除

### Phase 2-3: 抽出状態テーブルに未掲載のもの確認

sessions/にあるが抽出状態テーブルに載っていないもの:

- session165/cfg-uart1-spec.md
- session76/ubx-mon-ver-sec-uniqid-spec.md
- session64/ubx-messages-spec.md
- 他

**作業**: 価値判断 → docs/移動 or 削除

### Phase 3: ルール改善

`~/.claude/rules/14-document-management.md` に追加:

```markdown
## 7. 仕様抽出のライフサイクル（組み込み開発向け）

**背景**: Claude Code は PDF を直接読めない。抽出済み Markdown が必要。

### 仕組み

1. **目次ファイル**: `33-toc-*.md` — PDF全体のマップ
2. **抽出状態テーブル**: `README.md` — 何が抽出済みか
3. **抽出ファイル**: `docs/` — 正式配置場所

### 抽出時のフロー

1. 抽出前: README抽出状態テーブルを確認（既存あるか）
2. 抽出中: sessions/ に作成
3. 抽出後（セッション終了時）:
   - docs/ に移動（番号付与）
   - README抽出状態テーブルを ✅ 済 に更新
   - sessions/ から削除

### 禁止

- 抽出状態テーブルを確認せずに抽出開始
- 📝 session マークで放置（1セッション以上）
```

---

## 参照ファイル

次セッションで参照すべきファイル:

| ファイル | 内容 |
|----------|------|
| [docs/gnss/README.md](../../docs/missions/m1-sensor-evaluation/gnss/README.md) | 抽出状態テーブル |
| [docs/gnss/33-toc-*.md](../../docs/missions/m1-sensor-evaluation/gnss/33-toc-ublox-f9-interface-description.md) | PDF目次 |
| [~/.claude/rules/14-document-management.md](../../../.claude/rules/14-document-management.md) | 現行ルール |
| このファイル | 方針と作業詳細 |

---

## 次セッションでやること

| # | 作業 | 推定規模 |
|---|------|----------|
| 1 | 重複ファイル削除（10件） | 小 |
| 2 | 📝 sessionマーク移動（2件） | 小 |
| 3 | 未掲載抽出物の確認・判断 | 中 |
| 4 | ルールファイル改善 | 小 |

---

*作成: Session 230 (2026-03-17)*
