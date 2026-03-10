//! 検査結果判定ロジック
//!
//! 期待値と実測値を比較してVerdict（Pass/Fail/Error）を判定

use super::types::{ExpectedValue, Verdict};

/// 検査結果を判定する
///
/// # Arguments
/// * `expected` - 期待値
/// * `actual` - 実測値（Noneの場合はタイムアウトやパースエラー）
/// * `error` - エラーメッセージ（あれば）
///
/// # Returns
/// * `Verdict` - 判定結果
pub fn judge_result(
    expected: &ExpectedValue,
    actual: Option<&str>,
    error: Option<&str>,
) -> Verdict {
    // エラーがあればError
    if let Some(err) = error {
        return Verdict::Error(err.to_string());
    }

    // 実測値が取得できない場合
    let actual_value = match actual {
        Some(v) => v,
        None => return Verdict::Error("No value".to_string()),
    };

    match expected {
        // 期待値なし → 値取得成功でPass
        ExpectedValue::None => Verdict::Pass,

        // 文字列比較
        ExpectedValue::String(exp) => {
            if actual_value == exp {
                Verdict::Pass
            } else {
                Verdict::Fail
            }
        }

        // 整数比較
        ExpectedValue::Integer(exp) => {
            match actual_value.parse::<i64>() {
                Ok(actual_int) if actual_int == *exp => Verdict::Pass,
                Ok(_) => Verdict::Fail,
                Err(_) => Verdict::Error("ParseError".to_string()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===========================================
    // C1-C5: 結果判定ロジックテスト
    // ===========================================

    /// C1: 期待値と一致すればPass
    #[test]
    fn test_c1_expected_match_pass() {
        struct TestCase {
            name: &'static str,
            expected: ExpectedValue,
            actual: Option<&'static str>,
            should_pass: bool,
        }

        let cases = vec![
            // 正常系: 文字列一致
            TestCase {
                name: "string match",
                expected: ExpectedValue::String("HPG 1.32".to_string()),
                actual: Some("HPG 1.32"),
                should_pass: true,
            },
            // 正常系: 整数一致
            TestCase {
                name: "integer match",
                expected: ExpectedValue::Integer(100),
                actual: Some("100"),
                should_pass: true,
            },
        ];

        for tc in cases {
            let result = judge_result(&tc.expected, tc.actual, None);
            if tc.should_pass {
                assert_eq!(result, Verdict::Pass, "{}: should pass", tc.name);
            } else {
                assert_ne!(result, Verdict::Pass, "{}: should not pass", tc.name);
            }
        }
    }

    /// C2: 期待値と不一致ならFail
    #[test]
    fn test_c2_expected_mismatch_fail() {
        struct TestCase {
            name: &'static str,
            expected: ExpectedValue,
            actual: Option<&'static str>,
            should_fail: bool,
        }

        let cases = vec![
            // 異常系: 文字列不一致
            TestCase {
                name: "string mismatch",
                expected: ExpectedValue::String("HPG 1.32".to_string()),
                actual: Some("HPG 1.30"),
                should_fail: true,
            },
            // 異常系: 整数不一致
            TestCase {
                name: "integer mismatch",
                expected: ExpectedValue::Integer(100),
                actual: Some("200"),
                should_fail: true,
            },
        ];

        for tc in cases {
            let result = judge_result(&tc.expected, tc.actual, None);
            if tc.should_fail {
                assert_eq!(result, Verdict::Fail, "{}: should fail", tc.name);
            }
        }
    }

    /// C3: 期待値なし + 値取得成功でPass
    #[test]
    fn test_c3_no_expected_with_value_pass() {
        // シリアル番号: 期待値なし、値取得できればOK
        let result = judge_result(
            &ExpectedValue::None,
            Some("ABC123456789"),
            None,
        );

        assert_eq!(result, Verdict::Pass);
    }

    /// C4: タイムアウトでError("Timeout")
    #[test]
    fn test_c4_timeout_error() {
        let result = judge_result(
            &ExpectedValue::None,
            None, // 値取得できず
            Some("Timeout"),
        );

        assert_eq!(result, Verdict::Error("Timeout".to_string()));
    }

    /// C5: パースエラーでError("ParseError")
    #[test]
    fn test_c5_parse_error() {
        // 整数を期待しているのに文字列が来た
        let result = judge_result(
            &ExpectedValue::Integer(100),
            Some("not_a_number"),
            None,
        );

        assert_eq!(result, Verdict::Error("ParseError".to_string()));
    }

    /// 境界値: 空文字列
    #[test]
    fn test_boundary_empty_string() {
        // 空文字列と空文字列は一致
        let result = judge_result(
            &ExpectedValue::String("".to_string()),
            Some(""),
            None,
        );
        assert_eq!(result, Verdict::Pass);
    }

    /// 境界値: 負の整数
    #[test]
    fn test_boundary_negative_integer() {
        let result = judge_result(
            &ExpectedValue::Integer(-100),
            Some("-100"),
            None,
        );
        assert_eq!(result, Verdict::Pass);
    }

    /// エラーがあれば実測値は無視される
    #[test]
    fn test_error_takes_precedence() {
        let result = judge_result(
            &ExpectedValue::String("HPG 1.32".to_string()),
            Some("HPG 1.32"), // 一致するが
            Some("Connection lost"), // エラーがある
        );

        assert_eq!(result, Verdict::Error("Connection lost".to_string()));
    }
}
