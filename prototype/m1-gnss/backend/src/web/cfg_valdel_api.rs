//! CFG-VALDEL API
//!
//! DELETE /api/devices/{path}/cfg-valdel?layer=bbr&key=NAV_PVT_USB
//!
//! 作成: Session 228 (2026-03-17)
//!
//! 用途:
//! - BBRレイヤーから設定を削除（Flashが参照されるようになる）
//! - BBR優先順位問題の解決

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::device_api::{AppState, ErrorResponse, send_disable_nmea_output};
use crate::ubx::cfg_valdel::{delete_config_keys, DeleteLayer};
use crate::ubx::cfg_valset::{CFG_MSGOUT_NAV_PVT_USB, CFG_MSGOUT_NAV_PVT_UART1};

/// CFG-VALDELのClass/ID
const CFG_CLASS: u8 = 0x06;
const CFG_VALDEL_ID: u8 = 0x8C;
/// CFG-VALSET ID（NMEA OFF用）
const CFG_VALSET_ID: u8 = 0x8A;

/// クエリパラメータ
#[derive(Debug, Deserialize)]
pub struct CfgValdelQuery {
    /// 削除対象レイヤー: "bbr", "flash", "both"（必須）
    pub layer: String,
    /// 削除対象キー: "NAV_PVT_USB"（デフォルト）、"ALL"で主要キー全削除
    pub key: Option<String>,
}

/// CFG-VALDELレスポンス
#[derive(Debug, Serialize)]
pub struct CfgValdelResponse {
    pub path: String,
    pub layer: String,
    pub deleted_keys: Vec<String>,
    pub status: String,
}

/// レイヤー名を解析
fn parse_delete_layer(layer_name: &str) -> Result<DeleteLayer, String> {
    match layer_name.to_lowercase().as_str() {
        "bbr" => Ok(DeleteLayer::Bbr),
        "flash" => Ok(DeleteLayer::Flash),
        "both" | "all" | "bbr_flash" => Ok(DeleteLayer::BbrAndFlash),
        _ => Err(format!(
            "Unknown layer: {}. Supported: bbr, flash, both",
            layer_name
        )),
    }
}

/// キー名からキーIDを取得
fn get_key_id(key_name: &str) -> Result<Vec<u32>, String> {
    match key_name.to_uppercase().as_str() {
        "NAV_PVT_USB" | "CFG_MSGOUT_NAV_PVT_USB" => Ok(vec![CFG_MSGOUT_NAV_PVT_USB]),
        "NAV_PVT_UART1" | "CFG_MSGOUT_NAV_PVT_UART1" => Ok(vec![CFG_MSGOUT_NAV_PVT_UART1]),
        "NAV_PVT" | "NAV_PVT_BOTH" => Ok(vec![CFG_MSGOUT_NAV_PVT_USB, CFG_MSGOUT_NAV_PVT_UART1]),
        // ALLは主要な定期出力キーを全て削除
        "ALL" => Ok(vec![
            CFG_MSGOUT_NAV_PVT_USB,
            CFG_MSGOUT_NAV_PVT_UART1,
            // 必要に応じて他のキーを追加
        ]),
        _ => Err(format!(
            "Unknown key: {}. Supported: NAV_PVT_USB, NAV_PVT_UART1, NAV_PVT, ALL",
            key_name
        )),
    }
}

/// DELETE /api/devices/{path}/cfg-valdel - 設定削除
///
/// CFG-VALDELを送信して、指定レイヤーから設定を削除する
///
/// クエリパラメータ:
/// - layer: "bbr", "flash", "both"（必須）
/// - key: "NAV_PVT_USB"（デフォルト）、"ALL"で主要キー全削除
///
/// 動作:
/// 1. NMEA出力をOFF（レスポンス受信を妨げないため）
/// 2. CFG-VALDELを送信
/// 3. ACK/NAKを確認
///
/// 用途:
/// - BBRから設定を削除し、Flashが参照されるようにする
pub async fn cfg_valdel_handler(
    data: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<CfgValdelQuery>,
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
    let layer = match parse_delete_layer(&query.layer) {
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
    let key_ids = match get_key_id(key_name) {
        Ok(k) => k,
        Err(e) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: e,
                code: "INVALID_KEY".to_string(),
            });
        }
    };

    let key_hex: Vec<String> = key_ids.iter().map(|k| format!("0x{:08X}", k)).collect();
    tracing::info!(
        "CFG-VALDEL: layer={:?}, keys={} ({:?})",
        layer,
        key_name,
        key_hex
    );

    // NMEA出力を一時的にOFF
    if let Err(e) = send_disable_nmea_output(&mut manager) {
        tracing::warn!("NMEA OFF送信に失敗: {}", e);
    }
    let _ = manager.wait_for_ack(CFG_CLASS, CFG_VALSET_ID, Duration::from_millis(500));

    // バッファをクリア
    if let Err(e) = manager.drain_buffer() {
        tracing::warn!("バッファクリアに失敗: {}", e);
    }

    // CFG-VALDEL送信
    let valdel_cmd = delete_config_keys(layer, &key_ids);
    let hex_str: String = valdel_cmd
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ");
    tracing::info!(
        "CFG-VALDEL送信: {} ({}バイト)",
        hex_str,
        valdel_cmd.len()
    );

    if let Err(e) = manager.send_ubx(&valdel_cmd) {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("CFG-VALDEL送信に失敗: {}", e),
            code: "SEND_ERROR".to_string(),
        });
    }

    // ACK/NAK待ち
    let timeout = Duration::from_secs(2);
    match manager.wait_for_ack(CFG_CLASS, CFG_VALDEL_ID, timeout) {
        Ok(true) => {
            // ACK受信
            tracing::info!("CFG-VALDEL: ACK受信");
            HttpResponse::Ok().json(CfgValdelResponse {
                path: port_path,
                layer: format!("{:?}", layer),
                deleted_keys: key_hex,
                status: "ACK".to_string(),
            })
        }
        Ok(false) => {
            // NAK受信
            tracing::warn!("CFG-VALDEL: NAK受信");
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "CFG-VALDELがNAKを返しました。キーが無効か、レイヤーが指定されていない可能性があります。".to_string(),
                code: "NAK".to_string(),
            })
        }
        Err(e) => {
            tracing::error!("CFG-VALDEL: タイムアウトまたはエラー: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("CFG-VALDELのACK/NAK待ちでエラー: {}", e),
                code: "TIMEOUT".to_string(),
            })
        }
    }
}
