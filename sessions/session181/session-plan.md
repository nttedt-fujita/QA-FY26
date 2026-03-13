# Session 181 計画

**目的**: UI改善要望の優先順位整理→実装

**前提**: Session 180で履歴再生画面を実装完了

---

## 実装順序

### Phase 1: 確実にできる（すぐ実装）

| # | 要望 | 作業内容 | 対象ファイル |
|---|------|----------|-------------|
| 6 | RF波形表示サイズ拡大 | CSS調整（h-16→h-32等） | MonSpanPanel.tsx |
| 1 | シリアル番号表示 | FE表示追加 | 履歴画面、検査画面 |

### Phase 2: 軽い確認→実装

| # | 要望 | 確認事項 | 対象ファイル |
|---|------|----------|-------------|
| 3 | RF波形サンプルごと確認 | スナップショットにMON-SPAN含まれているか | スナップショットJSON |
| 7 | 履歴からGNSS切り分け | NAV-SATデータ構造確認 | スナップショットJSON |

### Phase 3: 調査が必要（後回し）

| # | 要望 | 調査内容 |
|---|------|----------|
| 2 | RF波形比較機能 | UI設計、複数データ取得方法 |
| 4 | L1 CNO MIN/MAX/平均 | ADR-008の根拠再確認、仕様変更可能性 |
| 5 | GNSS指定受信 | UBX CFG-GNSS設定調査 |

---

## 進捗

- [x] 優先順位整理
- [x] Phase 1: #6 RF波形表示サイズ
- [x] Phase 1: #1 シリアル番号表示
- [x] Phase 2: #3, #7 確認→実装
- [ ] Phase 3: 調査 → 次セッションへ

---

## 参照

- [Session 180 サマリー](../session180/session-summary.md)
- [MonSpanPanel.tsx](../../prototype/m1-gnss/frontend/src/components/MonSpanPanel.tsx)
- [schema.sql](../../prototype/m1-gnss/db/schema.sql)
