//! MON-VER API (`GET /api/devices/{path}/mon-ver`)
//!
//! モジュール情報（ソフトウェア/ハードウェアバージョン）を取得

use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

use crate::ubx::mon_ver;
use super::device_api::{AppState, ErrorResponse, send_disable_nmea_output};

#[derive(Serialize)]
pub struct MonVerResponse {
    pub sw_version: String,
    pub hw_version: String,
    pub fw_version: Option<String>,
    pub protocol_version: Option<String>,
    pub extensions: Vec<String>,
}

/// GET /api/devices/{path}/mon-ver
pub async fn get_mon_ver(
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
                error: format!("装置が接続されていません: {}", port_path),
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

    let mut manager = device_manager_arc.lock().unwrap();

    // NMEA OFF
    if let Err(e) = send_disable_nmea_output(&mut manager) {
        tracing::warn!("NMEA OFF失敗: {}", e);
    }

    // バッファドレイン
    if let Err(e) = manager.drain_buffer() {
        tracing::warn!("バッファドレイン失敗: {}", e);
    }

    // MON-VER poll送信
    let poll_msg = mon_ver::poll_request();
    if let Err(e) = manager.send_ubx(&poll_msg) {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("MON-VER送信エラー: {}", e),
            code: "SEND_ERROR".to_string(),
        });
    }

    // 受信（タイムアウト2秒）
    let raw = match manager.receive_ubx(std::time::Duration::from_millis(2000)) {
        Ok(r) => r,
        Err(crate::device::manager::DeviceManagerError::Timeout) => {
            return HttpResponse::GatewayTimeout().json(ErrorResponse {
                error: "MON-VER受信タイムアウト".to_string(),
                code: "TIMEOUT".to_string(),
            });
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("MON-VER受信エラー: {}", e),
                code: "RECEIVE_ERROR".to_string(),
            });
        }
    };

    // パース
    match mon_ver::parse(&raw) {
        Ok(ver) => {
            let fw_version = ver.fw_version();
            let protocol_version = ver.protocol_version();
            HttpResponse::Ok().json(MonVerResponse {
                sw_version: ver.sw_version,
                hw_version: ver.hw_version,
                fw_version,
                protocol_version,
                extensions: ver.extensions,
            })
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("MON-VERパースエラー: {}", e),
                code: "PARSE_ERROR".to_string(),
            })
        }
    }
}
