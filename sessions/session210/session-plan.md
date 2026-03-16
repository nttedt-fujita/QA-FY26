# Session 210 計画

**目的**: 古い機の定期出力無効化を完了 + CLAUDE.md整備

**前提**: Session 209で5メッセージ追加、10件→7件に改善

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 残り3メッセージのKey ID確認・追加 | - |
| 2 | 実機確認（定期出力が止まるまで） | - |
| 3 | CLAUDE.mdに仕様書場所を記録 | `CLAUDE.md` |
| 4 | （時間あれば）屋内検査を再実行してFWバージョン取得確認 | - |

---

## 追加が必要なメッセージ

| メッセージ | Class/ID | USB Key ID | UART1 Key ID |
|-----------|----------|------------|--------------|
| NAV-HPPOSLLH | 0x01-0x14 | 要確認 | 要確認 |
| NAV-VELNED | 0x01-0x12 | 要確認 | 要確認 |
| UNKNOWN | 0x01-0x34 | 要特定 | 要確認 |

---

## Key ID確認手順

```bash
# PDFから抽出
python tools/pdf_page_extractor.py "sessions/session200/P233-P273_u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf" 1-40 /tmp/cfg-msgout.md

# 検索
grep -i "HPPOSLLH" /tmp/cfg-msgout.md
grep -i "VELNED" /tmp/cfg-msgout.md
```

---

## 参照

- [Session 209 summary](../session209/session-summary.md)
- [cfg_valset.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_valset.rs)
