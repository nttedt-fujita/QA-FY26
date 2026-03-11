# Session 100 サマリー

**日付**: 2026-03-11
**目的**: 開発環境整理 + UBX通信タイミング問題のデバッグ

---

## 実施内容

### 1. Makefile作成（分割構成）

M3プロトタイプと同様の構成でMakefileを作成。

```
prototype/m1-gnss/
├── Makefile              # ルート（include + help）
└── makefiles/
    ├── backend.mk        # バックエンド関連
    ├── frontend.mk       # フロントエンド関連
    ├── dev.mk            # 全体操作
    ├── api.mk            # API呼び出し（デバッグ用）
    └── docker.mk         # Docker関連（今後）
```

### 2. Makefile構成ルール追加

`~/.claude/rules/12-makefile-structure.md` を作成。
今後のMakefile作成時はこの構成に従う。

### 3. タイミング問題の根本原因特定

実機テスト（`make dev-backend` + `make inspect LOT_ID=1`）でログを確認。

**原因**:
1. F9PがデフォルトでNMEAを定期出力
2. drain_buffer後にNMEAが届く
3. receive_ubxがNMEAをそのまま読んでしまう

**症状**:
- communication: Error（NMEAを受信、`invalid sync`）
- serial: Error（前レスポンスの途中から受信）
- fw, rate, port: Pass

### 4. 対策方針の決定

**案A + B の組み合わせ**:
- 案A: receive_ubxで`B5 62`を探す（フォールバック）
- 案B: 検査中にNMEA出力をOFF（根本対策）

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [Makefile](../../prototype/m1-gnss/Makefile) | ルートMakefile |
| [makefiles/backend.mk](../../prototype/m1-gnss/makefiles/backend.mk) | バックエンド操作 |
| [makefiles/frontend.mk](../../prototype/m1-gnss/makefiles/frontend.mk) | フロントエンド操作 |
| [makefiles/dev.mk](../../prototype/m1-gnss/makefiles/dev.mk) | 全体操作 |
| [makefiles/api.mk](../../prototype/m1-gnss/makefiles/api.mk) | API呼び出し |
| [makefiles/docker.mk](../../prototype/m1-gnss/makefiles/docker.mk) | Docker操作 |
| [~/.claude/rules/12-makefile-structure.md](~/.claude/rules/12-makefile-structure.md) | Makefile構成ルール |
| [nmea-timing-fix-plan.md](./nmea-timing-fix-plan.md) | タイミング問題対策計画 |

---

## 残課題

1. **案Aの実装**: receive_ubxでB5 62同期
2. **案Bの実装**: 検査中のNMEA OFF/ON
3. **Docker環境構築**: USB対応のdocker-compose.yml

---

## 次セッション（Session 101）でやること

1. 案A（receive_ubx B5 62同期）の実装
2. 実機テストで効果確認
3. 案B（NMEA OFF/ON）の実装

---

*Session 100 完了*
