# Session 84 計画

**目的**: InspectionEngine実装完了（TDD Phase 3-5）

---

## 背景

Session 83でinspectionモジュールを作成し、D1-D3（構造体）とC1-C5（判定ロジック）のテストを作成。
残り16シナリオのテスト作成と実装が必要。

---

## やること

### 1. ビルド可能にする

```bash
# lib.rsにinspectionモジュール追加
# engine.rs（空でも可）を作成
cargo build
```

### 2. TDD Phase 3続き: テストコード作成

| 優先度 | シナリオ | 内容 |
|--------|---------|------|
| 3 | A1-A4 | 検査シーケンス実行 |
| 4 | G1-G5 | 各検査項目のUBX送信 |
| 5 | B1-B2 | 通信疎通 |
| 6 | E1-E2 | 状態連携 |
| 7 | F1-F3 | 異常系 |

### 3. TDD Phase 4: 実装（Red → Green）

テストを1つずつ通していく。

### 4. TDD Phase 5: リファクタリング

全テストがグリーンの状態で構造を改善。

---

## 完了条件

- [ ] inspectionモジュールがビルド可能
- [ ] 24テストシナリオのテストコードが作成されている
- [ ] 全テストがパスしている
- [ ] リファクタリングが完了している

---

## 参照資料

- [inspection-engine-behavior.md](../session82/inspection-engine-behavior.md) — 振る舞い記述＋テストシナリオ
- [session83/session-summary.md](../session83/session-summary.md) — 前セッションサマリー

---

*計画作成: 2026-03-10 Session 83終了時*
