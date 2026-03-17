# Session 221 計画

**目的**: reset-configテスト再実行（Flash対応版）

**前提**: Session 220でset-periodic-output APIをFlash対応に修正済み

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | Phase 1: 定期出力設定 → USB抜き差し3回で維持確認 | - | make connect, make set-periodic-output, make message-scan |
| 2 | Phase 2: reset-config → USB抜き差し3回で消失確認 | - | make reset-config, make message-scan |

---

## 詳細

### Phase 1: 定期出力維持確認

1. バックエンド起動 (`make dev-backend`)
2. 装置接続 (`make connect`)
3. 定期出力有効化 (`make set-periodic-output`) → ACK確認
4. message-scan → NAV-PVT等検出を確認
5. USB抜き差し（電源断）
6. 再接続 → message-scan → **定期出力が維持されていることを確認**
7. 3回繰り返す

### Phase 2: reset-config消失確認

1. Phase 1で定期出力が維持されていることを確認
2. reset-config実行 (`make reset-config`) → ACK確認
3. message-scan → 0件を確認
4. USB抜き差し（電源断）
5. 再接続 → message-scan → **定期出力が消えていることを確認**
6. 3回繰り返す

---

## 期待する結果

| Phase | 条件 | 期待結果 |
|-------|------|----------|
| 1 | set-periodic-output後 | NAV-PVT等検出 |
| 1 | USB抜き差し後（3回） | NAV-PVT等が**維持**される |
| 2 | reset-config後 | 0件 |
| 2 | USB抜き差し後（3回） | 0件が**維持**される |

---

## 参照

- [Session 220 summary](../session220/session-summary.md)
- [config-layers-spec.md](../session220/config-layers-spec.md) - BBR/Flash仕様
