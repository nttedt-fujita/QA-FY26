# Session 181 計画

**目的**: UI改善要望の優先順位整理→調査→計画策定

**前提**: Session 180で履歴再生画面を実装完了

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 要望の優先順位整理 | - |
| 2 | 各要望の技術調査 | 下記参照 |
| 3 | 実装計画策定 | - |

---

## 要望一覧（優先順位未定）

| # | 要望 | 調査事項 | 読むべきファイル |
|---|------|----------|-----------------|
| 1 | シリアル番号表示 | DBスキーマ確認、FE表示箇所 | schema.sql, history/page.tsx |
| 2 | RF波形比較機能 | 比較UI設計、複数データ取得方法 | MonSpanPanel.tsx |
| 3 | RF波形サンプルごと確認 | 本当に取れてるか調査 | スナップショットJSON確認 |
| 4 | L1 CNO MIN/MAX/平均 | MINだけにした根拠再調査 | ADR-008, outdoor-inspection-design.md |
| 5 | GNSS指定受信 | UBX設定調査、CFG-GNSS | u-blox仕様書 |
| 6 | RF波形表示サイズ | CSS調整のみ | MonSpanPanel.tsx |
| 7 | 履歴からGNSS切り分け | NAV-SATデータ構造確認 | スナップショットJSON確認 |

---

## 進め方

1. **優先順位整理**: ユーザーと相談して優先度を決定
2. **調査フェーズ**: 各要望の技術的な実現可能性を調査
3. **計画策定**: 実装順序・工数を見積もり
4. **OKが出たら実装開始**

---

## 参照

- [Session 180 サマリー](../session180/session-summary.md)
- [ADR-008: 屋外検査の合格基準](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md)
- [履歴再生画面](../../prototype/m1-gnss/frontend/src/app/inspections/history/page.tsx)
