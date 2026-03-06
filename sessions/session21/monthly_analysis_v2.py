"""月別分析スクリプト v2（名寄せ機能付き）

受入検査工数の月別分析（パレート + 4M分解）を行う。
名寄せルールCSVが存在する場合、検査内容の表記揺れを正規化する。
"""

import csv
import re
import math
from pathlib import Path
from collections import defaultdict

from name_normalizer import NameNormalizer

# CSVディレクトリ
RAW_CSV_DIR = Path(__file__).parent.parent / 'session19' / 'csv-output' / 'raw'
MAPPING_DIR = Path(__file__).parent / 'mapping'
OUT_DIR = Path(__file__).parent / 'csv-output-v2'


def extract_year_month(date_str):
    """日付文字列から年月を抽出する"""
    if not date_str:
        return None
    if not isinstance(date_str, str):
        return None
    match = re.match(r'^(\d{4}-\d{2})-\d{2}$', date_str)
    if match:
        return match.group(1)
    return None


def parse_minutes(value):
    """工数値をfloatにパースする"""
    if value is None:
        return None
    if isinstance(value, (int, float)):
        return float(value)
    if isinstance(value, str):
        value = value.strip()
        if not value:
            return None
        try:
            return float(value)
        except ValueError:
            return None
    return None


def load_all_raw_csv():
    """全ての生データCSVを読み込む"""
    records = []
    for csv_file in RAW_CSV_DIR.glob('*_生データ.csv'):
        with open(csv_file, 'r', encoding='utf-8-sig') as f:
            reader = csv.DictReader(f)
            for row in reader:
                records.append(row)
    return records


