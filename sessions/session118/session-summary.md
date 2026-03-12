# Session 118 サマリー

**日付**: 2026-03-12
**目的**: MON-SPAN API TDDレビュー + 仕様書参照ルール強制化

---

## 実施内容

### 1. TDDレビュー（MON-SPAN）

Session 117で「仕様書を読まずに実装した」問題に対して、TDDスキルでヌケモレチェックを実施。

**レビュー結果**:
- 仕様書（ubx-mon-messages.md）とテストを照合
- パーサーテスト: 8項目すべて仕様をカバー
- APIテスト: 6項目すべて適切

**結論**: テストは仕様をカバーしている。追加テスト不要。

### 2. 仕様書参照ルールの強制化

Session 117の問題を再発防止するため、ルールを整備:

1. **グローバルルール作成**: `~/.claude/rules/13-spec-first-implementation.md`
   - プロトコル実装時は仕様書を読むことを強制
   - 読んだファイルを明記すること

2. **M1-GNSS固有ルール作成**: `prototype/m1-gnss/CLAUDE.md`
   - 対象ごとの仕様書パス一覧
   - 確認プロセスの明記

### 3. ドキュメント正式配置

Session 116/117のドラフトを統合し、正式配置:
- `docs/missions/m1-sensor-evaluation/gnss/23-mon-span-implementation.md`
  - 振る舞い記述
  - テストリスト
  - TDDレビュー結果

ドラフト削除:
- `sessions/session116/mon-span-parser-spec.md`
- `sessions/session117/mon-span-api-design.md`

---

## 作成・更新ファイル

| ファイル | 内容 |
|----------|------|
| `~/.claude/rules/13-spec-first-implementation.md` | 仕様書参照ルール（グローバル） |
| `prototype/m1-gnss/CLAUDE.md` | M1-GNSS固有ルール（新規作成） |
| `docs/missions/m1-sensor-evaluation/gnss/23-mon-span-implementation.md` | MON-SPAN実装仕様（正式版） |

---

## 決定事項

1. **仕様書参照ルール**をグローバルルールとして追加
2. プロトタイプごとにCLAUDE.mdで具体的な仕様書パスを記載
3. MON-SPANテストは追加不要

---

## 残タスク

- MON-SPAN FE実装（次セッション以降）
- 屋内/屋外検査ページ分離（ユーザー要望）

---

## 次セッションでやること

1. MON-SPAN FE実装
   - api.ts に getMonSpan() 追加
   - MonSpanPanel.tsx 新規作成（スペクトラム波形、PGAゲージ）
2. または屋内/屋外検査ページ分離

---

*作成: 2026-03-12 Session 118終了時*
