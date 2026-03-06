"""月別分析スクリプトのテスト（TDD）

analysis-plan.md に基づく月別分析機能のテスト。
Phase 2で承認されたテストシナリオリストに基づく。
"""

import pytest
from pathlib import Path

# テスト用のサンプルデータ（8件）
SAMPLE_RECORDS = [
    # 2024-09: メカ3件、エレキ1件
    {'カテゴリ': 'メカ', '入荷日': '2024-09-03', '品名': '保護パッド', '検査工数(分)': '5', '作業者': '西村', '検査内容': '外観'},
    {'カテゴリ': 'メカ', '入荷日': '2024-09-03', '品名': '隙間テープ', '検査工数(分)': '5', '作業者': '西村', '検査内容': '外観'},
    {'カテゴリ': 'メカ', '入荷日': '2024-09-19', '品名': 'Battery mount LH', '検査工数(分)': '20', '作業者': '西村', '検査内容': '外観'},
    {'カテゴリ': 'エレキ', '入荷日': '2024-09-25', '品名': 'U8 Lite Armset CW', '検査工数(分)': '130', '作業者': '田中', '検査内容': '機能'},
    # 2024-10: メカ2件
    {'カテゴリ': 'メカ', '入荷日': '2024-10-01', '品名': 'LED cover', '検査工数(分)': '15', '作業者': '西村', '検査内容': '外観'},
    {'カテゴリ': 'メカ', '入荷日': '2024-10-15', '品名': 'Slide Rail', '検査工数(分)': '60', '作業者': '佐藤', '検査内容': '寸法'},
    # 2024-11: Api1件、工数記録なし1件
    {'カテゴリ': 'Api', '入荷日': '2024-11-01', '品名': 'Skid(Assy)', '検査工数(分)': '100', '作業者': '西村', '検査内容': '外観'},
    {'カテゴリ': 'メカ', '入荷日': '2024-11-10', '品名': 'テスト部品', '検査工数(分)': '', '作業者': '西村', '検査内容': '外観'},
]

# 入荷日不明のテスト用データ
SAMPLE_WITH_UNKNOWN_DATE = SAMPLE_RECORDS + [
    {'カテゴリ': 'メカ', '入荷日': '', '品名': '不明部品', '検査工数(分)': '10', '作業者': '西村', '検査内容': '外観'},
]


# =============================================================================
# A. ユーティリティ関数のテスト
# =============================================================================

class TestExtractYearMonth:
    """A1-A3: 年月抽出のテスト"""

    @pytest.mark.parametrize("date_str,expected,should_succeed", [
        # 正常系
        ('2024-09-03', '2024-09', True),
        ('2024-10-01', '2024-10', True),
        ('2024-11-15', '2024-11', True),
        ('2025-01-01', '2025-01', True),
        # 異常系（空文字・None）
        ('', None, False),
        (None, None, False),
        # 異常系（不正形式）
        ('invalid', None, False),
        ('2024/09/03', None, False),
    ])
    def test_extract_year_month(self, date_str, expected, should_succeed):
        from monthly_analysis import extract_year_month
        result = extract_year_month(date_str)
        if should_succeed:
            assert result == expected
        else:
            assert result is None


class TestParseMinutes:
    """A4-A8: 工数パースのテスト"""

    @pytest.mark.parametrize("value,expected,should_succeed", [
        # 正常系: 数値文字列
        ('5', 5.0, True),
        ('130', 130.0, True),
        ('60.5', 60.5, True),
        # 正常系: 数値型
        (100, 100.0, True),
        (50.5, 50.5, True),
        # 異常系: 空・None
        ('', None, False),
        (None, None, False),
        # 異常系: 数値でない
        ('↑', None, False),
        ('↓', None, False),
        ('131.5h', None, False),
        ('６個／８０分', None, False),
    ])
    def test_parse_minutes(self, value, expected, should_succeed):
        from monthly_analysis import parse_minutes
        result = parse_minutes(value)
        if should_succeed:
            assert result == expected
        else:
            assert result is None


# =============================================================================
# B. 月別サマリーのテスト
# =============================================================================

class TestMonthlyAnalyzerBasic:
    """B1-B8: 月別サマリーの基本テスト"""

    @pytest.fixture
    def analyzer(self):
        from monthly_analysis import MonthlyAnalyzer
        return MonthlyAnalyzer(SAMPLE_RECORDS)

    def test_B1_月の一覧取得(self, analyzer):
        months = analyzer.get_months()
        assert months == ['2024-09', '2024-10', '2024-11']

    def test_B2_月別件数(self, analyzer):
        summary = analyzer.get_monthly_summary()
        assert summary['2024-09']['件数'] == 4
        assert summary['2024-10']['件数'] == 2
        assert summary['2024-11']['件数'] == 2

    def test_B3_月別工数合計(self, analyzer):
        summary = analyzer.get_monthly_summary()
        # 2024-09: 5+5+20+130 = 160
        assert summary['2024-09']['総工数(分)'] == 160
        # 2024-10: 15+60 = 75
        assert summary['2024-10']['総工数(分)'] == 75
        # 2024-11: 100 (1件は空欄)
        assert summary['2024-11']['総工数(分)'] == 100

    def test_B4_月別部品種類数(self, analyzer):
        summary = analyzer.get_monthly_summary()
        assert summary['2024-09']['部品種類数'] == 4

    def test_B5_月別作業者数(self, analyzer):
        summary = analyzer.get_monthly_summary()
        assert summary['2024-09']['作業者数'] == 2
        assert summary['2024-10']['作業者数'] == 2

    def test_B6_月別カテゴリ別件数(self, analyzer):
        summary = analyzer.get_monthly_summary()
        assert summary['2024-09']['カテゴリ別']['メカ'] == 3
        assert summary['2024-09']['カテゴリ別']['エレキ'] == 1

    def test_B7_パレート指標(self, analyzer):
        summary = analyzer.get_monthly_summary()
        # 2024-09: 4品名 → 上位20% = 1品名（切り上げ）
        # U8 Lite Armset CW (130分) が最大 → 130/160 = 81.25%
        pareto = summary['2024-09']['パレート指標(%)']
        assert 80 <= pareto <= 82

    def test_B8_工数未記入の処理(self, analyzer):
        summary = analyzer.get_monthly_summary()
        # 2024-11: 2件中1件が空欄
        assert summary['2024-11']['件数'] == 2
        assert summary['2024-11']['総工数(分)'] == 100
        assert summary['2024-11']['工数記録あり'] == 1


