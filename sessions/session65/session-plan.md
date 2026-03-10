# Session 65 計画

**日時**: 2026-03-XX（予定）
**前提**: Session 64でPDF抽出ツール作成、UBX仕様抽出完了

---

## 目的

1. 受け入れ検査エクセル分析資料の整理（ユーザーからの質問対応）
2. UBXパーサー実装（TDD）
3. DevContainer内でのテスト実行確認

---

## タスク

### 1. NAV-STATUSパーサー (0x01 0x03)

**取得データ**:
- gpsFix — FIXタイプ
- carrSoln — RTK状態（flags2のbit 7-6）
- ttff — Time to First Fix (ms)
- msss — 起動からの経過時間 (ms)

**参照**: [ubx-messages-spec.md](../session64/ubx-messages-spec.md)

### 2. NAV-DOPパーサー (0x01 0x04)

**取得データ**:
- pDOP, hDOP, vDOP, gDOP（スケール0.01）

### 3. MON-RFパーサー (0x0a 0x38)

**取得データ**:
- jammingState — ジャミング状態（0-3）
- noisePerMS — ノイズレベル
- agcCnt — AGCカウント

### 4. テスト実行確認

```bash
cd prototype/m1-gnss/backend
cargo test --lib
```

---

## 参照資料

- [Session 64 サマリー](../session64/session-summary.md)
- [UBX仕様書](../session64/ubx-messages-spec.md)
- [NAV-PVT設計判断](../../prototype/m1-gnss/docs/nav-pvt-design-decisions.md)
