# Session 286 計画

**目的**: 設定構成シンプル化 Phase 3（対応方針の決定）

**前提**: Session 285でPhase 2（問題の特定）完了、P1問題2件特定済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | P1対応の方針決定（案A or 案B） | phase2-problems.md |
| 2 | 具体的な変更計画の作成 | - |
| 3 | 影響範囲の確認 | - |

---

## 方針選択肢

### 案A: CLAUDE.md削減

グローバルCLAUDE.mdを「インデックス＋参照リンク」に限定し、詳細はルールに委譲。

**メリット**: ルール構成を変えずに済む
**デメリット**: ルール17個は多いまま

### 案B: ルール統合

17個のルールを5-7個に統合し、CLAUDE.mdと重複するルールを削除。

**メリット**: ルール数削減、読み込みトークン削減
**デメリット**: 大規模な変更

---

## 成果物

- `sessions/session286/phase3-action-plan.md` — 変更計画（対象ファイル・変更内容）

---

## 参照

- [Session 285 summary](../session285/session-summary.md)
- [phase2-problems.md](../session285/phase2-problems.md)
- [config-simplification-plan.md](../session283/config-simplification-plan.md)

---

*作成: Session 285 (2026-03-23)*
