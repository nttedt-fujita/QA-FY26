#!/usr/bin/env python3
"""品名のユニーク値抽出・表記揺れ分析"""

import csv
from pathlib import Path
from collections import Counter
import unicodedata
import re

def load_all_product_names(raw_dir: Path) -> list[tuple[str, str]]:
    """全生データCSVから品名を抽出（品名, カテゴリ）"""
    names = []
    for csv_file in raw_dir.glob("*_生データ.csv"):
        with open(csv_file, encoding="utf-8-sig") as f:
            reader = csv.DictReader(f)
            for row in reader:
                name = row.get("品名", "").strip()
                category = row.get("カテゴリ", "").strip()
                if name:
                    names.append((name, category))
    return names

def normalize_for_comparison(s: str) -> str:
    """比較用の正規化（全角→半角、スペース統一、小文字化）"""
    # NFKC正規化（全角英数→半角）
    s = unicodedata.normalize("NFKC", s)
    # 複数スペース→単一スペース
    s = re.sub(r"\s+", " ", s)
    # 小文字化
    s = s.lower()
    # 前後の空白除去
    s = s.strip()
    return s

def find_similar_names(names: list[str]) -> dict[str, list[str]]:
    """正規化後に同じになる品名グループを検出"""
    normalized_to_original = {}
    for name in names:
        norm = normalize_for_comparison(name)
        if norm not in normalized_to_original:
            normalized_to_original[norm] = []
        if name not in normalized_to_original[norm]:
            normalized_to_original[norm].append(name)

    # 2つ以上のバリエーションがあるものだけ抽出
    similar = {k: v for k, v in normalized_to_original.items() if len(v) > 1}
    return similar

def main():
    raw_dir = Path("sessions/session19/csv-output/raw")

    # 品名抽出
    name_category_pairs = load_all_product_names(raw_dir)
    print(f"総レコード数: {len(name_category_pairs)}")

    # 品名のみ抽出
    all_names = [n for n, _ in name_category_pairs]

    # カウント
    name_counts = Counter(all_names)
    print(f"ユニーク品名数: {len(name_counts)}")

    # 類似品名の検出
    similar = find_similar_names(list(name_counts.keys()))
    print(f"\n表記揺れグループ数: {len(similar)}")

    print("\n" + "="*60)
    print("表記揺れパターン一覧")
    print("="*60)

    for norm, variants in sorted(similar.items()):
        print(f"\n【{norm}】")
        for v in variants:
            count = name_counts[v]
            print(f"  - '{v}' ({count}件)")

    # CSVとして出力（名寄せルール用）
    output_path = Path("sessions/session22/品名_表記揺れ一覧.csv")
    with open(output_path, "w", encoding="utf-8-sig", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(["正規化後", "元の表記", "出現回数", "推奨（正規化先）"])
        for norm, variants in sorted(similar.items()):
            for i, v in enumerate(sorted(variants, key=lambda x: -name_counts[x])):
                # 最多出現を推奨
                recommended = variants[0] if i == 0 else ""
                writer.writerow([norm, v, name_counts[v], recommended])

    print(f"\n出力: {output_path}")

    # 上位品名（出現回数順）
    print("\n" + "="*60)
    print("品名出現回数 TOP30")
    print("="*60)
    for name, count in name_counts.most_common(30):
        print(f"  {count:3d}件: {name}")

if __name__ == "__main__":
    main()
