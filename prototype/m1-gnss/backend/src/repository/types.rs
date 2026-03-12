//! リポジトリ型定義
//!
//! 統合DB設計に基づくエンティティ定義
//! 参照: sessions/session86/gnss-unified-domain-model.md

/// ロット（入荷単位）
#[derive(Debug, Clone)]
pub struct Lot {
    pub id: Option<i64>,
    pub lot_number: String,
    /// 出力レート期待値 (ms)
    pub expected_rate_ms: Option<i32>,
    /// ポート入力プロトコル期待値
    pub expected_port_in_proto: Option<String>,
    /// ポート出力プロトコル期待値
    pub expected_port_out_proto: Option<String>,
    pub memo: Option<String>,
}

impl Lot {
    pub fn new(lot_number: String) -> Self {
        Self {
            id: None,
            lot_number,
            expected_rate_ms: None,
            expected_port_in_proto: None,
            expected_port_out_proto: None,
            memo: None,
        }
    }

    pub fn with_expected_rate(mut self, rate_ms: i32) -> Self {
        self.expected_rate_ms = Some(rate_ms);
        self
    }

    pub fn with_expected_port_proto(mut self, in_proto: &str, out_proto: &str) -> Self {
        self.expected_port_in_proto = Some(in_proto.to_string());
        self.expected_port_out_proto = Some(out_proto.to_string());
        self
    }
}

/// 装置（個別のGNSSモジュール）
#[derive(Debug, Clone)]
pub struct Device {
    pub id: Option<i64>,
    pub lot_id: Option<i64>,
    pub serial_number: String,
    pub model_number: Option<String>,
    /// FWバージョン（最後の検査で取得した値）
    pub fw_version: Option<String>,
    pub memo: Option<String>,
}

impl Device {
    pub fn new(serial_number: String) -> Self {
        Self {
            id: None,
            lot_id: None,
            serial_number,
            model_number: None,
            fw_version: None,
            memo: None,
        }
    }

    pub fn with_lot(mut self, lot_id: i64) -> Self {
        self.lot_id = Some(lot_id);
        self
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.model_number = Some(model.to_string());
        self
    }

    pub fn with_fw_version(mut self, fw: &str) -> Self {
        self.fw_version = Some(fw.to_string());
        self
    }
}

/// 屋内検査（1回の検査作業）
#[derive(Debug, Clone)]
pub struct IndoorInspection {
    pub id: Option<i64>,
    pub device_id: i64,
    /// 検査日時（ISO8601形式）
    pub inspected_at: String,
    /// 総合判定（Pass/Fail/Partial）
    pub overall_result: Option<String>,
}

impl IndoorInspection {
    pub fn new(device_id: i64, inspected_at: String) -> Self {
        Self {
            id: None,
            device_id,
            inspected_at,
            overall_result: None,
        }
    }

    pub fn with_result(mut self, result: &str) -> Self {
        self.overall_result = Some(result.to_string());
        self
    }
}

/// 検査項目名
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectionItemName {
    /// 通信疎通
    Communication,
    /// シリアル番号
    Serial,
    /// FWバージョン
    Fw,
    /// 出力レート
    Rate,
    /// ポート設定
    Port,
}

impl InspectionItemName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Communication => "communication",
            Self::Serial => "serial",
            Self::Fw => "fw",
            Self::Rate => "rate",
            Self::Port => "port",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "communication" => Some(Self::Communication),
            "serial" => Some(Self::Serial),
            "fw" => Some(Self::Fw),
            "rate" => Some(Self::Rate),
            "port" => Some(Self::Port),
            _ => None,
        }
    }
}

/// 判定（Verdict）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Pass,
    Fail,
    Error,
    /// 記録のみ（FWバージョンなど、期待値なしで記録）
    Recorded,
}

impl Verdict {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pass => "Pass",
            Self::Fail => "Fail",
            Self::Error => "Error",
            Self::Recorded => "Recorded",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Pass" => Some(Self::Pass),
            "Fail" => Some(Self::Fail),
            "Error" => Some(Self::Error),
            "Recorded" => Some(Self::Recorded),
            _ => None,
        }
    }
}

/// 検査項目結果
#[derive(Debug, Clone)]
pub struct InspectionItemResult {
    pub id: Option<i64>,
    pub inspection_id: i64,
    pub item_name: InspectionItemName,
    pub verdict: Verdict,
    pub actual_value: Option<String>,
    pub expected_value: Option<String>,
}

impl InspectionItemResult {
    pub fn new(inspection_id: i64, item_name: InspectionItemName, verdict: Verdict) -> Self {
        Self {
            id: None,
            inspection_id,
            item_name,
            verdict,
            actual_value: None,
            expected_value: None,
        }
    }

    pub fn with_actual(mut self, value: &str) -> Self {
        self.actual_value = Some(value.to_string());
        self
    }

    pub fn with_expected(mut self, value: &str) -> Self {
        self.expected_value = Some(value.to_string());
        self
    }
}

