"""受入検査データ分析ツール

このパッケージには以下のモジュールが含まれます:
- extract_csv: Excel→CSV変換
- name_normalizer: 表記揺れの名寄せ
- monthly_analysis: 月別分析

使用例:
    from tools.incoming_inspection import NameNormalizer
    normalizer = NameNormalizer("mapping/検査内容_名寄せルール.csv")
"""

from .name_normalizer import NameNormalizer

__all__ = ["NameNormalizer"]
