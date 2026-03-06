"""データクレンジングモジュール

日付異常やデータ品質の問題を修正する。

修正ルール:
- 2026-11/12 の日付 → 2025-11/12 に修正（入力時の年誤りとして処理）
- 入荷日が空欄の場合、検査完了日を入荷日として使用（フォールバック）
  方針決定: Session 23（2026-03-06）
  根拠: 入荷日→検査完了日の差は中央値5日、70%が7日以内のため月別集計に影響小
"""


class DateCleaner:
    """日付クレンジング処理クラス

    未来日付の誤りを検出し、修正する。
    """

    def __init__(self, reference_date="2026-03-06"):
        """
        Args:
            reference_date: 基準日（YYYY-MM-DD形式）。これ以降の日付を「未来」と判断
        """
        self.reference_date = reference_date
        self._stats = {
            'total_processed': 0,
            'total_fixed': 0,
        }

    def fix_future_date(self, date_str):
        """未来日付を修正する

        2026-11/12 の日付は 2025年の誤入力として修正する。
        （元データ分析で確認済みのパターン）

        Args:
            date_str: 日付文字列（YYYY-MM-DD形式）

        Returns:
            修正後の日付文字列
        """
        if date_str is None:
            return None

        if date_str == '':
            return ''

        self._stats['total_processed'] += 1

        # 2026-11 または 2026-12 で始まる日付を修正
        if date_str.startswith('2026-11') or date_str.startswith('2026-12'):
            self._stats['total_fixed'] += 1
            # 年を2025に置換
            return '2025' + date_str[4:]

        return date_str

    def get_stats(self):
        """統計情報を取得

        Returns:
            統計情報の辞書
        """
        return {
            'total_processed': self._stats['total_processed'],
            'total_fixed': self._stats['total_fixed'],
        }

    def reset_stats(self):
        """統計情報をリセット"""
        self._stats = {
            'total_processed': 0,
            'total_fixed': 0,
        }


class DataCleaner:
    """統合データクレンジング処理クラス

    レコード単位でクレンジング処理を適用する。
    """

    # 日付フィールドの列名
    DATE_FIELDS = ['入荷日', '検査完了日']

    def __init__(self, reference_date="2026-03-06"):
        """
        Args:
            reference_date: 基準日（YYYY-MM-DD形式）
        """
        self.date_cleaner = DateCleaner(reference_date)

    def clean_record(self, record):
        """レコードをクレンジングする

        処理順序:
        1. 入荷日フォールバック: 入荷日が空欄なら検査完了日を使用
        2. 未来日付修正: 2026-11/12 → 2025-11/12

        Args:
            record: 辞書形式のレコード

        Returns:
            クレンジング後のレコード（新しい辞書）
        """
        cleaned = record.copy()

        # 1. 入荷日フォールバック: 入荷日が空欄なら検査完了日を使用
        arrival = cleaned.get('入荷日')
        if self._is_empty(arrival):
            complete = cleaned.get('検査完了日')
            if not self._is_empty(complete):
                cleaned['入荷日'] = complete

        # 2. 日付フィールドの未来日付修正
        for field in self.DATE_FIELDS:
            if field in cleaned:
                cleaned[field] = self.date_cleaner.fix_future_date(cleaned[field])

        return cleaned

    def _is_empty(self, value):
        """値が空かどうか判定"""
        if value is None:
            return True
        if isinstance(value, str) and value.strip() == '':
            return True
        return False

    def clean_records(self, records):
        """複数レコードを一括クレンジングする

        Args:
            records: レコードのリスト

        Returns:
            クレンジング後のレコードリスト
        """
        return [self.clean_record(r) for r in records]

    def get_stats(self):
        """統計情報を取得"""
        return self.date_cleaner.get_stats()
