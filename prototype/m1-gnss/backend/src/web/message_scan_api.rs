//! メッセージスキャンAPI
//!
//! 接続中の装置から流れてくるUBXメッセージを一定時間モニタリングし、
//! どのメッセージが出力されているかを確認する。
//!
//! - GET /api/devices/{path}/message-scan - メッセージスキャン実行

use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::device_api::{AppState, ErrorResponse};
use crate::ubx::common::{UBX_SYNC_1, UBX_SYNC_2};

// ===========================================
// UBXメッセージ名マッピング
// 出典: u-blox F9 HPG 1.32 Interface Description
// ===========================================

/// UBXメッセージ名を取得
fn get_message_name(class: u8, id: u8) -> &'static str {
    match (class, id) {
        // NAV (0x01)
        (0x01, 0x01) => "NAV-POSECEF",
        (0x01, 0x02) => "NAV-POSLLH",
        (0x01, 0x03) => "NAV-STATUS",
        (0x01, 0x04) => "NAV-DOP",
        (0x01, 0x06) => "NAV-SOL",
        (0x01, 0x07) => "NAV-PVT",
        (0x01, 0x09) => "NAV-ODO",
        (0x01, 0x10) => "NAV-RESETODO",
        (0x01, 0x11) => "NAV-VELECEF",
        (0x01, 0x12) => "NAV-VELNED",
        (0x01, 0x13) => "NAV-HPPOSECEF",
        (0x01, 0x14) => "NAV-HPPOSLLH",
        (0x01, 0x20) => "NAV-TIMEGPS",
        (0x01, 0x21) => "NAV-TIMEUTC",
        (0x01, 0x22) => "NAV-CLOCK",
        (0x01, 0x23) => "NAV-TIMEGLO",
        (0x01, 0x24) => "NAV-TIMEBDS",
        (0x01, 0x25) => "NAV-TIMEGAL",
        (0x01, 0x26) => "NAV-TIMELS",
        (0x01, 0x27) => "NAV-TIMEQZSS",
        (0x01, 0x32) => "NAV-SBAS",
        (0x01, 0x35) => "NAV-SAT",
        (0x01, 0x39) => "NAV-GEOFENCE",
        (0x01, 0x3B) => "NAV-SVIN",
        (0x01, 0x3C) => "NAV-RELPOSNED",
        (0x01, 0x43) => "NAV-SIG",
        (0x01, 0x60) => "NAV-AOPSTATUS",
        (0x01, 0x61) => "NAV-EOE",
        // RXM (0x02)
        (0x02, 0x13) => "RXM-SFRBX",
        (0x02, 0x14) => "RXM-MEASX",
        (0x02, 0x15) => "RXM-RAWX",
        (0x02, 0x32) => "RXM-RTCM",
        (0x02, 0x41) => "RXM-PMREQ",
        (0x02, 0x59) => "RXM-RLM",
        (0x02, 0x61) => "RXM-IMES",
        // INF (0x04)
        (0x04, 0x00) => "INF-ERROR",
        (0x04, 0x01) => "INF-WARNING",
        (0x04, 0x02) => "INF-NOTICE",
        (0x04, 0x03) => "INF-TEST",
        (0x04, 0x04) => "INF-DEBUG",
        // ACK (0x05)
        (0x05, 0x00) => "ACK-NAK",
        (0x05, 0x01) => "ACK-ACK",
        // CFG (0x06)
        (0x06, 0x00) => "CFG-PRT",
        (0x06, 0x01) => "CFG-MSG",
        (0x06, 0x04) => "CFG-RST",
        (0x06, 0x06) => "CFG-DAT",
        (0x06, 0x08) => "CFG-RATE",
        (0x06, 0x09) => "CFG-CFG",
        (0x06, 0x13) => "CFG-ANT",
        (0x06, 0x16) => "CFG-SBAS",
        (0x06, 0x17) => "CFG-NMEA",
        (0x06, 0x1B) => "CFG-USB",
        (0x06, 0x23) => "CFG-NAVX5",
        (0x06, 0x24) => "CFG-NAV5",
        (0x06, 0x31) => "CFG-TP5",
        (0x06, 0x34) => "CFG-RINV",
        (0x06, 0x39) => "CFG-ITFM",
        (0x06, 0x3B) => "CFG-PM2",
        (0x06, 0x3D) => "CFG-TMODE2",
        (0x06, 0x3E) => "CFG-GNSS",
        (0x06, 0x47) => "CFG-LOGFILTER",
        (0x06, 0x53) => "CFG-TXSLOT",
        (0x06, 0x57) => "CFG-PWR",
        (0x06, 0x62) => "CFG-DGNSS",
        (0x06, 0x69) => "CFG-GEOFENCE",
        (0x06, 0x70) => "CFG-FIXSEED",
        (0x06, 0x71) => "CFG-TMODE3",
        (0x06, 0x84) => "CFG-PMS",
        (0x06, 0x86) => "CFG-VALDEL",
        (0x06, 0x8A) => "CFG-VALSET",
        (0x06, 0x8B) => "CFG-VALGET",
        // UPD (0x09)
        (0x09, 0x14) => "UPD-SOS",
        // MON (0x0A)
        (0x0A, 0x04) => "MON-VER",
        (0x0A, 0x09) => "MON-HW",
        (0x0A, 0x0B) => "MON-HW2",
        (0x0A, 0x21) => "MON-RXR",
        (0x0A, 0x27) => "MON-PATCH",
        (0x0A, 0x28) => "MON-GNSS",
        (0x0A, 0x2E) => "MON-SMGR",
        (0x0A, 0x31) => "MON-SPAN",
        (0x0A, 0x32) => "MON-TXBUF",
        (0x0A, 0x36) => "MON-COMMS",
        (0x0A, 0x37) => "MON-HW3",
        (0x0A, 0x38) => "MON-RF",
        // TIM (0x0D)
        (0x0D, 0x01) => "TIM-TP",
        (0x0D, 0x03) => "TIM-TM2",
        (0x0D, 0x04) => "TIM-SVIN",
        (0x0D, 0x06) => "TIM-VRFY",
        // ESF (0x10)
        (0x10, 0x02) => "ESF-MEAS",
        (0x10, 0x03) => "ESF-RAW",
        (0x10, 0x10) => "ESF-STATUS",
        (0x10, 0x14) => "ESF-ALG",
        (0x10, 0x15) => "ESF-INS",
        // MGA (0x13)
        (0x13, 0x00) => "MGA-GPS",
        (0x13, 0x02) => "MGA-GAL",
        (0x13, 0x03) => "MGA-BDS",
        (0x13, 0x05) => "MGA-QZSS",
        (0x13, 0x06) => "MGA-GLO",
        (0x13, 0x20) => "MGA-ANO",
        (0x13, 0x21) => "MGA-FLASH",
        (0x13, 0x40) => "MGA-INI",
        (0x13, 0x60) => "MGA-ACK",
        (0x13, 0x80) => "MGA-DBD",
        // LOG (0x21)
        (0x21, 0x03) => "LOG-ERASE",
        (0x21, 0x04) => "LOG-STRING",
        (0x21, 0x07) => "LOG-CREATE",
        (0x21, 0x08) => "LOG-INFO",
        (0x21, 0x09) => "LOG-RETRIEVE",
        (0x21, 0x0B) => "LOG-RETRIEVEPOS",
        (0x21, 0x0D) => "LOG-RETRIEVEPOSEXTRA",
        (0x21, 0x0E) => "LOG-FINDTIME",
        (0x21, 0x0F) => "LOG-RETRIEVESTRING",
        // SEC (0x27)
        (0x27, 0x01) => "SEC-SIGN",
        (0x27, 0x03) => "SEC-UNIQID",
        // HNR (0x28) - High Navigation Rate
        (0x28, 0x00) => "HNR-PVT",
        (0x28, 0x01) => "HNR-ATT",
        (0x28, 0x02) => "HNR-INS",
        // RTCM (0xF5)
        (0xF5, _) => "RTCM3",
        // 不明
        _ => "UNKNOWN",
    }
}

