#!/usr/bin/env python3
"""
AS-DT1 APIマニュアルからカテゴリ別にMarkdownファイルを抽出するスクリプト

使用例:
    python extract_api_manual.py
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
    "mode_trigger_sync": {
        "pages": list(range(5, 22)),  # p5-21
        "output": "01-mode-trigger-sync.md",
        "title": "モード・トリガー・同期・出力形式",
    },
    "obstacle_imu": {
        "pages": [30, 31, 32, 33],
        "output": "02-obstacle-imu-timesync.md",
        "title": "障害物検出・IMU・時間同期",
    },
    "firmware_update": {
        "pages": [37, 38],
        "output": "03-firmware-update.md",
        "title": "ファームウェアの更新",
    },
    "environment_setup": {
        "pages": [61, 62, 63],
        "output": "04-environment-setup.md",
        "title": "環境設定（Windows/Linux/Jetson/Raspberry Pi）",
    },
    "api_pointcloud": {
        "pages": list(range(66, 76)),  # p66-75
        "output": "05-api-pointcloud.md",
        "title": "Python API: 点群処理 (as_dt1_api_cdc.py)",
    },
    "api_update_enumeration": {
        "pages": [75, 76, 77, 78],
        "output": "06-api-update-enumeration.md",
        "title": "Python API: アップデート・デバイス検索",
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
    pdf_path = script_dir / "AS-DT1_APIマニュアル_FW1.00.pdf"
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
            f"**出典**: AS-DT1 APIマニュアル FW1.00",
            f"**抽出ページ**: {pages[0]}-{pages[-1]} ({len(pages)}ページ)",
            "",
            "---",
            "",
        ]

        extracted = extract_pages(doc, pages)
        content_lines.append(extracted)

        output_file.write_text("\n".join(content_lines), encoding="utf-8")
        print(f"  Created: {output_file.name} (pages {pages[0]}-{pages[-1]})")

    doc.close()
    print(f"\nDone. Output directory: {output_dir}")


if __name__ == "__main__":
    main()
