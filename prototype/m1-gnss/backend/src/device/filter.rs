//! ポートフィルタリング
//!
//! u-blox F9P装置を抽出するフィルタ
//! - USB直接接続: VID=0x1546, PID=0x01A9
//! - FTDI経由UART接続: VID=0x0403, PID=0x6015

/// ポート情報
#[derive(Debug, Clone, PartialEq)]
pub struct PortInfo {
    /// ポートパス（例: /dev/ttyACM0）
    pub path: String,
    /// Vendor ID
    pub vid: Option<u16>,
    /// Product ID
    pub pid: Option<u16>,
    /// シリアル番号
    pub serial_number: Option<String>,
}

/// u-blox F9P の Vendor ID（USB直接接続）
pub const F9P_VID: u16 = 0x1546;

/// u-blox F9P の Product ID（USB直接接続）
pub const F9P_PID: u16 = 0x01A9;

/// FTDI の Vendor ID
pub const FTDI_VID: u16 = 0x0403;

/// FTDI FT232H の Product ID（F9P評価ボードで使用）
pub const FTDI_FT232H_PID: u16 = 0x6015;

/// F9P装置かどうか判定（USB直接接続）
pub fn is_f9p_device(port: &PortInfo) -> bool {
    match (port.vid, port.pid) {
        (Some(vid), Some(pid)) => vid == F9P_VID && pid == F9P_PID,
        _ => false,
    }
}

/// FTDI経由のGNSS装置かどうか判定
pub fn is_ftdi_device(port: &PortInfo) -> bool {
    match (port.vid, port.pid) {
        (Some(vid), Some(pid)) => vid == FTDI_VID && pid == FTDI_FT232H_PID,
        _ => false,
    }
}

/// GNSS装置かどうか判定（F9P直接 OR FTDI経由）
pub fn is_gnss_device(port: &PortInfo) -> bool {
    is_f9p_device(port) || is_ftdi_device(port)
}

/// ポートリストからF9P装置のみを抽出（後方互換性のため維持）
pub fn filter_f9p_ports(ports: Vec<PortInfo>) -> Vec<PortInfo> {
    ports.into_iter().filter(is_f9p_device).collect()
}

