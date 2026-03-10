#!/usr/bin/env python3
"""
APIマニュアルの目次ページ（1-3）を抽出するスクリプト

使用例:
    python extract_toc.py
"""

from pathlib import Path
import sys

try:
    import fitz  # PyMuPDF
except ImportError:
    print("Error: pymupdf required. Install with: pip install pymupdf")
    sys.exit(1)


def main():
    script_dir = Path(__file__).parent
    pdf_path = script_dir / "AS-DT1_APIマニュアル_FW1.00.pdf"
    output_file = script_dir / "api-manual-toc.md"

    if not pdf_path.exists():
        print(f"Error: PDF not found: {pdf_path}")
        sys.exit(1)

    doc = fitz.open(pdf_path)
    print(f"PDF: {pdf_path.name} ({len(doc)} pages)")

    lines = [
        "# AS-DT1 APIマニュアル 目次",
        "",
        f"**総ページ数**: {len(doc)}",
        "",
        "---",
        "",
    ]

    # 目次ページ（1-3）を抽出
    for page_num in [1, 2, 3]:
        page_idx = page_num - 1
        if page_idx < len(doc):
            page = doc[page_idx]
            text = page.get_text().strip()
            lines.append(f"## Page {page_num}")
            lines.append("")
            lines.append("```")
            lines.append(text)
            lines.append("```")
            lines.append("")

    doc.close()

    output_file.write_text("\n".join(lines), encoding="utf-8")
    print(f"Output: {output_file}")


if __name__ == "__main__":
    main()
