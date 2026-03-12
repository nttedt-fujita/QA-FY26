# Session 142 計画

**目的**: 統合API実装 + FEポーリング集約

**前提**: Session 141でcfg_valset.rsテスト修正完了、06-test-style.md更新済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 統合API (`GET /api/gnss-state`) 実装 | nav_status_api.rs, nav_sat_api.rs, ADR-012 |
| 2 | FE側のポーリング集約 | outdoor/page.tsx |
| 3 | TDDスキルに従って実装 | ~/.claude/skills/tdd-practice/SKILL.md |

---

## 詳細

### 1. 統合API実装（TDDで進める）

**Phase 0**: 文脈確認（藤田さんと）
**Phase 1**: 振る舞い定義
**Phase 2**: テストシナリオリスト作成
**Phase 3**: テストコード作成（rstest形式必須）
**Phase 4**: 実装

新規エンドポイント `GET /api/gnss-state`:
```json
{
  "nav_status": { ... },
  "nav_sat": { ... },
  "nav_sig": { ... },
  "mon_span": { ... }
}
```

- 内部で順次ポーリング（競合回避）
- 1回のAPIで全データ取得

### 2. FE側のポーリング集約

現状:
- 複数のsetIntervalで独立ポーリング（nav_status, nav_sat等）

新:
- `gnss_state_api` を1秒ごとにポーリング
- 1回で全データ取得

---

## 参照

- [Session 141 summary](../session141/session-summary.md)
- [ADR-012](../../docs/adr/m1/ADR-012-periodic-output-and-unified-api.md)
- [TDDスキル](~/.claude/skills/tdd-practice/SKILL.md)

---

*計画作成: 2026-03-12 Session 141終了時*
