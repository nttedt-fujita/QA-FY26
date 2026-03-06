"""名寄せモジュールのテスト"""

import pytest
from pathlib import Path
from name_normalizer import NameNormalizer


class TestNameNormalizerLoad:
    """NameNormalizerのルール読み込みテスト"""

    def test_load_rules_from_csv(self, tmp_path):
        """CSVファイルからルールを読み込める"""
        # CSVファイル作成
        csv_path = tmp_path / "test_rules.csv"
        csv_path.write_text(
            "変換前,変換後,適用理由,登録日\n"
            "外観のみ,外観,意味同一,2026-03-06\n"
            "外観確認,外観,意味同一,2026-03-06\n",
            encoding='utf-8-sig'
        )

        normalizer = NameNormalizer(csv_path)
        assert normalizer.get_rule_count() == 2

    def test_load_empty_csv(self, tmp_path):
        """空のCSV（ヘッダのみ）はルール0件"""
        csv_path = tmp_path / "empty.csv"
        csv_path.write_text(
            "変換前,変換後,適用理由,登録日\n",
            encoding='utf-8-sig'
        )

        normalizer = NameNormalizer(csv_path)
        assert normalizer.get_rule_count() == 0

    def test_load_nonexistent_file(self, tmp_path):
        """存在しないファイルはルール0件（エラーにならない）"""
        csv_path = tmp_path / "nonexistent.csv"
        normalizer = NameNormalizer(csv_path)
        assert normalizer.get_rule_count() == 0


class TestNameNormalizerNormalize:
    """NameNormalizerの名寄せ処理テスト"""

    @pytest.fixture
    def normalizer(self, tmp_path):
        """テスト用ルールを持つNormalizer"""
        csv_path = tmp_path / "rules.csv"
        csv_path.write_text(
            "変換前,変換後,適用理由,登録日\n"
            "外観のみ,外観,意味同一,2026-03-06\n"
            "外観確認,外観,意味同一,2026-03-06\n"
            "外観員数,外観+員数,区切りなしを+に統一,2026-03-06\n"
            ",（未記入）,空欄を統一,2026-03-06\n"
            "0,（未記入）,数字は無効値,2026-03-06\n",
            encoding='utf-8-sig'
        )
        return NameNormalizer(csv_path)

    @pytest.mark.parametrize("input_value,expected,should_succeed", [
        # 正常系: ルールに一致する場合は変換
        ("外観のみ", "外観", True),
        ("外観確認", "外観", True),
        ("外観員数", "外観+員数", True),
        # 正常系: ルールにない場合はそのまま
        ("外観", "外観", True),
        ("員数", "員数", True),
        ("動作確認", "動作確認", True),
        # 正常系: 空欄変換
        ("", "（未記入）", True),
        ("0", "（未記入）", True),
    ])
    def test_normalize(self, normalizer, input_value, expected, should_succeed):
        """名寄せ処理の正常系テスト"""
        if should_succeed:
            result = normalizer.normalize(input_value)
            assert result == expected

    def test_normalize_preserves_unknown(self, normalizer):
        """ルールにない値はそのまま返す"""
        result = normalizer.normalize("特殊検査")
        assert result == "特殊検査"


class TestNameNormalizerBatch:
    """バッチ処理テスト"""

    @pytest.fixture
    def normalizer(self, tmp_path):
        """テスト用ルールを持つNormalizer"""
        csv_path = tmp_path / "rules.csv"
        csv_path.write_text(
            "変換前,変換後,適用理由,登録日\n"
            "外観のみ,外観,意味同一,2026-03-06\n",
            encoding='utf-8-sig'
        )
        return NameNormalizer(csv_path)

    def test_normalize_list(self, normalizer):
        """リストを一括変換できる"""
        input_list = ["外観のみ", "外観", "員数"]
        result = normalizer.normalize_list(input_list)
        assert result == ["外観", "外観", "員数"]

    def test_normalize_empty_list(self, normalizer):
        """空リストは空リストを返す"""
        result = normalizer.normalize_list([])
        assert result == []


class TestNameNormalizerStats:
    """統計情報テスト"""

    @pytest.fixture
    def normalizer(self, tmp_path):
        """テスト用ルールを持つNormalizer"""
        csv_path = tmp_path / "rules.csv"
        csv_path.write_text(
            "変換前,変換後,適用理由,登録日\n"
            "外観のみ,外観,意味同一,2026-03-06\n"
            "外観確認,外観,意味同一,2026-03-06\n",
            encoding='utf-8-sig'
        )
        return NameNormalizer(csv_path)

    def test_get_stats_after_normalize(self, normalizer):
        """変換後に統計情報を取得できる"""
        # 事前に変換実行
        normalizer.normalize_list(["外観のみ", "外観のみ", "外観確認", "外観", "員数"])

        stats = normalizer.get_stats()
        assert stats['total_processed'] == 5
        assert stats['total_converted'] == 3
        assert stats['conversion_rate'] == 60.0

    def test_stats_initial_state(self, normalizer):
        """初期状態では統計は0"""
        stats = normalizer.get_stats()
        assert stats['total_processed'] == 0
        assert stats['total_converted'] == 0

    def test_reset_stats(self, normalizer):
        """統計をリセットできる"""
        normalizer.normalize_list(["外観のみ", "外観確認"])
        normalizer.reset_stats()

        stats = normalizer.get_stats()
        assert stats['total_processed'] == 0
