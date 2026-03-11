#!/usr/bin/env python3
"""
PDFから指定ページを抽出してテキストファイルに保存するスクリプト
"""

import fitz  # PyMuPDF
import os

PDF_PATH = "../session64/アプリケーションノート_u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf"
OUTPUT_DIR = "extracted"

# 抽出するページ（PDFページ番号、0-indexed）
# 目次のページ番号はPDFの論理ページ番号なので、そのまま-1で0-indexedにする
PAGES_TO_EXTRACT = {
    "mon-span": (134, 135),     # p.134-135 (MON-SPAN + MON-SYS)
    "nav-sat": (150, 152),      # p.150-151 (NAV-SAT)
    "nav-sig": (152, 155),      # p.152-154 (NAV-SIG)
}


def extract_pages(doc, start_page: int, end_page: int) -> str:
    """指定範囲のページをテキスト抽出"""
    text_parts = []

    # 0-indexed に変換
    for page_num in range(start_page - 1, end_page - 1):
        if 0 <= page_num < len(doc):
            page = doc[page_num]
            text = page.get_text()
            text_parts.append(f"\n{'='*60}\nPage {page_num + 1}\n{'='*60}\n")
            text_parts.append(text)

    return "\n".join(text_parts)


def main():
    os.makedirs(OUTPUT_DIR, exist_ok=True)

    doc = fitz.open(PDF_PATH)
    print(f"PDF: {len(doc)} pages")

    for name, (start, end) in PAGES_TO_EXTRACT.items():
        print(f"Extracting {name}: pages {start}-{end-1}...")
        text = extract_pages(doc, start, end)

        output_path = os.path.join(OUTPUT_DIR, f"{name}.txt")
        with open(output_path, "w", encoding="utf-8") as f:
            f.write(text)
        print(f"  -> {output_path}")

    doc.close()
    print("Done!")


if __name__ == "__main__":
    main()
