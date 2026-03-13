# Session 167 計画

**目的**: 115200bpsでの待機時間検証 + 恒久対策の決定

**前提**: Session 166で38400bpsの検証完了（200ms必要、100msでは失敗）

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 115200bpsで待機時間検証 | session166/stabilize-delay-analysis.md |
| 2 | 恒久対策の検討・決定 | - |
| 3 | 実装（必要に応じて） | gnss_state_api.rs |

---

## 詳細

### 1. 115200bpsでの検証

**目的**: ボーレートを上げれば待機時間を短くできるか確認

**理論値**:
- 38400bps: 262bytes送信 ≈ 68ms → 実測200ms必要
- 115200bps: 262bytes送信 ≈ 23ms → 100ms以下で済む可能性

**テスト手順**:
1. DEFAULT_BAUD_RATE = 115200（現状のまま）
2. stabilize_delay_ms を変えてテスト
   - 100ms → 成功するか？
   - 50ms → 成功するか？

### 2. 恒久対策の選択肢

| 選択肢 | 概要 | メリット | デメリット |
|--------|------|---------|-----------|
| A | ボーレート依存で待機時間設定 | シンプル | ボーレートごとに値を決める必要 |
| B | 送信バッファ空待ち | 理想的 | 実装が複雑 |
| C | 固定500ms維持 | 変更なし | 無駄な待機時間 |

### 3. 決定基準

- 115200bpsで100ms以下で成功 → A案（ボーレート依存）を採用
- 115200bpsでも200ms必要 → B案（バッファ空待ち）を検討
- 時間がなければ → C案（現状維持）

---

## 参照

- [Session 166 summary](../session166/session-summary.md)
- [stabilize-delay-analysis.md](../session166/stabilize-delay-analysis.md)
- [gnss_state_api.rs](../../prototype/m1-gnss/backend/src/web/gnss_state_api.rs)

---

*作成: Session 166 (2026-03-13)*
