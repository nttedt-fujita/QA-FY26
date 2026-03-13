# Session 165 計画

**目的**: CFG-UART1-BAUDRATE仕様確認 → ボーレート変更テスト

**前提**: Session 164で500ms安定化待機で成功を確認済み

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | PDFからCFG-UART1-BAUDRATE仕様を抽出 | Interface Description PDF |
| 2 | 仕様に基づき実装を検証・修正 | cfg_valset.rs |
| 3 | 115200bpsに変更してテスト | - |
| 4 | 効果比較（500ms待機 vs 高ボーレート） | - |

---

## 詳細

### 1. PDF抽出手順

```bash
# ページ数確認
ls -lh sessions/session155/u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf

# 目次抽出（CFG-UART1-BAUDRATEの場所特定）
python tools/pdf_page_extractor.py \
  sessions/session155/u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf \
  1-5 --output sessions/session165/pdf-toc.md

# 該当ページ抽出
python tools/pdf_page_extractor.py \
  sessions/session155/u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf \
  XXX-YYY --output sessions/session165/cfg-uart1-baudrate-spec.md
```

### 2. 確認すべき仕様

- CFG-UART1-BAUDRATE (0x40520001) のキーID
- 値の型（U4 = 4バイト）
- 設定後の動作（即座に反映？再起動必要？）
- 有効なボーレート値の範囲

### 3. テスト手順

1. 現在の状態を確認（38400bps）
2. 115200に変更するコマンドを送信
3. 再接続（自動検出で115200を検出するはず）
4. `make rtk-debug` でテスト
5. 500ms待機との比較

---

## 参照

- [Session 164 summary](../session164/session-summary.md)
- [cfg_valset.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_valset.rs)
- [rules/13-spec-first-implementation.md](../../.claude/rules/13-spec-first-implementation.md)
- [rules/15-pdf-handling.md](../../.claude/rules/15-pdf-handling.md)

---

*作成: Session 164 (2026-03-13)*
