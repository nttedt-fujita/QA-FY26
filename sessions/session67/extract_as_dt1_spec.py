#!/usr/bin/env python3
"""
AS-DT1仕様書からカテゴリ別にMarkdownファイルを抽出するスクリプト

使用例:
    python extract_as_dt1_spec.py
"""

from pathlib import Path
import sys

try:
    import fitz  # PyMuPDF
except ImportError:
    print("Error: pymupdf required. Install with: pip install pymupdf")
    sys.exit(1)


# 抽出設定: カテゴリ名 → (ページリスト, 出力ファイル名, 説明)
EXTRACTION_CONFIG = {
    "overview": {
        "pages": [4],
        "output": "01-overview.md",
        "title": "概要・特長",
    },
    "hardware": {
        "pages": [5, 6, 7, 8],
        "output": "02-hardware.md",
        "title": "各部名称・コネクタ・取付",
    },
    "interface": {
        "pages": [17, 18, 19],
        "output": "03-interface.md",
        "title": "周辺機器接続（USB/UART/電源/トリガー）",
    },
    "measurement": {
        "pages": [24, 25, 26, 27],
        "output": "04-measurement.md",
        "title": "測距仕様・性能",
    },
    "specifications": {
        "pages": [31],
        "output": "05-specifications.md",
        "title": "主な仕様・外形寸法",
    },
    "commands": {
        "pages": [33],
        "output": "06-commands.md",
        "title": "対応コマンド一覧",
    },
}


def extract_pages(doc, pages: list[int]) -> str:
    """指定ページのテキストを抽出（1-indexed）"""
    lines = []
    for page_num in pages:
        page_idx = page_num - 1  # 0-indexed
        if 0 <= page_idx < len(doc):
            page = doc[page_idx]
            text = page.get_text().strip()
            lines.append(f"## Page {page_num}\n")
            lines.append("```")
            lines.append(text)
            lines.append("```\n")
    return "\n".join(lines)


def main():
    # パス設定
    script_dir = Path(__file__).parent
    pdf_path = script_dir / "AS-DT1_ユーザーズガイド_FW1.00.pdf"
    output_dir = script_dir / "extracted"

    if not pdf_path.exists():
        print(f"Error: PDF not found: {pdf_path}")
        sys.exit(1)

    # 出力ディレクトリ作成
    output_dir.mkdir(exist_ok=True)

    # PDF読み込み
    doc = fitz.open(pdf_path)
    print(f"PDF: {pdf_path.name} ({len(doc)} pages)")

    # カテゴリ別に抽出
    for category, config in EXTRACTION_CONFIG.items():
        pages = config["pages"]
        output_file = output_dir / config["output"]
        title = config["title"]

        content_lines = [
            f"# {title}",
            "",
            f"**出典**: AS-DT1 ユーザーズガイド FW1.00",
            f"**抽出ページ**: {', '.join(map(str, pages))}",
            "",
            "---",
            "",
        ]

        extracted = extract_pages(doc, pages)
        content_lines.append(extracted)

        output_file.write_text("\n".join(content_lines), encoding="utf-8")
        print(f"  Created: {output_file.name} (pages {pages})")

    doc.close()
    print(f"\nDone. Output directory: {output_dir}")


if __name__ == "__main__":
    main()
