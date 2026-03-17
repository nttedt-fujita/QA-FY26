# Session 229 計画

**目的**: USB抜き差し後のFlash値確認 + ドキュメント更新

**前提**: Session 228でBBR削除成功（cfg-valdel-bbr → ACK）

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | USB抜き差し + BE再起動 | - | make dev-backend |
| 2 | Flashの値が使われるか確認 | - | make connect-raw, make message-scan |
| 3 | 成功したらドキュメント更新 | 38-ublox-config-management.md | - |

---

## 詳細

### 1-2. USB抜き差し後のテスト

**手順**:
1. USB抜き差し
2. `make dev-backend` でBE再起動
3. `make connect-raw` で接続（定期出力無効化をスキップ）
4. `make message-scan` で定期出力確認

**期待結果**: NAV-PVT等が検出される（Flashの値1が使われている）

### 3. ドキュメント更新

成功したら以下に手順を追記:
- `docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md`
- BBR優先順位問題の解決手順セクション

---

## 参照

- [session228/session-summary.md](../session228/session-summary.md) - テスト結果
- [38-ublox-config-management.md](../../docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md) - 設定管理仕様

---

*作成: Session 228 (2026-03-17)*
