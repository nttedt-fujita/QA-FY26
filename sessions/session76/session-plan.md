# Session 76 計画

**目的**: UBXパーサー追加実装（MON-VER, SEC-UNIQID）— TDD

---

## 背景

[17-gnss-tool-implementation-plan.md](../../docs/missions/m1-sensor-evaluation/gnss/17-gnss-tool-implementation-plan.md) に基づき、Phase 1のStep 1を実施する。

Session 72で3メッセージ実装済み:
- NAV-STATUS
- NAV-DOP
- MON-RF

今回は2メッセージを追加実装。

---

## やること

### 1. MON-VER パーサー実装（TDD）

**用途**: FWバージョン取得

| 項目 | 内容 |
|------|------|
| Class/ID | 0x0A 0x04 |
| 取得情報 | swVersion, hwVersion, extension strings |

**TDDサイクル**:
1. Phase 0: 振る舞い記述
2. Phase 1: テスト作成（失敗）
3. Phase 2: 実装（成功）
4. Phase 3: リファクタリング

### 2. SEC-UNIQID パーサー実装（TDD）

**用途**: チップシリアル番号取得

| 項目 | 内容 |
|------|------|
| Class/ID | 0x27 0x03 |
| 取得情報 | 9バイトのユニークID |

---

## 参照資料

- [17-gnss-tool-implementation-plan.md](../../docs/missions/m1-sensor-evaluation/gnss/17-gnss-tool-implementation-plan.md) — 実装計画
- [08-ubx-protocol-index.md](../../docs/missions/m1-sensor-evaluation/gnss/08-ubx-protocol-index.md) — UBXプロトコル索引
- u-blox F9P Interface Description（PDF）— 仕様書

---

## 注意

- TDD作業前に `tdd-practice` スキルを読む
- テストは `06-test-style.md` のテーブルテスト形式で作成
- 仕様書フィールドのヌケモレチェックを実施（Session 72の反省）

---

*計画作成: 2026-03-10 Session 75終了時*