/// 現在の無効化リストに含まれているかチェック
fn is_in_disable_list(class: u8, id: u8) -> bool {
    // cfg_valset.rsのdisable_periodic_output()で無効化しているメッセージ
    matches!(
        (class, id),
        // ポーリング対象（元の6メッセージ）
        (0x01, 0x07) | // NAV-PVT
        (0x01, 0x03) | // NAV-STATUS
        (0x01, 0x35) | // NAV-SAT
        (0x01, 0x43) | // NAV-SIG
        (0x0A, 0x31) | // MON-SPAN
        (0x0A, 0x38) | // MON-RF
        // Session 199-201で追加
        (0x01, 0x22) | // NAV-CLOCK
        (0x01, 0x02) | // NAV-POSLLH
        (0x01, 0x13) | // NAV-HPPOSECEF
        (0x01, 0x20) | // NAV-TIMEGPS
        (0x01, 0x27) | // NAV-TIMEQZSS
        (0x01, 0x32)   // NAV-SBAS
    )
}

// ===========================================
// API レスポンス型
// ===========================================

/// 受信したメッセージ情報
#[derive(Debug, Clone, Serialize)]
pub struct ReceivedMessage {
    /// メッセージクラス（16進表記）
    pub class: String,
    /// メッセージID（16進表記）
    pub id: String,
    /// メッセージ名
    pub name: String,
    /// 受信回数
    pub count: u32,
    /// 無効化リストに含まれているか
    pub in_disable_list: bool,
}

