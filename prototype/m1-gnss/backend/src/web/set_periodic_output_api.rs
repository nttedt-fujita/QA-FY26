//! 定期出力設定API
//!
//! - POST /api/devices/{path}/set-periodic-output - 定期出力を有効化
//!
//! 作成: Session 218 (2026-03-17)
//!
//! 用途:
//! - reset-config APIの効果確認用テストAPI
//! - 定期出力を意図的に有効化してBBR/Flashに保存

use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use std::time::Duration;

use super::device_api::{AppState, ErrorResponse, send_disable_nmea_output};
use crate::ubx::cfg_valset::{set_periodic_output, PeriodicOutputConfig, Layer};

/// CFG-VALSETのClass/ID
const CFG_CLASS: u8 = 0x06;
const CFG_VALSET_ID: u8 = 0x8A;

/// 定期出力設定レスポンス
#[derive(Debug, Serialize)]
pub struct SetPeriodicOutputResponse {
    pub path: String,
    pub message: String,
}

/// POST /api/devices/{path}/set-periodic-output - 定期出力有効化
///
/// CFG-VALSETを送信して、定期出力をBBR+RAMに設定する
///
/// 動作:
/// 1. NMEA出力をOFF（ACK受信を妨げないため）
/// 2. CFG-VALSET（NAV-PVT等の定期出力設定）を送信
/// 3. ACK/NAKを確認
///
/// 設定内容（PeriodicOutputConfig::default()）:
/// - NAV-PVT: 毎エポック
/// - NAV-STATUS: 毎エポック
/// - NAV-SAT: 5エポックに1回
/// - NAV-SIG: 5エポックに1回
/// - MON-SPAN: 10エポックに1回
/// - MON-RF: 10エポックに1回
pub async fn set_periodic_output_handler(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let port_path = urlencoding::decode(&path.into_inner())
        .unwrap_or_else(|_| std::borrow::Cow::Borrowed(""))
        .to_string();

    // デバイスマネージャーを取得
    let device_manager_arc = match data.get_device_manager_by_path(&port_path) {
        Ok(Some(arc)) => arc,
        Ok(None) => {
            return HttpResponse::NotFound().json(ErrorResponse {
                error: format!("デバイスが接続されていません: {}", port_path),
                code: "NOT_CONNECTED".to_string(),
            });
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: e.to_string(),
                code: "INTERNAL_ERROR".to_string(),
            });
        }
    };

    let mut manager = match device_manager_arc.lock() {
        Ok(m) => m,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "デバイスロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            });
        }
    };

    // NMEA出力を一時的にOFF（ACK受信を妨げないため）
    if let Err(e) = send_disable_nmea_output(&mut manager) {
        tracing::warn!("NMEA OFF送信に失敗: {}", e);
    }
    // NMEA OFFのACKを待つ
    let _ = manager.wait_for_ack(CFG_CLASS, CFG_VALSET_ID, Duration::from_millis(500));

    // バッファをクリア
    if let Err(e) = manager.drain_buffer() {
        tracing::warn!("バッファクリアに失敗: {}", e);
    }

    // 定期出力設定（デフォルト値、RAM+BBR+Flashに保存）
    // Session 220: BBRはバッテリーバックアップがないと電源断で消えるため、Flashにも保存
    // 出典: u-blox F9 HPG 1.32 Interface Description p.224
    let config = PeriodicOutputConfig::default();
    let set_cmd = set_periodic_output(&config, Layer::RamBbrFlash);
    let hex_str: String = set_cmd.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ");
    tracing::info!("CFG-VALSET（定期出力）送信データ: {} ({}バイト)", hex_str, set_cmd.len());

    // 送信
    if let Err(e) = manager.send_ubx(&set_cmd) {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("CFG-VALSET送信に失敗: {}", e),
            code: "SEND_ERROR".to_string(),
        });
    }

    // ACK待ち
    let ack_result = manager.wait_for_ack(CFG_CLASS, CFG_VALSET_ID, Duration::from_secs(2));

    // ACK/NAK確認
    match ack_result {
        Ok(true) => {
            tracing::info!("CFG-VALSET（定期出力）ACK received");
            HttpResponse::Ok().json(SetPeriodicOutputResponse {
                path: port_path,
                message: "定期出力を有効化しました（NAV-PVT, NAV-STATUS, NAV-SAT, NAV-SIG, MON-SPAN, MON-RF）".to_string(),
            })
        }
        Ok(false) => {
            tracing::warn!("CFG-VALSET（定期出力）NAK received");
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "CFG-VALSETがデバイスに拒否されました".to_string(),
                code: "COMMAND_REJECTED".to_string(),
            })
        }
        Err(e) => {
            tracing::warn!("CFG-VALSET（定期出力）ACK待ちに失敗: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("CFG-VALSET応答受信に失敗: {}", e),
                code: "RECEIVE_ERROR".to_string(),
            })
        }
    }
}
