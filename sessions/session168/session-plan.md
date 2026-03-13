# Session 168 計画

**目的**: 115200bpsでの送信バッファ排出時間計測 + 恒久対策決定

**前提**: Session 167で38400bpsの計測完了（1255bytes排出に約245ms必要）

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | DEFAULT_BAUD_RATEを115200に変更 | manager.rs |
| 2 | 100msで計測（成功するか確認） | - |
| 3 | 必要に応じて50msでも計測 | - |
| 4 | 恒久対策を決定・実装 | - |

---

## 詳細

### 1. 115200bpsでの計測

**理論値**:
- 115200bps = 11520 bytes/sec
- 1255bytes排出: 約109ms
- 100msでほぼ排出できる見込み

**手順**:
1. manager.rs: `DEFAULT_BAUD_RATE = 115200`
2. gnss_state_api.rs: `stabilize_delay_ms = 100`（現状のまま）
3. `make rtk-debug`で計測

### 2. 恒久対策の選択肢

| 選択肢 | 概要 |
|--------|------|
| A | 115200bps固定 + 100ms待機 |
| B | 115200bps固定 + 送信バッファ空待ち |
| C | ボーレート依存で待機時間を設定 |

115200bpsで100msで成功すれば、**A案**が最もシンプル。

---

## 参照

- [Session 167 summary](../session167/session-summary.md)
- [timing-measurement-38400bps.md](../session167/timing-measurement-38400bps.md)

---

*作成: Session 167 (2026-03-13)*
