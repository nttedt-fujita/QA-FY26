//! 検査関連の型定義
//!
//! InspectionItem: 検査項目定義
//! InspectionResult: 検査結果

use std::time::Duration;

/// 検査項目の種類
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemType {
    /// 通信疎通確認
    Connectivity,
    /// FWバージョン（MON-VER）
    FwVersion,
    /// シリアル番号（SEC-UNIQID）
    SerialNumber,
    /// 出力レート（CFG-RATE）
    OutputRate,
    /// ポート設定（CFG-PRT）
    PortConfig,
}

/// 期待値
#[derive(Debug, Clone, PartialEq)]
pub enum ExpectedValue {
    /// 期待値なし（値取得成功でPass）
    None,
    /// 文字列期待値
    String(String),
    /// 整数期待値
    Integer(i64),
}

/// 検査項目定義
#[derive(Debug, Clone)]
pub struct InspectionItem {
    /// 項目タイプ
    pub item_type: ItemType,
    /// タイムアウト
    pub timeout: Duration,
    /// 期待値（Noneなら値取得成功でPass）
    pub expected: ExpectedValue,
}

impl InspectionItem {
    /// 新しい検査項目を作成
    pub fn new(item_type: ItemType) -> Self {
        Self {
            item_type,
            timeout: Duration::from_secs(1),
            expected: ExpectedValue::None,
        }
    }

    /// タイムアウトを設定
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// 期待値を設定
    pub fn with_expected(mut self, expected: ExpectedValue) -> Self {
        self.expected = expected;
        self
    }
}

/// 検査結果の判定
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// 合格
    Pass,
    /// 不合格（期待値と不一致）
    Fail,
    /// エラー（タイムアウト、パースエラー等）
    Error(String),
}

/// 検査結果
#[derive(Debug, Clone)]
pub struct InspectionResult {
    /// 検査項目
    pub item_type: ItemType,
    /// 判定
    pub verdict: Verdict,
    /// 実測値（取得できた場合）
    pub actual_value: Option<String>,
    /// 期待値
    pub expected_value: ExpectedValue,
}

impl InspectionResult {
    /// 新しい検査結果を作成
    pub fn new(item_type: ItemType, verdict: Verdict) -> Self {
        Self {
            item_type,
            verdict,
            actual_value: None,
            expected_value: ExpectedValue::None,
        }
    }

    /// 実測値を設定
    pub fn with_actual(mut self, actual: String) -> Self {
        self.actual_value = Some(actual);
        self
    }

    /// 期待値を設定
    pub fn with_expected(mut self, expected: ExpectedValue) -> Self {
        self.expected_value = expected;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===========================================
    // D1-D3: InspectionItem/Result構造体テスト
    // ===========================================

    /// D1: タイムアウト設定できる
    #[test]
    fn test_d1_inspection_item_timeout() {
        // 正常系: タイムアウトを500msに設定
        let item = InspectionItem::new(ItemType::Connectivity)
            .with_timeout(Duration::from_millis(500));

        assert_eq!(item.timeout, Duration::from_millis(500));
    }

    /// D1: デフォルトタイムアウトは1秒
    #[test]
    fn test_d1_default_timeout() {
        let item = InspectionItem::new(ItemType::Connectivity);
        assert_eq!(item.timeout, Duration::from_secs(1));
    }

    /// D2: 期待値設定できる（Optional）
    #[test]
    fn test_d2_expected_value_string() {
        // 正常系: 文字列期待値
        let item = InspectionItem::new(ItemType::FwVersion)
            .with_expected(ExpectedValue::String("HPG 1.32".to_string()));

        assert_eq!(
            item.expected,
            ExpectedValue::String("HPG 1.32".to_string())
        );
    }

    /// D2: 期待値なし（値取得成功でPass）
    #[test]
    fn test_d2_expected_value_none() {
        let item = InspectionItem::new(ItemType::SerialNumber);

        assert_eq!(item.expected, ExpectedValue::None);
    }

    /// D2: 整数期待値
    #[test]
    fn test_d2_expected_value_integer() {
        // 出力レート: 100ms = 10Hz
        let item = InspectionItem::new(ItemType::OutputRate)
            .with_expected(ExpectedValue::Integer(100));

        assert_eq!(item.expected, ExpectedValue::Integer(100));
    }

    /// D3: InspectionResultに必須フィールドが含まれる
    #[test]
    fn test_d3_inspection_result_required_fields() {
        let result = InspectionResult::new(ItemType::Connectivity, Verdict::Pass)
            .with_actual("OK".to_string())
            .with_expected(ExpectedValue::None);

        // 必須フィールドの確認
        assert_eq!(result.item_type, ItemType::Connectivity);
        assert_eq!(result.verdict, Verdict::Pass);
        assert_eq!(result.actual_value, Some("OK".to_string()));
        assert_eq!(result.expected_value, ExpectedValue::None);
    }

    /// D3: Verdictの3種類
    #[test]
    fn test_d3_verdict_variants() {
        // Pass
        let pass = InspectionResult::new(ItemType::FwVersion, Verdict::Pass);
        assert_eq!(pass.verdict, Verdict::Pass);

        // Fail
        let fail = InspectionResult::new(ItemType::FwVersion, Verdict::Fail);
        assert_eq!(fail.verdict, Verdict::Fail);

        // Error
        let error = InspectionResult::new(
            ItemType::FwVersion,
            Verdict::Error("Timeout".to_string())
        );
        assert!(matches!(error.verdict, Verdict::Error(_)));
    }

    /// D3: 全ItemTypeが定義されている
    #[test]
    fn test_d3_all_item_types() {
        // 5項目すべてが定義されていることを確認
        let types = vec![
            ItemType::Connectivity,
            ItemType::FwVersion,
            ItemType::SerialNumber,
            ItemType::OutputRate,
            ItemType::PortConfig,
        ];

        assert_eq!(types.len(), 5);
    }
}