/// ポートリストからGNSS装置を抽出（F9P直接 + FTDI経由）
pub fn filter_gnss_ports(ports: Vec<PortInfo>) -> Vec<PortInfo> {
    ports.into_iter().filter(is_gnss_device).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===========================================
    // B. フィルタリングテスト（B1-B5）
    // ===========================================

    /// B1-B4: is_f9p_device のテーブルテスト
    #[test]
    fn test_is_f9p_device() {
        struct TestCase {
            name: &'static str,
            vid: Option<u16>,
            pid: Option<u16>,
            expected: bool,
            should_succeed: bool,
        }

        let cases = vec![
            // B1: F9Pはフィルタを通過
            TestCase {
                name: "B1: F9P device (VID=0x1546, PID=0x01A9)",
                vid: Some(F9P_VID),
                pid: Some(F9P_PID),
                expected: true,
                should_succeed: true,
            },
            // B2: VIDが異なればフィルタで除外
            TestCase {
                name: "B2: Wrong VID",
                vid: Some(0x1234),
                pid: Some(F9P_PID),
                expected: false,
                should_succeed: true,
            },
            // B3: PIDが異なればフィルタで除外
            TestCase {
                name: "B3: Wrong PID",
                vid: Some(F9P_VID),
                pid: Some(0x0000),
                expected: false,
                should_succeed: true,
            },
            // B4: VID/PIDがNoneならフィルタで除外
            TestCase {
                name: "B4: VID/PID are None",
                vid: None,
                pid: None,
                expected: false,
                should_succeed: true,
            },
            // 追加: VIDのみNone
            TestCase {
                name: "VID is None",
                vid: None,
                pid: Some(F9P_PID),
                expected: false,
                should_succeed: true,
            },
            // 追加: PIDのみNone
            TestCase {
                name: "PID is None",
                vid: Some(F9P_VID),
                pid: None,
                expected: false,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let port = PortInfo {
                path: "/dev/ttyACM0".to_string(),
                vid: tc.vid,
                pid: tc.pid,
                serial_number: None,
            };

            if tc.should_succeed {
                let result = is_f9p_device(&port);
                assert_eq!(result, tc.expected, "{}", tc.name);
            }
        }
    }

    /// B5: 複数ポートからF9Pのみ抽出
    #[test]
    fn test_b5_filter_f9p_ports() {
        let ports = vec![
            // F9P device 1
            PortInfo {
                path: "/dev/ttyACM0".to_string(),
                vid: Some(F9P_VID),
                pid: Some(F9P_PID),
                serial_number: Some("ABC123".to_string()),
            },
            // 他社デバイス
            PortInfo {
                path: "/dev/ttyACM1".to_string(),
                vid: Some(0x1234),
                pid: Some(0x5678),
                serial_number: None,
            },
            // F9P device 2
            PortInfo {
                path: "/dev/ttyACM2".to_string(),
                vid: Some(F9P_VID),
                pid: Some(F9P_PID),
                serial_number: Some("DEF456".to_string()),
            },
        ];

        let filtered = filter_f9p_ports(ports);

        assert_eq!(filtered.len(), 2, "F9Pは2台のみ");
        assert_eq!(filtered[0].path, "/dev/ttyACM0");
        assert_eq!(filtered[1].path, "/dev/ttyACM2");
    }

    /// B5追加: 空リストの場合
    #[test]
    fn test_filter_empty_list() {
        let ports: Vec<PortInfo> = vec![];
        let filtered = filter_f9p_ports(ports);
        assert!(filtered.is_empty());
    }

    /// B5追加: F9Pがない場合
    #[test]
    fn test_filter_no_f9p() {
        let ports = vec![
            PortInfo {
                path: "/dev/ttyUSB0".to_string(),
                vid: Some(0x1234),
                pid: Some(0x5678),
                serial_number: None,
            },
        ];
        let filtered = filter_f9p_ports(ports);
        assert!(filtered.is_empty());
    }

    // ===========================================
    // C. FTDI対応テスト（C1-C5）
    // ===========================================

    /// C1-C4: is_ftdi_device のテーブルテスト
    #[test]
    fn test_is_ftdi_device() {
        struct TestCase {
            name: &'static str,
            vid: Option<u16>,
            pid: Option<u16>,
            expected: bool,
            should_succeed: bool,
        }

        let cases = vec![
            // C1: FTDI FT232Hはフィルタを通過
            TestCase {
                name: "C1: FTDI FT232H (VID=0x0403, PID=0x6015)",
                vid: Some(FTDI_VID),
                pid: Some(FTDI_FT232H_PID),
                expected: true,
                should_succeed: true,
            },
            // C2: VIDが異なればフィルタで除外
            TestCase {
                name: "C2: Wrong VID for FTDI",
                vid: Some(0x1234),
                pid: Some(FTDI_FT232H_PID),
                expected: false,
                should_succeed: true,
            },
            // C3: PIDが異なればフィルタで除外
            TestCase {
                name: "C3: Wrong PID for FTDI",
                vid: Some(FTDI_VID),
                pid: Some(0x0000),
                expected: false,
                should_succeed: true,
            },
            // C4: F9P直接接続はFTDIではない
            TestCase {
                name: "C4: F9P direct is not FTDI",
                vid: Some(F9P_VID),
                pid: Some(F9P_PID),
                expected: false,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let port = PortInfo {
                path: "/dev/ttyUSB0".to_string(),
                vid: tc.vid,
                pid: tc.pid,
                serial_number: None,
            };

            if tc.should_succeed {
                let result = is_ftdi_device(&port);
                assert_eq!(result, tc.expected, "{}", tc.name);
            }
        }
    }

    /// C5: is_gnss_device のテーブルテスト（F9P直接 OR FTDI経由）
    #[test]
    fn test_is_gnss_device() {
        struct TestCase {
            name: &'static str,
            vid: Option<u16>,
            pid: Option<u16>,
            expected: bool,
            should_succeed: bool,
        }

        let cases = vec![
            // F9P直接接続
            TestCase {
                name: "F9P direct",
                vid: Some(F9P_VID),
                pid: Some(F9P_PID),
                expected: true,
                should_succeed: true,
            },
            // FTDI経由
            TestCase {
                name: "FTDI FT232H",
                vid: Some(FTDI_VID),
                pid: Some(FTDI_FT232H_PID),
                expected: true,
                should_succeed: true,
            },
            // 他社デバイス
            TestCase {
                name: "Other device",
                vid: Some(0x1234),
                pid: Some(0x5678),
                expected: false,
                should_succeed: true,
            },
            // VID/PIDがNone
            TestCase {
                name: "None VID/PID",
                vid: None,
                pid: None,
                expected: false,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let port = PortInfo {
                path: "/dev/ttyUSB0".to_string(),
                vid: tc.vid,
                pid: tc.pid,
                serial_number: None,
            };

            if tc.should_succeed {
                let result = is_gnss_device(&port);
                assert_eq!(result, tc.expected, "{}", tc.name);
            }
        }
    }

    /// C6: filter_gnss_ports でF9P + FTDIを抽出
    #[test]
    fn test_filter_gnss_ports() {
        let ports = vec![
            // F9P直接接続
            PortInfo {
                path: "/dev/ttyACM0".to_string(),
                vid: Some(F9P_VID),
                pid: Some(F9P_PID),
                serial_number: Some("F9P_001".to_string()),
            },
            // FTDI経由
            PortInfo {
                path: "/dev/ttyUSB0".to_string(),
                vid: Some(FTDI_VID),
                pid: Some(FTDI_FT232H_PID),
                serial_number: None,
            },
            // 他社デバイス
            PortInfo {
                path: "/dev/ttyUSB1".to_string(),
                vid: Some(0x1234),
                pid: Some(0x5678),
                serial_number: None,
            },
        ];

        let filtered = filter_gnss_ports(ports);

        assert_eq!(filtered.len(), 2, "GNSSデバイスは2台（F9P + FTDI）");
        assert_eq!(filtered[0].path, "/dev/ttyACM0");
        assert_eq!(filtered[1].path, "/dev/ttyUSB0");
    }
}