class TestMonthlyAnalyzerAdvanced:
    """B9-B11: 月別サマリーの追加テスト"""

    @pytest.fixture
    def analyzer(self):
        from monthly_analysis import MonthlyAnalyzer
        return MonthlyAnalyzer(SAMPLE_RECORDS)

    def test_B9_最多担当者(self, analyzer):
        summary = analyzer.get_monthly_summary()
        # 2024-09: 西村3件、田中1件
        assert summary['2024-09']['最多担当者'] == '西村'

    def test_B10_工数トップ3部品(self, analyzer):
        summary = analyzer.get_monthly_summary()
        top3 = summary['2024-09']['工数トップ3']
        # U8 Lite Armset CW (130) > Battery mount LH (20) > 保護パッド/隙間テープ (5)
        assert top3[0] == 'U8 Lite Armset CW'
        assert top3[1] == 'Battery mount LH'
        assert len(top3) == 3

    def test_B11_カテゴリ別割合(self, analyzer):
        summary = analyzer.get_monthly_summary()
        # 2024-09: メカ3件(75%), エレキ1件(25%)
        ratio = summary['2024-09']['カテゴリ別割合(%)']
        assert ratio['メカ'] == 75.0
        assert ratio['エレキ'] == 25.0


# =============================================================================
# C. 月別×切り口の詳細テスト
# =============================================================================

class TestMonthlyByDetail:
    """C1-C5: 月別×切り口の集計テスト"""

    @pytest.fixture
    def analyzer(self):
        from monthly_analysis import MonthlyAnalyzer
        return MonthlyAnalyzer(SAMPLE_RECORDS)

    def test_C1_monthly_by_part(self, analyzer):
        by_part = analyzer.get_monthly_by_part()
        assert by_part['2024-09']['保護パッド']['件数'] == 1
        assert by_part['2024-09']['保護パッド']['工数(分)'] == 5

    def test_C2_monthly_by_worker(self, analyzer):
        by_worker = analyzer.get_monthly_by_worker()
        # 2024-09 の西村: 3件、5+5+20 = 30分
        assert by_worker['2024-09']['西村']['件数'] == 3
        assert by_worker['2024-09']['西村']['工数(分)'] == 30

    def test_C3_monthly_by_category(self, analyzer):
        by_cat = analyzer.get_monthly_by_category()
        assert by_cat['2024-09']['メカ']['件数'] == 3
        assert by_cat['2024-09']['メカ']['工数(分)'] == 30

    def test_C4_monthly_by_inspection(self, analyzer):
        by_insp = analyzer.get_monthly_by_inspection()
        assert by_insp['2024-09']['外観']['件数'] == 3
        assert by_insp['2024-09']['機能']['件数'] == 1

    def test_C5_monthly_worker_average(self, analyzer):
        by_worker = analyzer.get_monthly_by_worker()
        # 2024-09 の西村: 30分 / 3件 = 10分
        assert by_worker['2024-09']['西村']['平均(分)'] == 10.0


# =============================================================================
# D. 統合テスト（実データ）
# =============================================================================

class TestLoadFromCSV:
    """D1-D2: CSV読み込みの統合テスト"""

    def test_D1_CSV読み込み(self):
        from monthly_analysis import load_all_raw_csv
        records = load_all_raw_csv()
        assert len(records) == 574

    def test_D2_全月の工数合計(self):
        from monthly_analysis import load_all_raw_csv, MonthlyAnalyzer
        records = load_all_raw_csv()
        analyzer = MonthlyAnalyzer(records)
        summary = analyzer.get_monthly_summary()

        total_minutes = sum(s['総工数(分)'] for s in summary.values())
        # 約530.5時間 = 31830分（誤差許容）
        assert 31000 <= total_minutes <= 32500


# =============================================================================
# E. 出力・境界値テスト
# =============================================================================

class TestEdgeCases:
    """E1-E3: 出力形式・境界値テスト"""

    def test_E2_入荷日不明の処理(self):
        from monthly_analysis import MonthlyAnalyzer
        analyzer = MonthlyAnalyzer(SAMPLE_WITH_UNKNOWN_DATE)
        months = analyzer.get_months()
        # '不明' が月として含まれる
        assert '不明' in months

    def test_E3_1件だけの月(self):
        from monthly_analysis import MonthlyAnalyzer
        # 2024-11は工数記録ありが1件だけ
        analyzer = MonthlyAnalyzer(SAMPLE_RECORDS)
        summary = analyzer.get_monthly_summary()
        assert summary['2024-11']['工数記録あり'] == 1
        assert summary['2024-11']['総工数(分)'] == 100


if __name__ == '__main__':
    pytest.main([__file__, '-v'])
