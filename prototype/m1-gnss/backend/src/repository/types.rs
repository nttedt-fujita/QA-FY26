//! リポジトリ型定義
//!
//! InspectionRecord: DB保存用の検査レコード

use std::collections::HashMap;

/// 検査レコード（DB保存用）
#[derive(Debug, Clone)]
pub struct InspectionRecord {
    /// ID（保存後に付与）
    pub id: Option<i64>,
    /// 装置シリアル番号（SEC-UNIQID 16進文字列）
    pub device_serial: String,
    /// 検査日時（ISO8601形式）
    pub inspected_at: String,
    /// 全体判定（Pass/Fail/Error）
    pub overall_verdict: String,
    /// FWバージョン（実測値）
    pub fw_version: Option<String>,
    /// 各項目の判定
    pub item_verdicts: HashMap<String, String>,
    /// 備考
    pub notes: Option<String>,
}

impl InspectionRecord {
    /// 新しい検査レコードを作成
    pub fn new(device_serial: String, inspected_at: String, overall_verdict: String) -> Self {
        Self {
            id: None,
            device_serial,
            inspected_at,
            overall_verdict,
            fw_version: None,
            item_verdicts: HashMap::new(),
            notes: None,
        }
    }

    /// FWバージョンを設定
    pub fn with_fw_version(mut self, fw_version: String) -> Self {
        self.fw_version = Some(fw_version);
        self
    }

    /// 項目判定を追加
    pub fn with_item_verdict(mut self, item: &str, verdict: &str) -> Self {
        self.item_verdicts.insert(item.to_string(), verdict.to_string());
        self
    }

    /// 備考を設定
    pub fn with_notes(mut self, notes: String) -> Self {
        self.notes = Some(notes);
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
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::Connection(msg) => write!(f, "Connection error: {}", msg),
            RepositoryError::Sql(msg) => write!(f, "SQL error: {}", msg),
            RepositoryError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {}

#[cfg(test)]
mod tests {
    use super::*;

    // ===========================================
    // R1-R2: InspectionRecord構造体テスト
    // ===========================================

    /// R1: InspectionRecordを作成できる
    #[test]
    fn test_r1_create_inspection_record() {
        let record = InspectionRecord::new(
            "0102030405060708".to_string(),
            "2026-03-11T09:00:00+09:00".to_string(),
            "Pass".to_string(),
        );

        assert_eq!(record.device_serial, "0102030405060708");
        assert_eq!(record.inspected_at, "2026-03-11T09:00:00+09:00");
        assert_eq!(record.overall_verdict, "Pass");
        assert!(record.id.is_none());
    }

    /// R1: FWバージョンを設定できる
    #[test]
    fn test_r1_with_fw_version() {
        let record = InspectionRecord::new(
            "0102030405060708".to_string(),
            "2026-03-11T09:00:00+09:00".to_string(),
            "Pass".to_string(),
        )
        .with_fw_version("HPG 1.32".to_string());

        assert_eq!(record.fw_version, Some("HPG 1.32".to_string()));
    }

    /// R1: 項目判定を追加できる
    #[test]
    fn test_r1_with_item_verdict() {
        let record = InspectionRecord::new(
            "0102030405060708".to_string(),
            "2026-03-11T09:00:00+09:00".to_string(),
            "Pass".to_string(),
        )
        .with_item_verdict("connectivity", "Pass")
        .with_item_verdict("fw_version", "Pass")
        .with_item_verdict("serial_number", "Pass");

        assert_eq!(record.item_verdicts.len(), 3);
        assert_eq!(record.item_verdicts.get("connectivity"), Some(&"Pass".to_string()));
    }

    /// R2: ID は保存前は None
    #[test]
    fn test_r2_id_none_before_save() {
        let record = InspectionRecord::new(
            "0102030405060708".to_string(),
            "2026-03-11T09:00:00+09:00".to_string(),
            "Pass".to_string(),
        );

        assert!(record.id.is_none());
    }
}
