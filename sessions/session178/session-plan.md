# Session 178 計画

**目的**: FE/BEポーリング処理の図解作成

**前提**: Session 177でログ分析完了。処理の流れが不明確という課題が残った。

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | diagram-design スキルを読む | ~/.claude/skills/diagram-design/SKILL.md |
| 2 | 概念説明図作成（drawio） | - |
| 3 | シーケンス図作成（drawio） | - |
| 4 | 対策案Aを実装（ポーリング間隔6秒化） | prototype/m1-gnss/frontend/hooks/useOutdoorInspection.ts |

---

## 詳細

### 1. 概念説明図

**目的**: FE/BEのポーリング構造を説明

**含めるべき要素**:
- FE（ブラウザ）
- BE（Actix-web）
- シリアルポート（GNSS受信機）
- NTRIPサーバー
- Mutex（共有リソース）
- ポーリング間隔

### 2. シーケンス図

**目的**: リクエストのライフサイクルを可視化

**含めるべき要素**:
- FEからのGET /api/gnss-state
- BEでのMutex取得待ち
- シリアル通信（UBXメッセージ取得）
- NTRIP転送との競合
- FE終了後もBEが処理を続ける様子（パイプライン処理の生存期間）

### 3. 対策案A実装

**変更箇所**: useOutdoorInspection.ts
**変更内容**: ポーリング間隔を1秒→6秒に変更

---

## 参照

- [session177/log-analysis-results.md](../session177/log-analysis-results.md) - 分析結果
- [session177/session-summary.md](../session177/session-summary.md) - 前セッション
