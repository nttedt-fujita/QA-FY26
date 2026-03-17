//! CFG-VALGET API
//!
//! GET /api/devices/{path}/cfg-valget?layer=flash&key=NAV_PVT_USB
//!
//! 作成: Session 223 (2026-03-17)
//!
//! 用途:
//! - Flashレイヤーから設定値を読み取り、Flash搭載確認

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::device_api::{AppState, ErrorResponse, send_disable_nmea_output};
use crate::ubx::cfg_valget::{build_cfg_valget_poll, parse_cfg_valget_response, parse_layer_name};
use crate::ubx::cfg_valset::{CFG_MSGOUT_NAV_PVT_USB, CFG_MSGOUT_NAV_PVT_UART1};

/// CFG-VALGETのClass/ID
const CFG_CLASS: u8 = 0x06;
const CFG_VALGET_ID: u8 = 0x8B;
/// CFG-VALSET ID（NMEA OFF用）
const CFG_VALSET_ID: u8 = 0x8A;

/// クエリパラメータ
#[derive(Debug, Deserialize)]
pub struct CfgValgetQuery {
    /// 読み取り対象レイヤー: "ram", "bbr", "flash", "default"（必須）
    pub layer: String,
    /// 読み取り対象キー: "NAV_PVT_USB"（デフォルト）
    pub key: Option<String>,
}

/// CFG-VALGETレスポンス
#[derive(Debug, Serialize)]
pub struct CfgValgetResponse {
    pub path: String,
    pub layer: String,
    pub key: String,
    pub key_id: String,
    /// 値（U1型の場合は1バイト）
    pub value: u8,
}

/// キー名からキーIDを取得
fn get_key_id(key_name: &str) -> Result<u32, String> {
    match key_name.to_uppercase().as_str() {
        "NAV_PVT_USB" | "CFG_MSGOUT_NAV_PVT_USB" => Ok(CFG_MSGOUT_NAV_PVT_USB),
        "NAV_PVT_UART1" | "CFG_MSGOUT_NAV_PVT_UART1" => Ok(CFG_MSGOUT_NAV_PVT_UART1),
        _ => Err(format!("Unknown key: {}. Supported: NAV_PVT_USB, NAV_PVT_UART1", key_name)),
    }
}

/// GET /api/devices/{path}/cfg-valget - 設定値取得
///
/// CFG-VALGET Pollを送信して、指定レイヤーから設定値を読み取る
///
/// クエリパラメータ:
/// - layer: "ram", "bbr", "flash", "default"（必須）
/// - key: "NAV_PVT_USB"（デフォルト）
///
/// 動作:
/// 1. NMEA出力をOFF（レスポンス受信を妨げないため）
/// 2. CFG-VALGET Pollを送信
/// 3. CFG-VALGETレスポンスを受信・パース
///
/// 用途:
/// - Flashレイヤーから値が読めるか確認（読める→Flash搭載）
pub async fn cfg_valget_handler(
    data: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<CfgValgetQuery>,
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

    // レイヤー解析
    let layer = match parse_layer_name(&query.layer) {
        Ok(l) => l,
        Err(e) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: e,
                code: "INVALID_LAYER".to_string(),
            });
        }
    };

    // キー解析
    let key_name = query.key.as_deref().unwrap_or("NAV_PVT_USB");
    let key_id = match get_key_id(key_name) {
        Ok(k) => k,
        Err(e) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: e,
                code: "INVALID_KEY".to_string(),
            });
        }
    };

    tracing::info!("CFG-VALGET: layer={}, key={} (0x{:08X})", layer, key_name, key_id);

    // NMEA出力を一時的にOFF
    if let Err(e) = send_disable_nmea_output(&mut manager) {
        tracing::warn!("NMEA OFF送信に失敗: {}", e);
    }
    let _ = manager.wait_for_ack(CFG_CLASS, CFG_VALSET_ID, Duration::from_millis(500));

    // バッファをクリア
    if let Err(e) = manager.drain_buffer() {
        tracing::warn!("バッファクリアに失敗: {}", e);
    }

    // CFG-VALGET Poll送信
    let poll_cmd = build_cfg_valget_poll(layer, &[key_id]);
    let hex_str: String = poll_cmd.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ");
    tracing::info!("CFG-VALGET Poll送信: {} ({}バイト)", hex_str, poll_cmd.len());

    if let Err(e) = manager.send_ubx(&poll_cmd) {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("CFG-VALGET Poll送信に失敗: {}", e),
            code: "SEND_ERROR".to_string(),
        });
    }

    // レスポンス待ち（CFG-VALGETレスポンスを受信）
    // Class=0x06, ID=0x8Bのメッセージを待つ
    let timeout = Duration::from_secs(2);
    let response_result = manager.wait_for_ubx_message(CFG_CLASS, CFG_VALGET_ID, timeout);

    match response_result {
        Ok(payload) => {
            let hex_str: String = payload.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ");
            tracing::info!("CFG-VALGET レスポンス受信: {} ({}バイト)", hex_str, payload.len());

            match parse_cfg_valget_response(&payload) {
                Ok(response) => {
                    if response.values.is_empty() {
                        return HttpResponse::NotFound().json(ErrorResponse {
                            error: format!("指定されたキーの値が見つかりません: {}", key_name),
                            code: "KEY_NOT_FOUND".to_string(),
                        });
                    }

                    let (returned_key, value_bytes) = &response.values[0];
                    let value = if value_bytes.is_empty() { 0 } else { value_bytes[0] };

                    HttpResponse::Ok().json(CfgValgetResponse {
                        path: port_path,
                        layer: response.layer.to_string(),
                        key: key_name.to_string(),
                        key_id: format!("0x{:08X}", returned_key),
                        value,
                    })
                }
                Err(e) => {
                    tracing::error!("CFG-VALGET レスポンスパースに失敗: {}", e);
                    HttpResponse::InternalServerError().json(ErrorResponse {
                        error: format!("CFG-VALGETレスポンスのパースに失敗: {}", e),
                        code: "PARSE_ERROR".to_string(),
                    })
                }
            }
        }
        Err(e) => {
            // NAKまたはタイムアウト
            // Flashレイヤーが存在しない場合、NAKが返る可能性がある
            tracing::warn!("CFG-VALGET レスポンス受信に失敗: {}", e);
            HttpResponse::NotFound().json(ErrorResponse {
                error: format!(
                    "CFG-VALGETレスポンスを受信できませんでした (layer={})。このレイヤーは利用できない可能性があります。",
                    layer
                ),
                code: "LAYER_NOT_AVAILABLE".to_string(),
            })
        }
    }
}
