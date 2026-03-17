//! 設定リセットAPI
//!
//! - POST /api/devices/{path}/reset-config - デバイス設定をデフォルトにリセット
//!
//! 作成: Session 212 (2026-03-16)
//!
//! 用途:
//! - RTK基準局設定など、BBRに保存された定期出力設定をクリア
//! - 工場出荷時のデフォルト設定に復元

use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use std::time::Duration;

use super::device_api::{AppState, ErrorResponse, send_disable_nmea_output};
use crate::ubx::cfg_cfg::reset_config_to_default;

/// CFG-CFGのClass/ID
const CFG_CLASS: u8 = 0x06;
const CFG_CFG_ID: u8 = 0x09;

/// リセットレスポンス
#[derive(Debug, Serialize)]
pub struct ResetConfigResponse {
    pub path: String,
    pub message: String,
}

/// POST /api/devices/{path}/reset-config - 設定リセット
///
/// CFG-CFGを送信して、BBRの設定をクリアし、ROMデフォルトを復元する
///
/// 動作:
/// 1. CFG-CFG (clearMask=0xFFFF, loadMask=0xFFFF, deviceMask=BBR) を送信
/// 2. ACK/NAKを確認
///
/// 注意:
/// - この操作後、レシーバーは工場出荷時の設定で動作する
/// - RTK基準局設定など、BBRに保存された設定は全て消える
pub async fn reset_config(
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
    let _ = manager.wait_for_ack(0x06, 0x8A, Duration::from_millis(500));

    // バッファをクリア
    if let Err(e) = manager.drain_buffer() {
        tracing::warn!("バッファクリアに失敗: {}", e);
    }

    // CFG-CFG（BBRクリア + デフォルト復元）を生成
    let reset_cmd = reset_config_to_default();
    let hex_str: String = reset_cmd.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ");
    tracing::info!("CFG-CFG送信データ: {} ({}バイト)", hex_str, reset_cmd.len());

    // 送信
    if let Err(e) = manager.send_ubx(&reset_cmd) {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("CFG-CFG送信に失敗: {}", e),
            code: "SEND_ERROR".to_string(),
        });
    }

    // ACK待ち（Flash操作は時間がかかる可能性があるため5秒待機）
    let ack_result = manager.wait_for_ack(CFG_CLASS, CFG_CFG_ID, Duration::from_secs(5));

    // ACK/NAK確認
    match ack_result {
        Ok(true) => {
            tracing::info!("CFG-CFG ACK received");
            HttpResponse::Ok().json(ResetConfigResponse {
                path: port_path,
                message: "設定をデフォルトにリセットしました".to_string(),
            })
        }
        Ok(false) => {
            tracing::warn!("CFG-CFG NAK received");
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "CFG-CFGがデバイスに拒否されました".to_string(),
                code: "COMMAND_REJECTED".to_string(),
            })
        }
        Err(e) => {
            tracing::warn!("CFG-CFG ACK待ちに失敗: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("CFG-CFG応答受信に失敗: {}", e),
                code: "RECEIVE_ERROR".to_string(),
            })
        }
    }
}
