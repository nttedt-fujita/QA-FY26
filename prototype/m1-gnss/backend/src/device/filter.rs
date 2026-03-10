//! ポートフィルタリング
//!
//! u-blox F9P装置のみを抽出するフィルタ

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

/// u-blox F9P の Vendor ID
pub const F9P_VID: u16 = 0x1546;

/// u-blox F9P の Product ID
pub const F9P_PID: u16 = 0x01A9;

/// F9P装置かどうか判定
pub fn is_f9p_device(port: &PortInfo) -> bool {
    match (port.vid, port.pid) {
        (Some(vid), Some(pid)) => vid == F9P_VID && pid == F9P_PID,
        _ => false,
    }
}

/// ポートリストからF9P装置のみを抽出
pub fn filter_f9p_ports(ports: Vec<PortInfo>) -> Vec<PortInfo> {
    ports.into_iter().filter(is_f9p_device).collect()
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
}
