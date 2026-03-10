# Session 81 計画

**目的**: DeviceManager実装（TDD Phase 3〜5）

---

## 背景

Session 80でTDD Phase 1-2が完了。テストシナリオリストの承認後、実装に進む。

---

## やること

### 1. テストシナリオリストの承認

[device-manager-test-scenarios.md](../session80/device-manager-test-scenarios.md) を確認し、承認を得る。

### 2. TDD Phase 3: テストコード作成

**優先順位**:
1. **A. 状態遷移テスト**（11シナリオ）— 純粋ロジック
2. **B. フィルタリングテスト**（5シナリオ）— 純粋ロジック
3. **C. DeviceManagerテスト**（16シナリオ）— モック使用

**テストスタイル**: テーブルテスト形式、`should_succeed` パラメータ使用

### 3. TDD Phase 4: 実装（Red → Green）

1つずつテストを通す最小限のコードを書く。

### 4. TDD Phase 5: リファクタリング

全テストがグリーンの状態でコード構造を改善。

---

## 完了条件

- [ ] 状態遷移テスト（A1-A11）がパス
- [ ] フィルタリングテスト（B1-B5）がパス
- [ ] DeviceManagerテスト（C1-C4）がパス
- [ ] コードがリファクタリング済み

---

## 参照資料

- [device-manager-behavior.md](../session80/device-manager-behavior.md) — 振る舞い記述
- [device-manager-test-scenarios.md](../session80/device-manager-test-scenarios.md) — テストシナリオ
- [17-gnss-tool-implementation-plan.md](../../docs/missions/m1-sensor-evaluation/gnss/17-gnss-tool-implementation-plan.md) — 実装計画

---

## 注意事項

- 古典派TDD: モックは外部依存（SerialPortProvider）の分離目的のみ
- テストコード作成前にシナリオリストの承認を得る

---

*計画作成: 2026-03-10 Session 80終了時*
