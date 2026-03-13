# Session 177 計画

**目的**: ログ分析によるFE/BE時間ずれ問題の傾向把握

**前提**: Session 176でログ分析方針を策定済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 全10件の検査ログを分割・抽出 | session176/log-analysis-plan.md |
| 2 | 各検査の時間データを表にまとめる | - |
| 3 | 傾向を分析 | - |
| 4 | 根本原因の特定と対策案検討 | - |

---

## 詳細

### 1. ログ分割

```bash
# ログディレクトリ作成
mkdir -p sessions/session177/logs

# 各検査のログを抽出（例: 2回目）
sed -n '1200,1400p' prototype/m1-gnss/backend/logs/gnss-backend.log.2026-03-13 > sessions/session177/logs/inspection-02.log
```

### 2. 各検査で計測すること

- FE終了時刻（POST /api/outdoor-inspection-results）
- BE終了時刻（最後のGNSS-STATE完了）
- ずれ（秒）
- API呼び出し回数
- 1回あたり処理時間

### 3. 傾向分析

- ずれは一定か、増加傾向か
- 処理時間のばらつき
- 競合の発生頻度

---

## 参照

- [session176/log-analysis-plan.md](../session176/log-analysis-plan.md) - 分析方針
- [session176/session-summary.md](../session176/session-summary.md) - 前セッション
