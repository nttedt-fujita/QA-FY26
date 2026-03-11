#!/usr/bin/env python3
"""
PDFの目次からMON-SPANを検索するスクリプト
"""

import fitz  # PyMuPDF

PDF_PATH = "../session64/アプリケーションノート_u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf"

def main():
    doc = fitz.open(PDF_PATH)
    toc = doc.get_toc()

    print("=" * 60)
    print("MON関連のメッセージを検索")
    print("=" * 60)

    found_mon = False
    for level, title, page in toc:
        # MONを含むエントリを表示
        if "MON" in title.upper():
            indent = "  " * (level - 1)
            print(f"{indent}{title} ... p.{page}")
            found_mon = True

    if not found_mon:
        print("目次にMONが見つかりません。全目次からSPANを検索...")
        for level, title, page in toc:
            if "SPAN" in title.upper():
                indent = "  " * (level - 1)
                print(f"{indent}{title} ... p.{page}")

    # NAV-SIG, NAV-SATも確認
    print()
    print("=" * 60)
    print("NAV-SAT, NAV-SIG関連")
    print("=" * 60)
    for level, title, page in toc:
        if "NAV-SAT" in title or "NAV-SIG" in title:
            indent = "  " * (level - 1)
            print(f"{indent}{title} ... p.{page}")

    doc.close()

if __name__ == "__main__":
    main()
