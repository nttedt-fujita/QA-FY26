//! 検査実行モジュール
//!
//! GNSS装置の受入検査を実行するエンジン

pub mod types;
pub mod judge;
pub mod engine;

pub use types::{InspectionItem, InspectionResult, ItemType, Verdict, ExpectedValue};
pub use judge::judge_result;
pub use engine::InspectionEngine;
