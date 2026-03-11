//! 型変換ユーティリティ
//!
//! InspectionEngine側の型とRepository側の型を相互変換

use crate::inspection::types::{
    ExpectedValue as EngineExpectedValue, InspectionResult as EngineResult, ItemType,
    Verdict as EngineVerdict,
};
use crate::repository::{InspectionItemName, InspectionItemResult, Verdict as RepoVerdict};

/// ItemType → InspectionItemName の変換
pub fn item_type_to_name(item_type: &ItemType) -> InspectionItemName {
    match item_type {
        ItemType::Connectivity => InspectionItemName::Communication,
        ItemType::FwVersion => InspectionItemName::Fw,
        ItemType::SerialNumber => InspectionItemName::Serial,
        ItemType::OutputRate => InspectionItemName::Rate,
        ItemType::PortConfig => InspectionItemName::Port,
    }
}

/// InspectionItemName → ItemType の変換
pub fn name_to_item_type(name: &InspectionItemName) -> ItemType {
    match name {
        InspectionItemName::Communication => ItemType::Connectivity,
        InspectionItemName::Fw => ItemType::FwVersion,
        InspectionItemName::Serial => ItemType::SerialNumber,
        InspectionItemName::Rate => ItemType::OutputRate,
        InspectionItemName::Port => ItemType::PortConfig,
    }
}

/// EngineVerdict → RepoVerdict の変換
///
/// FWバージョンは期待値なしで記録するのでRecordedに変換
pub fn engine_verdict_to_repo(
    verdict: &EngineVerdict,
    item_type: &ItemType,
    expected: &EngineExpectedValue,
) -> RepoVerdict {
    // FWバージョンは期待値なしならRecorded
    if matches!(item_type, ItemType::FwVersion) && matches!(expected, EngineExpectedValue::None) {
        return RepoVerdict::Recorded;
    }

    // シリアル番号も期待値なしならRecorded
    if matches!(item_type, ItemType::SerialNumber) && matches!(expected, EngineExpectedValue::None)
    {
        return RepoVerdict::Recorded;
    }

    match verdict {
        EngineVerdict::Pass => RepoVerdict::Pass,
        EngineVerdict::Fail => RepoVerdict::Fail,
        EngineVerdict::Error(_) => RepoVerdict::Error,
    }
}

/// EngineResult → InspectionItemResult の変換
///
/// inspection_idは呼び出し元で指定
pub fn engine_result_to_repo(result: &EngineResult, inspection_id: i64) -> InspectionItemResult {
    let item_name = item_type_to_name(&result.item_type);
    let verdict = engine_verdict_to_repo(&result.verdict, &result.item_type, &result.expected_value);

    let mut repo_result = InspectionItemResult::new(inspection_id, item_name, verdict);

    // 実測値
    if let Some(actual) = &result.actual_value {
        repo_result = repo_result.with_actual(actual);
    }

    // 期待値
    match &result.expected_value {
        EngineExpectedValue::String(s) => {
            repo_result = repo_result.with_expected(s);
        }
        EngineExpectedValue::Integer(i) => {
            repo_result = repo_result.with_expected(&i.to_string());
        }
        EngineExpectedValue::None => {}
    }

    repo_result
}