/// 屋外検査結果（DB保存対象）
///
/// ADR-008の合格基準に基づく集計結果
/// 参照: docs/missions/m1-sensor-evaluation/gnss/27-outdoor-inspection-implementation-plan.md
#[derive(Debug, Clone)]
pub struct OutdoorInspectionResult {
    pub id: Option<i64>,
    pub device_id: Option<i64>,
    pub lot_id: Option<i64>,
    /// 検査日時（ISO8601形式）
    pub inspected_at: String,
    /// 検査時間（秒）
    pub duration_sec: i32,
    /// 総サンプル数
    pub sample_count: i32,
    /// RTK FIX率 (0.0-1.0)
    pub rtk_fix_rate: f64,
    /// RTK FIX時間（ms）。FIXしなかった場合はNone
    pub rtk_fix_time_ms: Option<i32>,
    /// L2受信率（平均）(0.0-1.0)
    pub l2_reception_rate: f64,
    /// L1最小C/N0（dBHz）
    pub l1_min_cno: f64,
    /// 総合判定（合格=true）
    pub is_pass: bool,
    /// L1受信感度判定（≥30dBHz）
    pub l1_cno_pass: bool,
    /// L2受信率判定（≥50%）
    pub l2_rate_pass: bool,
    /// RTK FIX時間判定（≤30秒）
    pub rtk_fix_time_pass: bool,
    /// RTK FIX率判定（>95%）
    pub rtk_fix_rate_pass: bool,
    /// 不合格理由（JSONエンコード）
    pub failure_reasons: Option<String>,
}

impl OutdoorInspectionResult {
    pub fn new(inspected_at: String, duration_sec: i32, sample_count: i32) -> Self {
        Self {
            id: None,
            device_id: None,
            lot_id: None,
            inspected_at,
            duration_sec,
            sample_count,
            rtk_fix_rate: 0.0,
            rtk_fix_time_ms: None,
            l2_reception_rate: 0.0,
            l1_min_cno: 0.0,
            is_pass: false,
            l1_cno_pass: false,
            l2_rate_pass: false,
            rtk_fix_time_pass: false,
            rtk_fix_rate_pass: false,
            failure_reasons: None,
        }
    }

    pub fn with_device(mut self, device_id: i64) -> Self {
        self.device_id = Some(device_id);
        self
    }

    pub fn with_lot(mut self, lot_id: i64) -> Self {
        self.lot_id = Some(lot_id);
        self
    }

    pub fn with_metrics(
        mut self,
        rtk_fix_rate: f64,
        rtk_fix_time_ms: Option<i32>,
        l2_reception_rate: f64,
        l1_min_cno: f64,
    ) -> Self {
        self.rtk_fix_rate = rtk_fix_rate;
        self.rtk_fix_time_ms = rtk_fix_time_ms;
        self.l2_reception_rate = l2_reception_rate;
        self.l1_min_cno = l1_min_cno;
        self
    }

    pub fn with_judgment(
        mut self,
        is_pass: bool,
        l1_cno_pass: bool,
        l2_rate_pass: bool,
        rtk_fix_time_pass: bool,
        rtk_fix_rate_pass: bool,
        failure_reasons: Option<String>,
    ) -> Self {
        self.is_pass = is_pass;
        self.l1_cno_pass = l1_cno_pass;
        self.l2_rate_pass = l2_rate_pass;
        self.rtk_fix_time_pass = rtk_fix_time_pass;
        self.rtk_fix_rate_pass = rtk_fix_rate_pass;
        self.failure_reasons = failure_reasons;
        self
    }
}

/// リポジトリエラー
#[derive(Debug)]
pub enum RepositoryError {
    /// DB接続エラー
    Connection(String),
    /// SQLエラー
    Sql(String),
    /// データ不正
    InvalidData(String),
    /// 見つからない
    NotFound(String),
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::Connection(msg) => write!(f, "Connection error: {}", msg),
            RepositoryError::Sql(msg) => write!(f, "SQL error: {}", msg),
            RepositoryError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            RepositoryError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {}

#[cfg(test)]
mod tests {
    use super::*;

    // ===========================================
    // L1-L3: Lot構造体テスト
    // ===========================================

    #[test]
    fn test_l1_create_lot() {
        let lot = Lot::new("LOT-2026-001".to_string());

        assert_eq!(lot.lot_number, "LOT-2026-001");
        assert!(lot.id.is_none());
        assert!(lot.expected_rate_ms.is_none());
    }

    #[test]
    fn test_l2_lot_with_expected_rate() {
        let lot = Lot::new("LOT-2026-001".to_string())
            .with_expected_rate(100);

        assert_eq!(lot.expected_rate_ms, Some(100));
    }

    #[test]
    fn test_l3_lot_with_port_proto() {
        let lot = Lot::new("LOT-2026-001".to_string())
            .with_expected_port_proto("UBX+NMEA", "UBX+NMEA");

        assert_eq!(lot.expected_port_in_proto, Some("UBX+NMEA".to_string()));
        assert_eq!(lot.expected_port_out_proto, Some("UBX+NMEA".to_string()));
    }

