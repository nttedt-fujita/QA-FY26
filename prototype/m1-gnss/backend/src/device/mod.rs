//! デバイス管理モジュール
//!
//! GNSS装置（u-blox F9P）の検出・接続・状態管理を担当

pub mod status;
pub mod filter;
pub mod manager;

pub use status::{DeviceStatus, TransitionError};
pub use filter::{is_f9p_device, filter_f9p_ports, PortInfo};
pub use manager::{DeviceManager, DeviceManagerError};
