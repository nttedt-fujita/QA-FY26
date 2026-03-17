# Session 230 計画

**目的**: 設定確認方法の整理 + Phase 3テスト再開

**前提**: Session 229でBBR優先順位問題の解決確認完了

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | 未整理の疑問を整理 | session229/session-summary.md | - |
| 2 | 全設定一覧取得の可能性調査 | IF p.95-96 (CFG-VALGET) | - |
| 3 | 古い機での動作確認 | - | make connect, make message-scan |
| 4 | Phase 3テスト再開 | session206/multi-device-inspection-plan.md | - |

---

## 詳細

### 1. 未整理の疑問を整理

Session 229で出た疑問：
- Flashだけ見れば間違いないか？
- 全設定を一覧表示できないか？
- 「切っても切っても出てくる」問題の根本原因
- スキャン時間と設定確認の関係

### 2. 全設定一覧取得の可能性調査

CFG-VALGETで複数キー一括取得可能か仕様書確認：
- IF p.95-96 (CFG-VALGET)
- Wildcard指定の可否

### 3-4. Phase 3テスト再開

古い機（RTK基準局設定あり）での動作確認：
- reset-config または cfg-valdel でクリア
- 複数台同時検査の再テスト

---

## 参照

- [session229/session-summary.md](../session229/session-summary.md) - 未整理の疑問
- [session229/investigation-summary.md](../session229/investigation-summary.md) - Session 199-229調査サマリー
- [layer-config-cheatsheet.md](../session229/layer-config-cheatsheet.md) - コマンド一覧

---

*作成: Session 229 (2026-03-17)*