    // ===========================================
    // D1-D3: Device構造体テスト
    // ===========================================

    #[test]
    fn test_d1_create_device() {
        let device = Device::new("0102030405060708".to_string());

        assert_eq!(device.serial_number, "0102030405060708");
        assert!(device.id.is_none());
        assert!(device.lot_id.is_none());
    }

    #[test]
    fn test_d2_device_with_lot() {
        let device = Device::new("0102030405060708".to_string())
            .with_lot(1);

        assert_eq!(device.lot_id, Some(1));
    }

    #[test]
    fn test_d3_device_with_fw_version() {
        let device = Device::new("0102030405060708".to_string())
            .with_fw_version("HPG 1.32");

        assert_eq!(device.fw_version, Some("HPG 1.32".to_string()));
    }

    // ===========================================
    // I1-I2: IndoorInspection構造体テスト
    // ===========================================

    #[test]
    fn test_i1_create_inspection() {
        let inspection = IndoorInspection::new(1, "2026-03-11T09:00:00+09:00".to_string());

        assert_eq!(inspection.device_id, 1);
        assert_eq!(inspection.inspected_at, "2026-03-11T09:00:00+09:00");
        assert!(inspection.id.is_none());
        assert!(inspection.overall_result.is_none());
    }

    #[test]
    fn test_i2_inspection_with_result() {
        let inspection = IndoorInspection::new(1, "2026-03-11T09:00:00+09:00".to_string())
            .with_result("Pass");

        assert_eq!(inspection.overall_result, Some("Pass".to_string()));
    }

    // ===========================================
    // N1-N2: InspectionItemName enumテスト
    // ===========================================

    #[test]
    fn test_n1_item_name_as_str() {
        assert_eq!(InspectionItemName::Communication.as_str(), "communication");
        assert_eq!(InspectionItemName::Serial.as_str(), "serial");
        assert_eq!(InspectionItemName::Fw.as_str(), "fw");
        assert_eq!(InspectionItemName::Rate.as_str(), "rate");
        assert_eq!(InspectionItemName::Port.as_str(), "port");
    }

    #[test]
    fn test_n2_item_name_from_str() {
        assert_eq!(InspectionItemName::from_str("communication"), Some(InspectionItemName::Communication));
        assert_eq!(InspectionItemName::from_str("serial"), Some(InspectionItemName::Serial));
        assert_eq!(InspectionItemName::from_str("fw"), Some(InspectionItemName::Fw));
        assert_eq!(InspectionItemName::from_str("rate"), Some(InspectionItemName::Rate));
        assert_eq!(InspectionItemName::from_str("port"), Some(InspectionItemName::Port));
        assert_eq!(InspectionItemName::from_str("unknown"), None);
    }

    // ===========================================
    // V1-V2: Verdict enumテスト
    // ===========================================

    #[test]
    fn test_v1_verdict_as_str() {
        assert_eq!(Verdict::Pass.as_str(), "Pass");
        assert_eq!(Verdict::Fail.as_str(), "Fail");
        assert_eq!(Verdict::Error.as_str(), "Error");
        assert_eq!(Verdict::Recorded.as_str(), "Recorded");
    }

    #[test]
    fn test_v2_verdict_from_str() {
        assert_eq!(Verdict::from_str("Pass"), Some(Verdict::Pass));
        assert_eq!(Verdict::from_str("Fail"), Some(Verdict::Fail));
        assert_eq!(Verdict::from_str("Error"), Some(Verdict::Error));
        assert_eq!(Verdict::from_str("Recorded"), Some(Verdict::Recorded));
        assert_eq!(Verdict::from_str("Unknown"), None);
    }

    // ===========================================
    // R1-R3: InspectionItemResult構造体テスト
    // ===========================================

    #[test]
    fn test_r1_create_item_result() {
        let result = InspectionItemResult::new(
            1,
            InspectionItemName::Communication,
            Verdict::Pass,
        );

        assert_eq!(result.inspection_id, 1);
        assert_eq!(result.item_name, InspectionItemName::Communication);
        assert_eq!(result.verdict, Verdict::Pass);
        assert!(result.id.is_none());
    }

    #[test]
    fn test_r2_item_result_with_actual() {
        let result = InspectionItemResult::new(
            1,
            InspectionItemName::Fw,
            Verdict::Recorded,
        )
        .with_actual("HPG 1.32");

        assert_eq!(result.actual_value, Some("HPG 1.32".to_string()));
    }

    #[test]
    fn test_r3_item_result_with_expected() {
        let result = InspectionItemResult::new(
            1,
            InspectionItemName::Rate,
            Verdict::Pass,
        )
        .with_actual("100")
        .with_expected("100");

        assert_eq!(result.actual_value, Some("100".to_string()));
        assert_eq!(result.expected_value, Some("100".to_string()));
    }
}
