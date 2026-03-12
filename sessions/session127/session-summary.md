# Session 127 サマリー

**目的**: M1-GNSS ドキュメント整理の実行

**前提**: Session 126で作成した計画書に基づく

---

## 実施内容

### 1. docs/gnss/ の番号振り直し（4ファイル）

| 変更前 | 変更後 |
|--------|--------|
| 23-outdoor-inspection-domain-model.md | **26**-outdoor-inspection-domain-model.md |
| 24-outdoor-inspection-implementation-plan.md | **27**-outdoor-inspection-implementation-plan.md |
| 25-tdd-review-result.md | **28**-tdd-review-result.md |
| architecture-nextjs-rust.md | **29**-architecture-nextjs-rust.md |

### 2. sessions/ の置き去りドキュメント整理（7件）

| ファイル | 行き先 |
|----------|--------|
| session107/ubx-spec-extract.md | docs/ → 24-ubx-spec-extract.md |
| session107/nav-sig-behavior-spec.md | docs/ → 25-nav-sig-behavior-spec.md |
| session111/ttff-monrf-spec.md | docs/ → 30-ttff-monrf-spec.md |
| session112/nav-sig-api-design.md | prototype/m1-gnss/docs/ |
| session111/outdoor-inspection-priority.md | **削除** |
| session111/m1-gnss-all-tasks.md | **削除** |
| session111/m1-gnss-milestone.md | **削除** |

### 3. README.md の更新

- 「○○するときは△△を見る」チェックリスト追加
- Phase情報を「Phase 3: ツール実装」に更新
- ファイル一覧を正しい番号順に修正

### 4. prototype/m1-gnss/CLAUDE.md の更新

- 仕様書参照ルールに新しいドキュメント追加
- ADR-009追加
- プロトタイプ固有ドキュメントセクション追加

---

## 追加で発見した問題

### 導線の問題

gnss/README.mdへの導線が3段階のホップを必要とする：
```
CLAUDE.md → docs/index.md → m1-sensor-evaluation/README.md → gnss/README.md
```

**対応**: Session 128で修正予定

### 計画テンプレートの改善

セッション計画に「読むべきファイル」を明記することで、トークン節約が可能。

**方針決定**:
- 新しいルールを追加するのではなく、session-managementスキルの計画テンプレートを更新
- 「読むべきファイル」カラムを必須化
- Session 130で実施

---

## 成果物

| ファイル | 内容 |
|----------|------|
| [gnss/README.md](../../docs/missions/m1-sensor-evaluation/gnss/README.md) | チェックリスト付きREADME |
| [prototype/m1-gnss/CLAUDE.md](../../prototype/m1-gnss/CLAUDE.md) | 仕様書参照ルール更新 |
| [prototype/m1-gnss/docs/nav-sig-api-design.md](../../prototype/m1-gnss/docs/nav-sig-api-design.md) | API設計（移動） |
| [documentation-improvement-plan.md](../../docs/documentation-improvement-plan.md) | Session 130計画を更新（正式配置済み） |

---

## 次セッション（Session 128）でやること

1. 導線の修正（gnss/README.mdへの直接リンク追加）
2. M1整理結果の検証（探しやすくなったか）
3. M3/M4に同様の整理が必要か判断

---

*作成: 2026-03-12 Session 127*
