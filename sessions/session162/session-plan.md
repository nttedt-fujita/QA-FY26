# Session 162 計画

**目的**: NTRIP + UBXポーリング競合問題の仮説検証（50ms安定化待機の効果確認）

**前提**: Session 161でgnss-state側に50ms安定化待機を追加済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 実機テスト実行 | - |
| 2 | ログ分析・仮説検証 | gnss_state_api.rs |
| 3 | 効果に応じた次アクション | - |

---

## 詳細

### 1. 実機テスト

```bash
cd prototype/m1-gnss

# ターミナル1: ログ監視
make rtk-log

# ターミナル2: テスト実行
make rtk-start
make rtk-poll  # 3-5回繰り返す
make rtk-stop
```

### 2. ログ確認ポイント

- `[GNSS-STATE] シリアルポート安定化待機: 50ms` が出ているか
- `[GNSS-STATE] 安定化待機完了` の後、NMEA OFF送信が成功しているか
- 全6メッセージが成功しているか（エラー数: 0）

### 3. 判断基準

| 結果 | 次アクション |
|------|-------------|
| 成功率向上 | 対策確定、待機時間の微調整を検討 |
| 変化なし | 別原因を探る（シリアルポートタイムアウト延長等） |

---

## 参照

- [Session 161 summary](../session161/session-summary.md)
- [Session 159 テストレポート](../session159/ntrip-polling-test-report.md)

---

*計画作成: 2026-03-13 Session 161終了時*
