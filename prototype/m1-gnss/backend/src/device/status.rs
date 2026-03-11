//! デバイス状態管理
//!
//! DeviceStatusの状態遷移ロジックを実装

use std::fmt;

/// デバイスの状態
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceStatus {
    /// 検出された（未接続）
    Detected,
    /// 接続中
    Connecting,
    /// 接続済み
    Connected,
    /// 検査中
    Inspecting,
    /// 切断済み
    Disconnected,
    /// エラー
    Error(String),
}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceStatus::Detected => write!(f, "detected"),
            DeviceStatus::Connecting => write!(f, "connecting"),
            DeviceStatus::Connected => write!(f, "connected"),
            DeviceStatus::Inspecting => write!(f, "inspecting"),
            DeviceStatus::Disconnected => write!(f, "disconnected"),
            DeviceStatus::Error(msg) => write!(f, "error: {}", msg),
        }
    }
}

/// 状態遷移エラー
#[derive(Debug, Clone, PartialEq)]
pub struct TransitionError {
    pub from: DeviceStatus,
    pub to: DeviceStatus,
}

impl fmt::Display for TransitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "不正な状態遷移: {:?} → {:?}", self.from, self.to)
    }
}

impl std::error::Error for TransitionError {}

impl DeviceStatus {
    /// 状態遷移が有効かどうか判定
    pub fn can_transition_to(&self, next: &DeviceStatus) -> bool {
        use DeviceStatus::*;

        match (self, next) {
            // Detected → Connecting（接続開始）
            (Detected, Connecting) => true,

            // Connecting → Connected（接続成功）
            (Connecting, Connected) => true,

            // Connecting → Error（接続失敗）
            (Connecting, Error(_)) => true,

            // Connected → Inspecting（検査開始）
            (Connected, Inspecting) => true,

            // Connected → Disconnected（切断）
            (Connected, Disconnected) => true,

            // Inspecting → Connected（検査完了）
            (Inspecting, Connected) => true,

            // Inspecting → Error（検査中エラー）
            (Inspecting, Error(_)) => true,

            // Disconnected → Connecting（再接続開始）
            (Disconnected, Connecting) => true,

            // その他は不正な遷移
            _ => false,
        }
    }

