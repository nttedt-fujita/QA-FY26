"""品名の名寄せテスト"""

import pytest
from pathlib import Path

# conftest.pyでパス設定済み
from name_normalizer import NameNormalizer

# プロジェクトルート
PROJECT_ROOT = Path(__file__).parent.parent.parent.parent


@pytest.fixture
def normalizer():
    """品名用の名寄せNormalizerを返す"""
    rules_path = PROJECT_ROOT / "tools" / "incoming_inspection" / "mapping" / "品名_名寄せルール.csv"
    return NameNormalizer(rules_path)


@pytest.mark.parametrize("input_name,expected,should_convert", [
    # 正常系: 全角スペース統一
    ("Arm grommet", "Arm　grommet", True),
    ("Liquid Tank 9L", "Liquid　Tank　9L", True),

    # 正常系: 余分なスペース削除
    ("Battery lock  handle Assy", "Battery lock handle Assy", True),

    # 正常系: 先頭大文字統一
    ("boom mount", "Boom mount", True),
    ("center beam", "Center beam", True),

    # 正常系: 大文字統一
    ("Condition LED base", "Condition LED Base", True),
    ("Upper plate", "Upper Plate", True),
    ("Mecha Tray", "Mecha tray", True),
    ("Slide mount", "Slide Mount", True),

    # 正常系: 全角/半角括弧統一
    ("Harness(FC-MC)", "Harness（FC-MC）", True),
    ("Harness(PCM-MC)", "Harness（PCM-MC）", True),
    ("Harness（PCM-FC）", "Harness(PCM-FC)", True),
    ("Harness（PCM-PDB）", "Harness(PCM-PDB)", True),
    ("Lower Plate(AC102)", "Lower Plate（AC102）", True),

    # 正常系: 日本語の括弧統一
    ("サーボホルダ（AC20）", "サーボホルダ(AC20)", True),
    ("タンクキャップ（穴追加工）", "タンクキャップ(穴追加工)", True),

    # 正常系: 日本語スペース統一
    ("サーボ　AC20-05H-A01", "サーボ AC20-05H-A01", True),

    # 変換不要（ルールにない）
    ("U8 Lite Armset CCW（ver2023)", "U8 Lite Armset CCW（ver2023)", False),
    ("Skid(Assy)", "Skid(Assy)", False),
    ("フライトコントローラー(NTT e-DT製)", "フライトコントローラー(NTT e-DT製)", False),
])
def test_product_name_normalize(normalizer, input_name, expected, should_convert):
    """品名の名寄せテスト"""
    result = normalizer.normalize(input_name)
    assert result == expected

    # 変換されたかどうかの確認
    if should_convert:
        assert result != input_name, f"'{input_name}' should be converted"
    else:
        assert result == input_name, f"'{input_name}' should not be converted"


def test_rule_count(normalizer):
    """ルール数の確認"""
    assert normalizer.get_rule_count() == 18


def test_stats(normalizer):
    """統計情報のテスト"""
    # 変換対象
    normalizer.normalize("Arm grommet")
    normalizer.normalize("boom mount")
    # 変換対象外
    normalizer.normalize("Skid(Assy)")

    stats = normalizer.get_stats()
    assert stats['total_processed'] == 3
    assert stats['total_converted'] == 2
    assert stats['conversion_rate'] == 66.7


def test_normalize_list(normalizer):
    """リスト一括変換のテスト"""
    input_list = ["Arm grommet", "boom mount", "Skid(Assy)"]
    expected = ["Arm　grommet", "Boom mount", "Skid(Assy)"]

    result = normalizer.normalize_list(input_list)
    assert result == expected
