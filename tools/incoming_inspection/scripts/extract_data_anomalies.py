#!/usr/bin/env python3
"""データ異常の抽出・レポート作成"""

import csv
from pathlib import Path
from collections import defaultdict


def load_all_records(raw_dir: Path) -> list[dict]:
    """全生データCSVからレコードを読み込む"""
    records = []
    for csv_file in raw_dir.glob("*_生データ.csv"):
        with open(csv_file, encoding="utf-8-sig") as f:
            reader = csv.DictReader(f)
            for row in reader:
                records.append(row)
    return records


def analyze_future_dates(records: list[dict]) -> list[dict]:
    """2026年の未来日付を持つレコードを抽出"""
    future_records = []
    for r in records:
        入荷日 = r.get("入荷日", "")
        検査完了日 = r.get("検査完了日", "")

        # 2026-11 以降のレコードを抽出（2025年3月時点での「未来」）
        is_future = False
        if 入荷日.startswith("2026-1"):  # 2026-10, 2026-11, 2026-12
            is_future = True
        if 検査完了日.startswith("2026-1"):
            is_future = True

        if is_future:
            future_records.append(r)

    return future_records


def analyze_missing_arrival_dates(records: list[dict]) -> list[dict]:
    """入荷日が空のレコードを抽出"""
    missing = []
    for r in records:
        入荷日 = r.get("入荷日", "").strip()
        if not 入荷日:
            missing.append(r)
    return missing


def main():
    raw_dir = Path("sessions/session19/csv-output/raw")
    records = load_all_records(raw_dir)
    print(f"総レコード数: {len(records)}")

    # 1. 未来日付の分析
    print("\n" + "="*60)
    print("1. 未来日付（2026-10以降）のレコード")
    print("="*60)

    future_records = analyze_future_dates(records)
    print(f"該当レコード数: {len(future_records)}")

    if future_records:
        print("\n【詳細】")
        for r in future_records:
            print(f"  カテゴリ: {r.get('カテゴリ', '')}")
            print(f"  品名: {r.get('品名', '')}")
            print(f"  入荷日: {r.get('入荷日', '')}")
            print(f"  検査完了日: {r.get('検査完了日', '')}")
            print(f"  作業者: {r.get('作業者', '')}")
            print(f"  検査工数(分): {r.get('検査工数(分)', '')}")
            print()

    # 2. 入荷日不明の分析
    print("\n" + "="*60)
    print("2. 入荷日不明のレコード")
    print("="*60)

    missing_arrival = analyze_missing_arrival_dates(records)
    print(f"該当レコード数: {len(missing_arrival)}")

    # カテゴリ別集計
    by_category = defaultdict(list)
    for r in missing_arrival:
        cat = r.get("カテゴリ", "不明")
        by_category[cat].append(r)

    print("\n【カテゴリ別内訳】")
    for cat, items in sorted(by_category.items(), key=lambda x: -len(x[1])):
        print(f"  {cat}: {len(items)}件")

    # 検査内容別集計
    by_inspection = defaultdict(list)
    for r in missing_arrival:
        inspection = r.get("検査内容", "不明")
        by_inspection[inspection].append(r)

    print("\n【検査内容別内訳】")
    for insp, items in sorted(by_inspection.items(), key=lambda x: -len(x[1])):
        print(f"  {insp}: {len(items)}件")

    # 総工数
    total_minutes = 0
    for r in missing_arrival:
        try:
            minutes = float(r.get("検査工数(分)", "0") or "0")
            total_minutes += minutes
        except ValueError:
            pass
    print(f"\n【総工数】{total_minutes:.1f}分 ({total_minutes/60:.2f}時間)")

    # 詳細リスト
    print("\n【詳細リスト】")
    for i, r in enumerate(missing_arrival, 1):
        print(f"{i:2d}. [{r.get('カテゴリ', '')}] {r.get('品名', '')} / {r.get('検査内容', '')} / {r.get('検査工数(分)', '')}分")

    # レポート用データ出力
    output_dir = Path("sessions/session22")

    # 未来日付CSV
    with open(output_dir / "異常_未来日付.csv", "w", encoding="utf-8-sig", newline="") as f:
        if future_records:
            writer = csv.DictWriter(f, fieldnames=future_records[0].keys())
            writer.writeheader()
            writer.writerows(future_records)

    # 入荷日不明CSV
    with open(output_dir / "異常_入荷日不明.csv", "w", encoding="utf-8-sig", newline="") as f:
        if missing_arrival:
            writer = csv.DictWriter(f, fieldnames=missing_arrival[0].keys())
            writer.writeheader()
            writer.writerows(missing_arrival)

    print(f"\n出力: {output_dir}/異常_未来日付.csv")
    print(f"出力: {output_dir}/異常_入荷日不明.csv")


if __name__ == "__main__":
    main()
