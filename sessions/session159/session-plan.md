# Session 159 計画

**目的**: NTRIP + UBXポーリングの実機テスト、ログから問題特定

**前提**: Session 158でデバッグログ追加・Makefileターゲット整備完了

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | ntrip.conf作成 | ntrip.conf.example |
| 2 | make rtk-start でテスト開始 | - |
| 3 | make rtk-poll でポーリング | - |
| 4 | ログ確認・問題特定 | /tmp/m1-gnss-rtk-debug.log |
| 5 | 問題があれば修正 | - |

---

## テスト手順

```bash
cd prototype/m1-gnss

# 1. 設定ファイル作成
cp ntrip.conf.example ntrip.conf
vim ntrip.conf

# 2. ターミナル1: ログ監視
make rtk-log

# 3. ターミナル2: テスト実行
make rtk-start
make rtk-poll
make rtk-stop
```

---

## 確認ポイント

ログで以下を確認:
- `[NTRIP-RTCM]` と `[GNSS-STATE]` のロック競合はあるか
- ロック取得待ち時間は異常に長くないか
- どのメッセージでエラーが出ているか

---

## 参照

- [Session 158 summary](../session158/session-summary.md)
- [Session 157 調査結果](../session156/ntrip-rtk-spec-findings.md)