class MonthlyAnalyzer:
    """月別分析クラス（名寄せ対応版）"""

    def __init__(self, records, inspection_normalizer=None):
        """
        Args:
            records: レコードのリスト
            inspection_normalizer: 検査内容用のNameNormalizer（任意）
        """
        self.records = records
        self.inspection_normalizer = inspection_normalizer
        self._monthly_data = None

    def _normalize_inspection(self, value):
        """検査内容を名寄せする"""
        if self.inspection_normalizer:
            return self.inspection_normalizer.normalize(value)
        return value

    def _build_monthly_data(self):
        """月別にレコードを分類する"""
        if self._monthly_data is not None:
            return self._monthly_data

        self._monthly_data = defaultdict(list)
        for rec in self.records:
            year_month = extract_year_month(rec.get('入荷日', ''))
            if year_month is None:
                year_month = '不明'
            self._monthly_data[year_month].append(rec)
        return self._monthly_data

    def get_months(self):
        """月の一覧を取得（ソート済み）"""
        monthly = self._build_monthly_data()
        months = sorted([m for m in monthly.keys() if m != '不明'])
        if '不明' in monthly:
            months.append('不明')
        return months

    def get_monthly_summary(self):
        """月別サマリーを取得"""
        monthly = self._build_monthly_data()
        summary = {}

        for month, recs in monthly.items():
            件数 = len(recs)
            parts = set()
            workers = set()
            categories = defaultdict(int)
            worker_counts = defaultdict(int)
            part_minutes = defaultdict(float)

            総工数 = 0.0
            工数記録あり = 0

            for rec in recs:
                part = rec.get('品名', '') or '不明'
                parts.add(part)

                worker = rec.get('作業者', '') or '不明'
                workers.add(worker)
                worker_counts[worker] += 1

                cat = rec.get('カテゴリ', '') or '不明'
                categories[cat] += 1

                minutes = parse_minutes(rec.get('検査工数(分)'))
                if minutes is not None:
                    総工数 += minutes
                    工数記録あり += 1
                    part_minutes[part] += minutes

            pareto = self._calc_pareto(part_minutes, 総工数)
            最多担当者 = max(worker_counts.items(), key=lambda x: x[1])[0]

            sorted_parts = sorted(
                part_minutes.items(), key=lambda x: x[1], reverse=True
            )
            工数トップ3 = [p[0] for p in sorted_parts[:3]]

            カテゴリ別割合 = {}
            for cat, cnt in categories.items():
                カテゴリ別割合[cat] = round(cnt / 件数 * 100, 1)

            summary[month] = {
                '件数': 件数,
                '総工数(分)': 総工数,
                '工数記録あり': 工数記録あり,
                '部品種類数': len(parts),
                '作業者数': len(workers),
                'カテゴリ別': dict(categories),
                'カテゴリ別割合(%)': カテゴリ別割合,
                'パレート指標(%)': pareto,
                '最多担当者': 最多担当者,
                '工数トップ3': 工数トップ3,
            }

        return summary

    def _calc_pareto(self, part_minutes, total):
        """パレート指標を計算"""
        if total == 0 or not part_minutes:
            return 0.0
        sorted_parts = sorted(part_minutes.values(), reverse=True)
        top_count = max(1, math.ceil(len(sorted_parts) * 0.2))
        top_sum = sum(sorted_parts[:top_count])
        return round(top_sum / total * 100, 2)

    def get_monthly_by_part(self):
        """月別×品名の集計"""
        monthly = self._build_monthly_data()
        result = {}

        for month, recs in monthly.items():
            by_part = defaultdict(lambda: {'件数': 0, '工数(分)': 0.0})
            for rec in recs:
                part = rec.get('品名', '') or '不明'
                by_part[part]['件数'] += 1
                minutes = parse_minutes(rec.get('検査工数(分)'))
                if minutes is not None:
                    by_part[part]['工数(分)'] += minutes
            result[month] = dict(by_part)

        return result

    def get_monthly_by_worker(self):
        """月別×作業者の集計"""
        monthly = self._build_monthly_data()
        result = {}

        for month, recs in monthly.items():
            by_worker = defaultdict(
                lambda: {'件数': 0, '工数(分)': 0.0, '工数記録あり': 0}
            )
            for rec in recs:
                worker = rec.get('作業者', '') or '不明'
                by_worker[worker]['件数'] += 1
                minutes = parse_minutes(rec.get('検査工数(分)'))
                if minutes is not None:
                    by_worker[worker]['工数(分)'] += minutes
                    by_worker[worker]['工数記録あり'] += 1

            for worker, data in by_worker.items():
                if data['工数記録あり'] > 0:
                    data['平均(分)'] = round(
                        data['工数(分)'] / data['工数記録あり'], 1
                    )
                else:
                    data['平均(分)'] = 0.0

            result[month] = dict(by_worker)

        return result

    def get_monthly_by_category(self):
        """月別×カテゴリの集計"""
        monthly = self._build_monthly_data()
        result = {}

        for month, recs in monthly.items():
            by_cat = defaultdict(lambda: {'件数': 0, '工数(分)': 0.0})
            for rec in recs:
                cat = rec.get('カテゴリ', '') or '不明'
                by_cat[cat]['件数'] += 1
                minutes = parse_minutes(rec.get('検査工数(分)'))
                if minutes is not None:
                    by_cat[cat]['工数(分)'] += minutes
            result[month] = dict(by_cat)

        return result

    def get_monthly_by_inspection(self):
        """月別×検査内容の集計（名寄せ適用）"""
        monthly = self._build_monthly_data()
        result = {}

        for month, recs in monthly.items():
            by_insp = defaultdict(lambda: {'件数': 0, '工数(分)': 0.0})
            for rec in recs:
                # 名寄せを適用
                insp_raw = rec.get('検査内容', '') or '不明'
                insp = self._normalize_inspection(insp_raw)
                by_insp[insp]['件数'] += 1
                minutes = parse_minutes(rec.get('検査工数(分)'))
                if minutes is not None:
                    by_insp[insp]['工数(分)'] += minutes
            result[month] = dict(by_insp)

        return result