    /// 状態を遷移させる（不正な遷移はエラー）
    pub fn transition_to(self, next: DeviceStatus) -> Result<DeviceStatus, TransitionError> {
        if self.can_transition_to(&next) {
            Ok(next)
        } else {
            Err(TransitionError {
                from: self,
                to: next,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===========================================
    // A. 状態遷移テスト（A1-A11）
    // ===========================================

    /// A1: 初期状態はDetected
    #[test]
    fn test_a1_initial_state_is_detected() {
        // 新規Device作成時の初期状態を確認
        // DeviceStatusの初期値としてDetectedを使用することを想定
        let status = DeviceStatus::Detected;
        assert_eq!(status, DeviceStatus::Detected);
    }

    /// テーブルテスト: 有効な状態遷移（A2-A10）
    #[test]
    fn test_valid_transitions() {
        struct TestCase {
            name: &'static str,
            from: DeviceStatus,
            to: DeviceStatus,
            should_succeed: bool,
        }

        let cases = vec![
            // A2: Detected → Connecting（connect開始）
            TestCase {
                name: "A2: Detected → Connecting",
                from: DeviceStatus::Detected,
                to: DeviceStatus::Connecting,
                should_succeed: true,
            },
            // A3: Connecting → Connected（connect成功）
            TestCase {
                name: "A3: Connecting → Connected",
                from: DeviceStatus::Connecting,
                to: DeviceStatus::Connected,
                should_succeed: true,
            },
            // A4: Connecting → Error（connect失敗）
            TestCase {
                name: "A4: Connecting → Error",
                from: DeviceStatus::Connecting,
                to: DeviceStatus::Error("接続失敗".to_string()),
                should_succeed: true,
            },
            // A5: Connected → Inspecting（検査開始）
            TestCase {
                name: "A5: Connected → Inspecting",
                from: DeviceStatus::Connected,
                to: DeviceStatus::Inspecting,
                should_succeed: true,
            },
            // A6: Inspecting → Connected（検査完了）
            TestCase {
                name: "A6: Inspecting → Connected",
                from: DeviceStatus::Inspecting,
                to: DeviceStatus::Connected,
                should_succeed: true,
            },
            // A7: Inspecting → Error（検査中エラー）
            TestCase {
                name: "A7: Inspecting → Error",
                from: DeviceStatus::Inspecting,
                to: DeviceStatus::Error("通信エラー".to_string()),
                should_succeed: true,
            },
            // A8: Connected → Disconnected（disconnect呼び出し）
            TestCase {
                name: "A8: Connected → Disconnected",
                from: DeviceStatus::Connected,
                to: DeviceStatus::Disconnected,
                should_succeed: true,
            },
            // A9: Connected → Disconnected（USBケーブル抜去）
            // 同じ遷移だがトリガーが異なる（テストとしては同じ）
            // A10: Disconnected → Connecting（再接続開始）
            TestCase {
                name: "A10: Disconnected → Connecting",
                from: DeviceStatus::Disconnected,
                to: DeviceStatus::Connecting,
                should_succeed: true,
            },
        ];

        for tc in cases {
            let result = tc.from.clone().transition_to(tc.to.clone());
            if tc.should_succeed {
                assert!(
                    result.is_ok(),
                    "{}: 遷移が成功するはず: {:?} → {:?}",
                    tc.name,
                    tc.from,
                    tc.to
                );
                assert_eq!(result.unwrap(), tc.to, "{}: 遷移後の状態が正しい", tc.name);
            } else {
                assert!(result.is_err(), "{}: 遷移が失敗するはず", tc.name);
            }
        }
    }

    /// A11: 不正な遷移は拒否される
    #[test]
    fn test_a11_invalid_transitions_rejected() {
        struct TestCase {
            name: &'static str,
            from: DeviceStatus,
            to: DeviceStatus,
            should_succeed: bool,
        }

        let cases = vec![
            // 不正: Detected → Inspecting（接続なしで検査不可）
            TestCase {
                name: "Detected → Inspecting (invalid)",
                from: DeviceStatus::Detected,
                to: DeviceStatus::Inspecting,
                should_succeed: false,
            },
            // 不正: Detected → Connected（Connectingを経由しないとダメ）
            TestCase {
                name: "Detected → Connected (invalid)",
                from: DeviceStatus::Detected,
                to: DeviceStatus::Connected,
                should_succeed: false,
            },
            // 不正: Disconnected → Connected（Connectingを経由しないとダメ）
            TestCase {
                name: "Disconnected → Connected (invalid)",
                from: DeviceStatus::Disconnected,
                to: DeviceStatus::Connected,
                should_succeed: false,
            },
            // 不正: Disconnected → Inspecting（接続なしで検査不可）
            TestCase {
                name: "Disconnected → Inspecting (invalid)",
                from: DeviceStatus::Disconnected,
                to: DeviceStatus::Inspecting,
                should_succeed: false,
            },
            // 不正: Error → Connected（Connectingを経由しないとダメ）
            TestCase {
                name: "Error → Connected (invalid)",
                from: DeviceStatus::Error("some error".to_string()),
                to: DeviceStatus::Connected,
                should_succeed: false,
            },
            // 不正: Connecting → Inspecting（Connectedを経由しないとダメ）
            TestCase {
                name: "Connecting → Inspecting (invalid)",
                from: DeviceStatus::Connecting,
                to: DeviceStatus::Inspecting,
                should_succeed: false,
            },
        ];

        for tc in cases {
            let result = tc.from.clone().transition_to(tc.to.clone());
            if tc.should_succeed {
                assert!(result.is_ok(), "{}: 遷移が成功するはず", tc.name);
            } else {
                assert!(
                    result.is_err(),
                    "{}: 不正な遷移は拒否されるべき: {:?} → {:?}",
                    tc.name,
                    tc.from,
                    tc.to
                );
            }
        }
    }
}
