#!/usr/bin/env python3
"""
AS-DT1ユーザーズガイドから追加ページ（29-32）を抽出するスクリプト

Session 67で抽出漏れがあったページを追加抽出する。
対象:
- p29-30: カバーガラスのお手入れ方法、その他注意事項
- p31-32: 主な仕様（p31は抽出済みだが再確認）

使用例:
    python extract_additional_pages.py
"""

from pathlib import Path
import sys

try:
    import fitz  # PyMuPDF
except ImportError:
    print("Error: pymupdf required. Install with: pip install pymupdf")
    sys.exit(1)


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
    # パス設定（Session 67のPDFを参照）
    script_dir = Path(__file__).parent
    pdf_path = script_dir.parent / "session67" / "AS-DT1_ユーザーズガイド_FW1.00.pdf"
    output_dir = script_dir / "extracted"

    if not pdf_path.exists():
        print(f"Error: PDF not found: {pdf_path}")
        sys.exit(1)

    # 出力ディレクトリ作成
    output_dir.mkdir(exist_ok=True)

    # PDF読み込み
    doc = fitz.open(pdf_path)
    print(f"PDF: {pdf_path.name} ({len(doc)} pages)")

    # 抽出対象: 8, 20, 29-32ページ
    pages = [8, 20, 29, 30, 31, 32]
    output_file = output_dir / "07-maintenance-and-specs.md"

    content_lines = [
        "# お手入れ方法・注意事項・主な仕様",
        "",
        "**出典**: AS-DT1 ユーザーズガイド FW1.00",
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