/// 総合判定を計算
///
/// - 全てPassまたはRecorded → Pass
/// - 1つでもError → Error
/// - それ以外（Fail含む） → Fail
pub fn calculate_overall_result(verdicts: &[RepoVerdict]) -> &'static str {
    if verdicts.is_empty() {
        return "Error";
    }

    let has_error = verdicts.iter().any(|v| matches!(v, RepoVerdict::Error));
    let has_fail = verdicts.iter().any(|v| matches!(v, RepoVerdict::Fail));

    if has_error {
        "Error"
    } else if has_fail {
        "Fail"
    } else {
        "Pass"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===========================================
    // T1: ItemType ↔ InspectionItemName 変換
    // ===========================================

    #[test]
    fn test_item_type_to_name_all_types() {
        assert_eq!(
            item_type_to_name(&ItemType::Connectivity),
            InspectionItemName::Communication
        );
        assert_eq!(
            item_type_to_name(&ItemType::FwVersion),
            InspectionItemName::Fw
        );
        assert_eq!(
            item_type_to_name(&ItemType::SerialNumber),
            InspectionItemName::Serial
        );
        assert_eq!(
            item_type_to_name(&ItemType::OutputRate),
            InspectionItemName::Rate
        );
        assert_eq!(
            item_type_to_name(&ItemType::PortConfig),
            InspectionItemName::Port
        );
    }

    #[test]
    fn test_name_to_item_type_all_types() {
        assert_eq!(
            name_to_item_type(&InspectionItemName::Communication),
            ItemType::Connectivity
        );
        assert_eq!(
            name_to_item_type(&InspectionItemName::Fw),
            ItemType::FwVersion
        );
        assert_eq!(
            name_to_item_type(&InspectionItemName::Serial),
            ItemType::SerialNumber
        );
        assert_eq!(
            name_to_item_type(&InspectionItemName::Rate),
            ItemType::OutputRate
        );
        assert_eq!(
            name_to_item_type(&InspectionItemName::Port),
            ItemType::PortConfig
        );
    }

    // ===========================================
    // T2: Verdict変換
    // ===========================================

    #[test]
    fn test_verdict_pass_to_pass() {
        let verdict = engine_verdict_to_repo(
            &EngineVerdict::Pass,
            &ItemType::Connectivity,
            &EngineExpectedValue::None,
        );
        assert_eq!(verdict, RepoVerdict::Pass);
    }

    #[test]
    fn test_verdict_fail_to_fail() {
        let verdict = engine_verdict_to_repo(
            &EngineVerdict::Fail,
            &ItemType::OutputRate,
            &EngineExpectedValue::Integer(100),
        );
        assert_eq!(verdict, RepoVerdict::Fail);
    }

    #[test]
    fn test_verdict_error_to_error() {
        let verdict = engine_verdict_to_repo(
            &EngineVerdict::Error("Timeout".to_string()),
            &ItemType::Connectivity,
            &EngineExpectedValue::None,
        );
        assert_eq!(verdict, RepoVerdict::Error);
    }

    #[test]
    fn test_fw_version_no_expected_to_recorded() {
        // FWバージョンで期待値なしの場合はRecorded
        let verdict = engine_verdict_to_repo(
            &EngineVerdict::Pass,
            &ItemType::FwVersion,
            &EngineExpectedValue::None,
        );
        assert_eq!(verdict, RepoVerdict::Recorded);
    }

    #[test]
    fn test_serial_no_expected_to_recorded() {
        // シリアル番号で期待値なしの場合はRecorded
        let verdict = engine_verdict_to_repo(
            &EngineVerdict::Pass,
            &ItemType::SerialNumber,
            &EngineExpectedValue::None,
        );
        assert_eq!(verdict, RepoVerdict::Recorded);
    }

    // ===========================================
    // T3: EngineResult → RepoResult 変換
    // ===========================================

    #[test]
    fn test_engine_result_to_repo_basic() {
        let engine_result = EngineResult::new(ItemType::Connectivity, EngineVerdict::Pass)
            .with_actual("OK".to_string());

        let repo_result = engine_result_to_repo(&engine_result, 1);

        assert_eq!(repo_result.inspection_id, 1);
        assert_eq!(repo_result.item_name, InspectionItemName::Communication);
        assert_eq!(repo_result.verdict, RepoVerdict::Pass);
        assert_eq!(repo_result.actual_value, Some("OK".to_string()));
    }

    #[test]
    fn test_engine_result_to_repo_with_expected() {
        let engine_result = EngineResult::new(ItemType::OutputRate, EngineVerdict::Pass)
            .with_actual("100".to_string())
            .with_expected(EngineExpectedValue::Integer(100));

        let repo_result = engine_result_to_repo(&engine_result, 1);

        assert_eq!(repo_result.item_name, InspectionItemName::Rate);
        assert_eq!(repo_result.verdict, RepoVerdict::Pass);
        assert_eq!(repo_result.expected_value, Some("100".to_string()));
    }

    #[test]
    fn test_engine_result_to_repo_fw_recorded() {
        let engine_result = EngineResult::new(ItemType::FwVersion, EngineVerdict::Pass)
            .with_actual("HPG 1.32".to_string());

        let repo_result = engine_result_to_repo(&engine_result, 1);

        assert_eq!(repo_result.item_name, InspectionItemName::Fw);
        assert_eq!(repo_result.verdict, RepoVerdict::Recorded);
    }

    // ===========================================
    // T4: 総合判定計算
    // ===========================================

    #[test]
    fn test_overall_result_all_pass() {
        let verdicts = vec![
            RepoVerdict::Pass,
            RepoVerdict::Pass,
            RepoVerdict::Recorded,
        ];
        assert_eq!(calculate_overall_result(&verdicts), "Pass");
    }

    #[test]
    fn test_overall_result_has_fail() {
        let verdicts = vec![
            RepoVerdict::Pass,
            RepoVerdict::Fail,
            RepoVerdict::Pass,
        ];
        assert_eq!(calculate_overall_result(&verdicts), "Fail");
    }

    #[test]
    fn test_overall_result_has_error() {
        let verdicts = vec![
            RepoVerdict::Pass,
            RepoVerdict::Error,
            RepoVerdict::Pass,
        ];
        assert_eq!(calculate_overall_result(&verdicts), "Error");
    }

    #[test]
    fn test_overall_result_empty() {
        let verdicts: Vec<RepoVerdict> = vec![];
        assert_eq!(calculate_overall_result(&verdicts), "Error");
    }
}
