# Session 117 計画

**目的**: MON-SPAN API/FE連携、または屋内/屋外ページ分離

---

## 背景

Session 116でMON-SPANパーサーを実装完了。
パーサーを活用するためにはAPI/FE連携が必要。

---

## やること候補

### 案A: MON-SPAN API実装

- `GET /api/mon-span` エンドポイント追加
- NAV-SIG APIと同様のパターン

### 案B: MON-SPAN FE実装

- スペクトラム波形表示（256点のグラフ）
- PGAゲージ（38dB異常 vs 54dB正常）

### 案C: 屋内/屋外検査ページ分離

- Session 113で出たユーザー要望
- 現在の `/inspections` を分割

---

## 参照資料

- [mon-span-parser-spec.md](../session116/mon-span-parser-spec.md) — パーサー仕様
- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) — 屋外検査合格基準
- [outdoor-inspection-priority.md](../session111/outdoor-inspection-priority.md) — Phase 1.5優先度

---

*計画作成: 2026-03-12 Session 116終了時*
