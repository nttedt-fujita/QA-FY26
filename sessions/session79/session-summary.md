# Session 79 サマリー

**日付**: 2026-03-10
**目的**: CFG-RATE/CFG-PRT パーサー実装（TDD Phase 2〜5）

---

## 実施内容

### 1. TDD Phase 2: テストシナリオリスト作成

- 振る舞い記述（Session 78）を元にテストシナリオを作成
- ヌケモレ確認で以下を追加：
  - R7: timeRef範囲外 → Unknown
  - P5: 複数プロトコル同時有効
  - P9: portID≠3 → UnsupportedPortエラー

### 2. 設計判断のドキュメント化

| 項目 | 決定 |
|------|------|
| timeRef範囲外（6以上） | Unknown として扱う（エラーではない） |
| 複数プロトコル同時 | サポートする（ビットマスクなので当然） |
| portID≠3 | UnsupportedPortエラー（USBのみ対応） |

### 3. TDD Phase 3: テストコード作成

- CFG-RATE: 11テストシナリオ（3テスト関数）
- CFG-PRT: 13テストシナリオ（5テスト関数）
- テーブルテスト形式、should_succeedパラメータ使用

### 4. TDD Phase 4: 実装（Red → Green）

- CFG-RATE: パーサー実装、TimeRef enum（Unknown対応）
- CFG-PRT: パーサー実装、ProtoMask struct、UnsupportedPortエラー

### 5. TDD Phase 5: リファクタリング

- 特に追加の共通化は不要と判断
- 既存パターン（mon_ver.rs等）との一貫性を維持

---

## テスト結果

**全28テストパス**（CFG-RATE: 3関数、CFG-PRT: 5関数、既存: 20関数）

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [cfg_rate.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_rate.rs) | CFG-RATEパーサー（実装+テスト） |
| [cfg_prt.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_prt.rs) | CFG-PRTパーサー（実装+テスト） |
| [cfg-parser-design-decisions.md](cfg-parser-design-decisions.md) | 設計判断メモ |

---

## 進捗

**Phase 1 Step 1（UBXパーサー）: 7/7 完了 ✅**

| メッセージ | 状態 |
|-----------|------|
| NAV-PVT | ✅ 完了 |
| NAV-STATUS | ✅ 完了 |
| NAV-DOP | ✅ 完了 |
| MON-RF | ✅ 完了 |
| MON-VER | ✅ 完了 |
| SEC-UNIQID | ✅ 完了 |
| CFG-RATE | ✅ 完了 |
| CFG-PRT | ✅ 完了 |

---

## 次セッション（Session 80）でやること

Phase 1 Step 1が完了したので、Step 2（DeviceManager）に進む：

1. DeviceManager設計（シリアルポート管理）
2. UBXコマンド送信機能
3. 実機接続テスト準備

---

*作成: 2026-03-10 Session 79*
