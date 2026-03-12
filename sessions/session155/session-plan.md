# Session 155 計画

**目的**: L1 C/N0=0問題のデバッグログ追加 + 屋外テストで確認

**前提**: Session 154でFE状態表示改善は完了

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | FEにL1 C/N0デバッグログ追加 | prototype/m1-gnss/frontend/src/app/inspections/outdoor/page.tsx |
| 2 | 屋外テストで確認 | - |
| 3 | （オプション）u-centerコンフィグ解析 | sessions/session154/test260310.txt |

---

## 詳細

### 1. L1 C/N0デバッグログ追加

page.tsx 79-91行目付近にconsole.logを追加:
- `sig.signals`の内容
- `gpsL1Signals`のフィルタ結果
- 各信号のgnss_id, sig_id, is_l1, cno

### 2. 屋外テストで確認

- GPS L1信号（gnss_id=0, is_l1=true）が取得できているか
- cno値が正しく読み取れているか
- L2受信率と整合性があるか

### 3. u-centerコンフィグ解析（オプション）

test260310.txtの内容:
- MON-VER: FW HPG 1.32, PROTVER 27.31, ZED-F9P
- CFG-VALGET: 各種設定値

---

## 参照

- [Session 154 summary](../session154/session-summary.md)
