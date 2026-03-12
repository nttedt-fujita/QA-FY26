# Session 144 計画

**目的**: FE側を統合API (`/api/gnss-state`) に移行

**前提**: Session 143でBE側の統合API実装完了

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 現状のFE API呼び出し箇所を特定 | frontend/src/ 内のfetch/axios呼び出し |
| 2 | 統合API用のhook/service作成 | gnss_state_api.rs（レスポンス型参考） |
| 3 | 各画面を統合APIに切り替え | 該当コンポーネント |
| 4 | 動作確認 | - |

---

## 詳細

### 1. 現状のFE API呼び出し

以下の個別APIを統合APIに置き換える:
- `/api/nav-sat`
- `/api/nav-sig`
- `/api/nav-status`
- `/api/mon-span`

### 2. 統合API用hook

```typescript
// 例: useGnssState.ts
const { data, error, isLoading } = useGnssState();
// data.nav_pvt, data.nav_sat, data.nav_sig, etc.
```

### 3. 切り替え

各画面で個別APIを呼んでいる箇所を、統合APIのデータを参照するように変更。

---

## 参照

- [Session 143 サマリー](../session143/session-summary.md)
- [統合API仕様書](../session142/gnss-state-api-spec.md)
- [gnss_state_api.rs](../../prototype/m1-gnss/backend/src/web/gnss_state_api.rs)
