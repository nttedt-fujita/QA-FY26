"""データクレンジングモジュールのテスト

日付異常の修正をテストする。
"""

import pytest


class TestDateCleaner:
    """日付クレンジングのテスト"""

    @pytest.mark.parametrize("input_date,expected,should_succeed", [
        # 正常系: 修正不要な日付はそのまま
        ("2025-01-15", "2025-01-15", True),
        ("2024-12-01", "2024-12-01", True),
        ("2025-11-25", "2025-11-25", True),
        # 正常系: 未来日付（2026-11/12）を2025年に修正
        ("2026-11-25", "2025-11-25", True),
        ("2026-12-12", "2025-12-12", True),
        ("2026-12-24", "2025-12-24", True),
        ("2026-12-25", "2025-12-25", True),
        ("2026-12-26", "2025-12-26", True),
        # 正常系: 2026年の他の月はそのまま（現在2026-03なので1-3月は未来ではない）
        ("2026-01-15", "2026-01-15", True),
        ("2026-02-28", "2026-02-28", True),
        ("2026-03-01", "2026-03-01", True),
        # 正常系: 空欄はそのまま
        ("", "", True),
        (None, None, True),
        # 境界値: 2026-10は修正対象外（現時点では「未来」だが誤入力パターンから外れる）
        # → 誤入力パターンは2026-11/12のみ（元データで確認済み）
        ("2026-10-01", "2026-10-01", True),
    ])
    def test_fix_future_date(self, input_date, expected, should_succeed):
        """未来日付を修正する"""
        from tools.incoming_inspection.data_cleaner import DateCleaner

        cleaner = DateCleaner(reference_date="2026-03-06")
        result = cleaner.fix_future_date(input_date)

        if should_succeed:
            assert result == expected
        else:
            # 現状は失敗ケースなし
            pass

    def test_fix_future_date_stats(self):
        """修正件数の統計を取得できる"""
        from tools.incoming_inspection.data_cleaner import DateCleaner

        cleaner = DateCleaner(reference_date="2026-03-06")

        # 修正対象と非対象を混ぜてテスト
        cleaner.fix_future_date("2026-11-25")  # 修正される
        cleaner.fix_future_date("2025-10-01")  # 修正されない
        cleaner.fix_future_date("2026-12-12")  # 修正される

        stats = cleaner.get_stats()
        assert stats['total_processed'] == 3
        assert stats['total_fixed'] == 2


class TestDataCleaner:
    """統合データクレンジングのテスト"""

    def test_clean_record_fixes_future_dates(self):
        """レコードの日付フィールドを一括修正"""
        from tools.incoming_inspection.data_cleaner import DataCleaner

        cleaner = DataCleaner(reference_date="2026-03-06")

        record = {
            'カテゴリ': 'Api',
            '入荷日': '2026-11-25',
            '検査完了日': '2026-12-04',
            '品名': 'Arm grommet',
        }

        cleaned = cleaner.clean_record(record)

        assert cleaned['入荷日'] == '2025-11-25'
        assert cleaned['検査完了日'] == '2025-12-04'
        # 他のフィールドは変更なし
        assert cleaned['カテゴリ'] == 'Api'
        assert cleaned['品名'] == 'Arm grommet'

    def test_clean_record_preserves_valid_dates(self):
        """正常な日付は変更しない"""
        from tools.incoming_inspection.data_cleaner import DataCleaner

        cleaner = DataCleaner(reference_date="2026-03-06")

        record = {
            '入荷日': '2025-10-23',
            '検査完了日': '2025-10-25',
        }

        cleaned = cleaner.clean_record(record)

        assert cleaned['入荷日'] == '2025-10-23'
        assert cleaned['検査完了日'] == '2025-10-25'

    def test_clean_records_batch(self):
        """複数レコードを一括処理"""
        from tools.incoming_inspection.data_cleaner import DataCleaner

        cleaner = DataCleaner(reference_date="2026-03-06")

        records = [
            {'入荷日': '2026-11-25', '検査完了日': '2026-12-04'},
            {'入荷日': '2025-01-15', '検査完了日': '2025-01-20'},
            {'入荷日': '2026-12-12', '検査完了日': '2026-12-24'},
        ]

        cleaned = cleaner.clean_records(records)

        assert len(cleaned) == 3
        assert cleaned[0]['入荷日'] == '2025-11-25'
        assert cleaned[1]['入荷日'] == '2025-01-15'  # 変更なし
        assert cleaned[2]['入荷日'] == '2025-12-12'


class TestArrivalDateFallback:
    """入荷日フォールバックのテスト

    入荷日が空欄の場合、検査完了日を入荷日として使用する。
    方針決定: Session 23（2026-03-06）
    根拠: 入荷日→検査完了日の差は中央値5日、70%が7日以内のため月別集計に影響小
    """

    @pytest.mark.parametrize(
        "arrival,complete,expected_arrival,should_succeed",
        [
            # 正常系: 入荷日空欄＋検査完了日あり → 検査完了日を使用
            ("", "2025-10-27", "2025-10-27", True),
            (None, "2025-10-27", "2025-10-27", True),
            # 正常系: 両方空欄 → 空欄のまま
            ("", "", "", True),
            (None, None, None, True),
            ("", None, "", True),
            # 正常系: 入荷日あり → 変更なし（検査完了日は使わない）
            ("2025-10-20", "2025-10-27", "2025-10-20", True),
            # 複合: フォールバック＋未来日付修正
            ("", "2026-11-25", "2025-11-25", True),
            ("", "2026-12-12", "2025-12-12", True),
        ],
    )
    def test_arrival_date_fallback(
        self, arrival, complete, expected_arrival, should_succeed
    ):
        """入荷日が空欄なら検査完了日をフォールバックとして使用"""
        from tools.incoming_inspection.data_cleaner import DataCleaner

        cleaner = DataCleaner(reference_date="2026-03-06")

        record = {
            '入荷日': arrival,
            '検査完了日': complete,
        }

        cleaned = cleaner.clean_record(record)

        if should_succeed:
            assert cleaned['入荷日'] == expected_arrival
