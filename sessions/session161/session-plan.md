# Session 161 計画

**目的**: NTRIP + UBXポーリング競合問題の実機テストと対策確定

**前提**: Session 160でデバッグログ追加・50msフラッシュ待機を追加済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 実機テスト（デバッグログ確認） | - |
| 2 | 50msフラッシュ待機の効果検証 | ntrip_api.rs, gnss_state_api.rs |
| 3 | 効果に応じた対策確定 | - |

---

## 詳細

### 1. 実機テスト

```bash
cd prototype/m1-gnss

# ターミナル1: ログ監視
make rtk-log

# ターミナル2: テスト実行
make rtk-start
make rtk-poll
make rtk-stop
```

### 2. ログ確認ポイント

- ロック待機時間（100ms超で警告が出るか）
- 各メッセージの送信/受信時間
- 失敗時のエラー内容

### 3. 対策候補

| 対策 | 詳細 |
|------|------|
| フラッシュ待機延長 | 50ms → 100ms等 |
| gnss-state側で待機 | ロック取得直後に待機 |
| リトライ方式 | 送信失敗時に再試行 |

---

## 参照

- [Session 160 summary](../session160/session-summary.md)
- [Session 159 テストレポート](../session159/ntrip-polling-test-report.md)

---

*計画作成: 2026-03-13 Session 160終了時*
