# Session 160 計画

**目的**: NTRIP + UBXポーリング競合問題の解析と対策

**前提**: Session 159でテスト実施、競合問題を確認済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 競合メカニズムの詳細解析 | session159/ntrip-polling-test-report.md |
| 2 | 対策案の検討 | ntrip_api.rs, gnss_state_api.rs, manager.rs |
| 3 | 対策の実装 | - |

---

## 詳細

### 1. 競合メカニズムの詳細解析

Session 159のテストレポートを元に、以下を明確化:
- なぜ20msでタイムアウトするのか（通常は2000ms）
- シリアルポートのwrite_timeoutの設定
- RTCM転送後の状態

### 2. 対策案の検討

**候補**:
1. **NTRIP一時停止方式**: gnss-state実行中はRTCM転送を停止
2. **リトライ方式**: タイムアウト時に再試行
3. **排他制御の見直し**: より細粒度のロック

### 3. 対策の実装

選択した対策をTDDで実装

---

## 参照

- [Session 159 summary](../session159/session-summary.md)
- [ntrip-polling-test-report.md](../session159/ntrip-polling-test-report.md)

---

*計画作成: 2026-03-13 Session 159終了時*