/// メッセージスキャン結果
#[derive(Debug, Serialize)]
pub struct MessageScanResponse {
    /// スキャン時間（秒）
    pub scan_duration_sec: f64,
    /// 受信したメッセージ一覧
    pub received_messages: Vec<ReceivedMessage>,
    /// 無効化されていないメッセージ（対応が必要）
    pub not_disabled: Vec<String>,
    /// 総受信メッセージ数
    pub total_count: u32,
}

// ===========================================
// APIハンドラー
// ===========================================

/// GET /api/devices/{path}/message-scan - メッセージスキャン実行
///
/// 3秒間受信バッファをモニタリングし、流れてくるUBXメッセージを収集
pub async fn scan_messages(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let port_path = urlencoding::decode(&path.into_inner())
        .unwrap_or_else(|_| std::borrow::Cow::Borrowed(""))
        .to_string();

    // デバイスマネージャー取得
    let device_manager_arc = match data.get_device_manager_by_path(&port_path) {
        Ok(Some(manager)) => manager,
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

    // スキャン実行（3秒間）
    let scan_duration = Duration::from_secs(3);
    let mut message_counts: HashMap<(u8, u8), u32> = HashMap::new();
    let start = Instant::now();

    // メッセージ収集ループ
    while start.elapsed() < scan_duration {
        let mut manager = match device_manager_arc.lock() {
            Ok(m) => m,
            Err(_) => {
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "デバイスロック取得に失敗".to_string(),
                    code: "LOCK_ERROR".to_string(),
                });
            }
        };

        // バッファを読み取り
        match manager.receive_ubx(Duration::from_millis(200)) {
            Ok(data) => {
                // UBXメッセージをパース
                if data.len() >= 6 && data[0] == UBX_SYNC_1 && data[1] == UBX_SYNC_2 {
                    let class = data[2];
                    let id = data[3];
                    *message_counts.entry((class, id)).or_insert(0) += 1;
                }
            }
            Err(_) => {
                // タイムアウトは無視
            }
        }

        // ロックを解放してから少し待つ
        drop(manager);
        std::thread::sleep(Duration::from_millis(10));
    }

    // 結果を整形
    let mut received_messages: Vec<ReceivedMessage> = message_counts
        .iter()
        .map(|((class, id), count)| {
            let name = get_message_name(*class, *id);
            ReceivedMessage {
                class: format!("0x{:02X}", class),
                id: format!("0x{:02X}", id),
                name: name.to_string(),
                count: *count,
                in_disable_list: is_in_disable_list(*class, *id),
            }
        })
        .collect();

    // カウント降順でソート
    received_messages.sort_by(|a, b| b.count.cmp(&a.count));

    // 無効化されていないメッセージを抽出
    let not_disabled: Vec<String> = received_messages
        .iter()
        .filter(|m| !m.in_disable_list && m.name != "UNKNOWN" && m.name != "ACK-ACK" && m.name != "ACK-NAK")
        .map(|m| m.name.clone())
        .collect();

    let total_count: u32 = received_messages.iter().map(|m| m.count).sum();

    HttpResponse::Ok().json(MessageScanResponse {
        scan_duration_sec: start.elapsed().as_secs_f64(),
        received_messages,
        not_disabled,
        total_count,
    })
}
