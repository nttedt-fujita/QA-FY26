# Session 158 計画

**目的**: バックエンドAPIだけでNTRIP + UBXポーリングをテスト、問題切り分け

**前提**: Session 157で調査完了。理論上は並行動作可能だが、実際にはエラー発生。バックエンドだけでテストして問題を特定する。

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | APIエンドポイント確認 | prototype/m1-gnss/backend/src/web/ |
| 2 | Makefileターゲット作成 | prototype/m1-gnss/makefiles/api.mk |
| 3 | 実機テスト | - |
| 4 | 問題切り分け | - |

---

## 詳細

### 1. APIエンドポイント確認

必要なAPI:
- `POST /api/ntrip/connect` - NTRIP接続
- `GET /api/ntrip/status` - NTRIP状態確認
- `GET /api/device/gnss-state` - UBXポーリング（NAV-PVT等）

### 2. Makefileターゲット作成

```makefile
# api.mk に追加
ntrip-connect:
	curl -X POST http://localhost:8080/api/ntrip/connect \
	  -H "Content-Type: application/json" \
	  -d '{"caster_url":"...", "port":2101, ...}'

ntrip-status:
	curl http://localhost:8080/api/ntrip/status

gnss-state:
	curl http://localhost:8080/api/device/gnss-state
```

### 3. 実機テスト

手順:
1. `make backend-up`
2. `make device-connect` (デバイス接続)
3. `make ntrip-connect` (NTRIP接続)
4. `make gnss-state` (UBXポーリング)
5. エラーが出るか確認

### 4. 問題切り分け

- **再現する** → バックエンドの問題を修正
- **再現しない** → FE連携の問題を調査

---

## 参照

- [Session 157 summary](../session157/session-summary.md)
- [ntrip-rtk-spec-findings.md](../session156/ntrip-rtk-spec-findings.md)
