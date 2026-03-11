# Session 92 計画

**目的**: ボーレート自動検出実装 + 基本UI設計

---

## 背景

Phase 1のコンポーネント統合が完了した。次は実機テストに向けた準備:
- ボーレート自動検出（ADR-007）
- 基本UIの設計・実装

---

## やること

### 1. ボーレート自動検出実装（ADR-007）

DeviceManagerに自動検出機能を追加:
- 候補: 38400 → 115200 → 9600
- 各ボーレートでNAV-STATUS pollを送信
- 応答があれば成功

参照: [ADR-007](../../docs/adr/m1/ADR-007-baud-rate-detection.md)

### 2. 基本UI設計

Phase 1の必要最小限UI:
- ロット選択/作成画面
- 装置検出・接続画面
- 検査実行・結果表示画面

---

## 完了条件

- [ ] ボーレート自動検出がDeviceManagerに実装
- [ ] 自動検出のテスト追加
- [ ] 基本UI設計（モックアップまたは実装開始）

---

## 参照資料

- [Session 91 サマリー](../session91/session-summary.md)
- [ADR-007 ボーレート自動検出](../../docs/adr/m1/ADR-007-baud-rate-detection.md)
- [統合ドメインモデル](../../docs/missions/m1-sensor-evaluation/gnss/19-gnss-unified-domain-model.md)

---

*計画作成: 2026-03-11 Session 91終了時*
