#!/usr/bin/env python3
"""
PDFから特定ページを抽出するツール

使用例:
    python pdf_page_extractor.py input.pdf 132-134 output.md
    python pdf_page_extractor.py input.pdf 155 output.md
"""

import argparse
import sys
from pathlib import Path

try:
    import fitz  # PyMuPDF
except ImportError:
    print("Error: pymupdf required. Install with: pip install pymupdf")
    sys.exit(1)


def parse_page_range(page_spec: str) -> list[int]:
    """
    ページ指定をパースして0-indexedのページ番号リストを返す

    入力例: "132-134" -> [131, 132, 133]
            "155" -> [154]
            "132,155,200-202" -> [131, 154, 199, 200, 201]
    """
    pages = []
    for part in page_spec.split(","):
        part = part.strip()
        if "-" in part:
            start, end = part.split("-", 1)
            start_page = int(start) - 1  # 0-indexed
            end_page = int(end)  # 1-indexed end inclusive -> 0-indexed exclusive
            pages.extend(range(start_page, end_page))
        else:
            pages.append(int(part) - 1)  # 0-indexed
    return sorted(set(pages))


def extract_pages_to_markdown(pdf_path: str, pages: list[int], output_path: str | None = None) -> str:
    """
    指定ページのテキストをMarkdown形式で抽出
    """
    doc = fitz.open(pdf_path)
    total_pages = len(doc)

    output_lines = [
        f"# PDF抽出: {Path(pdf_path).name}",
        f"",
        f"**総ページ数**: {total_pages}",
        f"**抽出ページ**: {', '.join(str(p + 1) for p in pages)}",
        f"",
        "---",
        "",
    ]

    for page_num in pages:
        if page_num < 0 or page_num >= total_pages:
            output_lines.append(f"## Page {page_num + 1} - 範囲外")
            output_lines.append("")
            continue

        page = doc[page_num]
        text = page.get_text()

        output_lines.append(f"## Page {page_num + 1}")
        output_lines.append("")
        output_lines.append("```")
        output_lines.append(text.strip())
        output_lines.append("```")
        output_lines.append("")
        output_lines.append("---")
        output_lines.append("")

    doc.close()

    result = "\n".join(output_lines)

    if output_path:
        Path(output_path).write_text(result, encoding="utf-8")
        print(f"Saved to: {output_path}")

    return result


def main():
    parser = argparse.ArgumentParser(description="PDFから特定ページを抽出")
    parser.add_argument("pdf", help="入力PDFファイル")
    parser.add_argument("pages", help="ページ指定（例: 132-134, 155, 132,155,200-202）")
    parser.add_argument("output", nargs="?", help="出力ファイル（省略時は標準出力）")

    args = parser.parse_args()

    pages = parse_page_range(args.pages)
    result = extract_pages_to_markdown(args.pdf, pages, args.output)

    if not args.output:
        print(result)


if __name__ == "__main__":
    main()
