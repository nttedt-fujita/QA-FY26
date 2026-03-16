//! LED点滅API
//!
//! - POST /api/devices/{path}/blink - デバイスのLEDを点滅させる
//!
//! 作成: Session 204 (2026-03-16)

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

use super::device_api::{AppState, ErrorResponse};
use crate::ubx::common::build_ubx_poll;

/// 点滅リクエスト
#[derive(Debug, Deserialize)]
pub struct BlinkRequest {
    /// 点滅時間（秒）。デフォルト: 3
    #[serde(default = "default_duration")]
    pub duration_sec: u64,
}

fn default_duration() -> u64 {
    3
}

/// 点滅レスポンス
#[derive(Debug, Serialize)]
pub struct BlinkResponse {
    pub path: String,
    pub message: String,
    pub send_count: u32,
}

/// POST /api/devices/{path}/blink - LED点滅
///
/// USB-UART基板のTX LEDを点滅させて、デバイスを物理的に識別する
pub async fn blink_device(
    data: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<BlinkRequest>,
) -> impl Responder {
    let port_path = urlencoding::decode(&path.into_inner())
        .unwrap_or_else(|_| std::borrow::Cow::Borrowed(""))
        .to_string();

    let duration_sec = query.duration_sec.min(10); // 最大10秒

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

    // MON-VER poll (Class: 0x0A, ID: 0x04)
    let poll_cmd = build_ubx_poll(0x0A, 0x04);

    let start = Instant::now();
    let mut count: u32 = 0;

    // 点滅ループ（100msごとにMON-VERポーリング送信）
    while start.elapsed().as_secs() < duration_sec {
        let mut manager = match device_manager_arc.lock() {
            Ok(m) => m,
            Err(_) => {
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "デバイスロック取得に失敗".to_string(),
                    code: "LOCK_ERROR".to_string(),
                });
            }
        };

        if let Err(e) = manager.send_ubx(&poll_cmd) {
            tracing::warn!("点滅送信エラー: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("送信エラー: {}", e),
                code: "SEND_ERROR".to_string(),
            });
        }
        count += 1;

        // ロックを解放してから待機
        drop(manager);
        std::thread::sleep(Duration::from_millis(100));
    }

    HttpResponse::Ok().json(BlinkResponse {
        path: port_path,
        message: format!("点滅完了（{}秒間）", duration_sec),
        send_count: count,
    })
}
