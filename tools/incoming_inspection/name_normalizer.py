"""名寄せモジュール

表記揺れのあるデータを正規化する。
ルールはCSVファイルで管理し、変換履歴を追跡可能にする。
"""

import csv
from pathlib import Path


class NameNormalizer:
    """名寄せ処理クラス

    CSVファイルから変換ルールを読み込み、
    入力値を正規化された値に変換する。
    """

    def __init__(self, rules_csv_path):
        """
        Args:
            rules_csv_path: 変換ルールCSVファイルのパス
        """
        self.rules_csv_path = Path(rules_csv_path)
        self._rules = {}  # {変換前: 変換後}
        self._stats = {
            'total_processed': 0,
            'total_converted': 0,
        }
        self._load_rules()

    def _load_rules(self):
        """CSVからルールを読み込む"""
        if not self.rules_csv_path.exists():
            return

        with open(self.rules_csv_path, 'r', encoding='utf-8-sig') as f:
            reader = csv.DictReader(f)
            for row in reader:
                変換前 = row.get('変換前', '')
                変換後 = row.get('変換後', '')
                if 変換後:  # 変換後が空でないルールのみ登録
                    self._rules[変換前] = 変換後

    def get_rule_count(self):
        """登録されているルール数を返す"""
        return len(self._rules)

    def normalize(self, value):
        """値を名寄せする

        Args:
            value: 変換対象の値

        Returns:
            変換後の値（ルールにない場合はそのまま）
        """
        self._stats['total_processed'] += 1

        if value in self._rules:
            self._stats['total_converted'] += 1
            return self._rules[value]

        return value

    def normalize_list(self, values):
        """リストを一括で名寄せする

        Args:
            values: 変換対象のリスト

        Returns:
            変換後のリスト
        """
        return [self.normalize(v) for v in values]

    def get_stats(self):
        """統計情報を取得

        Returns:
            統計情報の辞書
        """
        rate = 0.0
        if self._stats['total_processed'] > 0:
            rate = round(
                self._stats['total_converted'] / self._stats['total_processed'] * 100,
                1
            )

        return {
            'total_processed': self._stats['total_processed'],
            'total_converted': self._stats['total_converted'],
            'conversion_rate': rate,
        }

    def reset_stats(self):
        """統計情報をリセット"""
        self._stats = {
            'total_processed': 0,
            'total_converted': 0,
        }