def main():
    """メイン処理"""
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    detail_dir = OUT_DIR / 'detail'
    detail_dir.mkdir(exist_ok=True)

    print("=" * 60)
    print("月別分析スクリプト v2（名寄せ機能付き）")
    print("=" * 60)

    # 名寄せルール読み込み
    inspection_rules_path = MAPPING_DIR / '検査内容_名寄せルール.csv'
    inspection_normalizer = None
    if inspection_rules_path.exists():
        inspection_normalizer = NameNormalizer(inspection_rules_path)
        print(f"名寄せルール読み込み: {inspection_normalizer.get_rule_count()}件")
    else:
        print("名寄せルール: なし（名寄せなしで実行）")

    # データ読み込み
    records = load_all_raw_csv()
    print(f"読み込みレコード数: {len(records)}件")

    analyzer = MonthlyAnalyzer(records, inspection_normalizer)
    months = analyzer.get_months()
    print(f"対象月: {', '.join(months)}")

    # 1. 月別サマリーCSV
    summary = analyzer.get_monthly_summary()
    summary_path = OUT_DIR / '月別サマリー.csv'
    with open(summary_path, 'w', newline='', encoding='utf-8-sig') as f:
        writer = csv.writer(f)
        writer.writerow([
            '年月', '件数', '総工数(分)', '総工数(時間)', '工数記録あり',
            '部品種類数', '作業者数', '最多担当者', 'パレート指標(%)',
            '工数トップ1', '工数トップ2', '工数トップ3',
            'メカ件数', 'エレキ件数', 'Api件数'
        ])
        for month in months:
            s = summary[month]
            top3 = s['工数トップ3'] + [''] * (3 - len(s['工数トップ3']))
            writer.writerow([
                month,
                s['件数'],
                s['総工数(分)'],
                round(s['総工数(分)'] / 60, 2),
                s['工数記録あり'],
                s['部品種類数'],
                s['作業者数'],
                s['最多担当者'],
                s['パレート指標(%)'],
                top3[0], top3[1], top3[2],
                s['カテゴリ別'].get('メカ', 0),
                s['カテゴリ別'].get('エレキ', 0),
                s['カテゴリ別'].get('Api', 0),
            ])
    print(f"出力: {summary_path}")

    # 2. 月別×部品CSV
    by_part = analyzer.get_monthly_by_part()
    part_path = detail_dir / '月別×部品.csv'
    with open(part_path, 'w', newline='', encoding='utf-8-sig') as f:
        writer = csv.writer(f)
        writer.writerow(['年月', '品名', '件数', '工数(分)', '工数(時間)'])
        for month in months:
            for part, data in sorted(
                by_part[month].items(),
                key=lambda x: x[1]['工数(分)'],
                reverse=True
            ):
                writer.writerow([
                    month, part, data['件数'],
                    data['工数(分)'], round(data['工数(分)'] / 60, 2)
                ])
    print(f"出力: {part_path}")

    # 3. 月別×作業者CSV
    by_worker = analyzer.get_monthly_by_worker()
    worker_path = detail_dir / '月別×作業者.csv'
    with open(worker_path, 'w', newline='', encoding='utf-8-sig') as f:
        writer = csv.writer(f)
        writer.writerow([
            '年月', '作業者', '件数', '工数(分)', '工数(時間)', '平均(分/件)'
        ])
        for month in months:
            for worker, data in sorted(
                by_worker[month].items(),
                key=lambda x: x[1]['工数(分)'],
                reverse=True
            ):
                writer.writerow([
                    month, worker, data['件数'],
                    data['工数(分)'], round(data['工数(分)'] / 60, 2),
                    data['平均(分)']
                ])
    print(f"出力: {worker_path}")

    # 4. 月別×カテゴリCSV
    by_cat = analyzer.get_monthly_by_category()
    cat_path = detail_dir / '月別×カテゴリ.csv'
    with open(cat_path, 'w', newline='', encoding='utf-8-sig') as f:
        writer = csv.writer(f)
        writer.writerow(['年月', 'カテゴリ', '件数', '工数(分)', '工数(時間)'])
        for month in months:
            for cat, data in sorted(
                by_cat[month].items(),
                key=lambda x: x[1]['工数(分)'],
                reverse=True
            ):
                writer.writerow([
                    month, cat, data['件数'],
                    data['工数(分)'], round(data['工数(分)'] / 60, 2)
                ])
    print(f"出力: {cat_path}")

    # 5. 月別×検査内容CSV（名寄せ適用）
    by_insp = analyzer.get_monthly_by_inspection()
    insp_path = detail_dir / '月別×検査内容.csv'
    with open(insp_path, 'w', newline='', encoding='utf-8-sig') as f:
        writer = csv.writer(f)
        writer.writerow(['年月', '検査内容', '件数', '工数(分)', '工数(時間)'])
        for month in months:
            for insp, data in sorted(
                by_insp[month].items(),
                key=lambda x: x[1]['工数(分)'],
                reverse=True
            ):
                writer.writerow([
                    month, insp, data['件数'],
                    data['工数(分)'], round(data['工数(分)'] / 60, 2)
                ])
    print(f"出力: {insp_path}")

    # 名寄せ統計
    if inspection_normalizer:
        stats = inspection_normalizer.get_stats()
        print()
        print(f"名寄せ統計:")
        print(f"  処理件数: {stats['total_processed']}")
        print(f"  変換件数: {stats['total_converted']}")
        print(f"  変換率: {stats['conversion_rate']}%")

    print()
    print("完了!")


if __name__ == '__main__':
    main()
