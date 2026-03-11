//! リポジトリモジュール
//!
//! 検査結果の永続化を担当

mod types;
mod sqlite;

pub use types::*;
pub use sqlite::SqliteRepository;
