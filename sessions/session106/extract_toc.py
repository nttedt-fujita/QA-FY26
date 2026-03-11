#!/usr/bin/env python3
"""
PDFから目次を抽出するスクリプト
"""

import fitz  # PyMuPDF

PDF_PATH = "../session64/アプリケーションノート_u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf"

def main():
    doc = fitz.open(PDF_PATH)

    print(f"PDF情報:")
    print(f"  ページ数: {len(doc)}")
    print(f"  タイトル: {doc.metadata.get('title', '不明')}")
    print()

    # 目次を取得
    toc = doc.get_toc()

    if toc:
        print("=" * 60)
        print("目次（Table of Contents）")
        print("=" * 60)
        for level, title, page in toc:
            indent = "  " * (level - 1)
            print(f"{indent}{title} ... p.{page}")
    else:
        print("目次が埋め込まれていません。")
        print("最初の数ページからテキスト抽出を試みます...")
        print()

        # 最初の10ページからContentsを探す
        for page_num in range(min(10, len(doc))):
            page = doc[page_num]
            text = page.get_text()
            if "Contents" in text or "目次" in text or "MON-SPAN" in text:
                print(f"--- Page {page_num + 1} ---")
                print(text[:2000])
                print()

    doc.close()

if __name__ == "__main__":
    main()
