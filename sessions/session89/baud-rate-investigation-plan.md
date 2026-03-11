# ボーレート自動判定の調査計画

**作成日**: 2026-03-11
**Session**: 89
**目的**: F9P実機のボーレート自動判定機能の有無を調査

---

## 背景

Session 83でF9P実機テストを実施した際、FTDI経由UART接続で以下が判明：

- VID=0x0403, PID=0x6015（FTDI FT232H）
- ボーレート38400bpsでNMEA受信成功
- ボードごとにボーレート設定が異なる可能性

**課題**: ボーレートを自動判定できるか、u-bloxの仕様書を確認する必要がある。

---

## 調査対象ドキュメント

### 1. Integration Manual（統合マニュアル）

**想定内容**: ハードウェア統合、UART設定、電源管理など
**調査項目**:
- [ ] Auto-baud detection（自動ボーレート検出）機能の有無
- [ ] デフォルトボーレート設定
- [ ] UART設定の推奨事項

### 2. Interface Description（インターフェース仕様書）

**既に調査済み**: Session 78でCFG-PRT, CFG-RATEを抽出
**追加調査項目**:
- [ ] UBX-CFG-PRT のボーレート設定オプション
- [ ] Autobauding関連の記載

---

## 調査手順

### Phase 1: ドキュメント取得・目次抽出

1. u-bloxのドキュメントページからPDFを取得
   - F9P Integration Manual
   - F9P Interface Description（既存）
2. Pythonスクリプトで目次を抽出
3. ボーレート関連のセクションを特定

### Phase 2: 該当セクションの抽出

1. 目次から特定したページを抽出
2. Markdown形式で保存
3. 内容を分析

### Phase 3: 設計への反映

1. 自動判定が可能な場合 → DeviceManagerに実装
2. 自動判定が不可能な場合 → 手動設定のままで運用

---

## Python抽出スクリプト（既存パターン）

過去のセッション（Session 64, 76, 78）で使用したパターン：

```python
import fitz  # PyMuPDF

def extract_toc(pdf_path):
    """PDFの目次を抽出"""
    doc = fitz.open(pdf_path)
    toc = doc.get_toc()
    for item in toc:
        level, title, page = item
        print(f"{'  ' * (level-1)}{title} (p.{page})")

def extract_pages(pdf_path, pages):
    """指定ページを抽出"""
    doc = fitz.open(pdf_path)
    for page_num in pages:
        page = doc[page_num - 1]
        text = page.get_text()
        print(f"--- Page {page_num} ---")
        print(text)
```

---

## 想定される調査結果

### パターンA: 自動判定機能あり

u-bloxが提供するAutobaud機能を使用。DeviceManagerで複数ボーレートを試行する必要なし。

### パターンB: 自動判定機能なし（ホスト側で実装）

複数ボーレート（115200, 38400, 9600など）を順に試行し、応答を確認する方式。

### パターンC: 手動指定のまま

最初の1台で確認したボーレートをロット全体に適用。現場運用として問題なし。

---

## 次回セッションでやること

1. Integration ManualのPDFを取得
2. 目次を抽出してボーレート関連セクションを特定
3. 該当ページを抽出して分析
4. 設計判断（ADR候補）

---

## 参照資料

- [Session 78 CFG-RATE/PRT抽出結果](../session78/cfg-rate-prt-raw.md)
- [Session 17 PDF分析](../session17/pdf-excel-analysis.md)
- [Session 83 F9P実機テスト](../session83/session-summary.md)
