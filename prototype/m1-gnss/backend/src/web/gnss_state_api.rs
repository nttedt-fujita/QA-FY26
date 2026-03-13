//! 統合API (`GET /api/gnss-state`)
//!
//! F9Pの現在状態を1回のAPIで全取得（ポーリング競合防止）
//! 対象: NAV-PVT, NAV-STATUS, NAV-SAT, NAV-SIG, MON-SPAN, MON-RF

use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

use crate::ubx::{nav_pvt, nav_sat, nav_sig, nav_status, mon_span, mon_rf};
use super::device_api::{AppState, ErrorResponse, send_disable_nmea_output};
use super::nav_sat_api::{NavSatResponse, SatelliteInfoResponse, SatelliteStatsResponse, GnssCountsResponse};
use super::nav_sig_api::{NavSigResponse, SignalInfoResponse, SignalStatsResponse};
use super::mon_span_api::{MonSpanResponse, SpanBlockResponse};

// ===========================================
// マクロ（使用箇所より先に定義）
// ===========================================

/// poll送信 → 受信 → パース
macro_rules! poll_and_parse {
    ($manager:expr, $class:expr, $id:expr, $name:expr, $parser:expr) => {{
        (|| -> Result<_, String> {
            // ドレイン
            if let Err(e) = $manager.drain_buffer() {
                return Err(format!("{}: バッファクリアエラー - {}", $name, e));
            }

            // 送信
            let poll_msg = build_poll($class, $id);
            if let Err(e) = $manager.send_ubx(&poll_msg) {
                tracing::error!("[GNSS-STATE:{}] 送信失敗: {}", $name, e);
                return Err(format!("{}: 送信エラー - {}", $name, e));
            }

            // 送信後バッファ空待ち（ポーリング）
            let wait_start = std::time::Instant::now();
            let timeout_ms: u128 = 10;
            let poll_interval_ms = 1;
            let bytes_before = $manager.get_bytes_to_write().unwrap_or(0);

            while wait_start.elapsed().as_millis() < timeout_ms {
                match $manager.get_bytes_to_write() {
                    Ok(0) => break,
                    Ok(_) => std::thread::sleep(std::time::Duration::from_millis(poll_interval_ms)),
                    Err(_) => break,
                }
            }

            let wait_elapsed = wait_start.elapsed();
            let bytes_after = $manager.get_bytes_to_write().unwrap_or(0);
            tracing::debug!(
                "[GNSS-STATE:{}] バッファ空待ち: 残量{}→{}bytes, 待機{:?}",
                $name, bytes_before, bytes_after, wait_elapsed
            );
            if wait_elapsed.as_millis() >= timeout_ms {
                tracing::warn!(
                    "[GNSS-STATE:{}] バッファ空待ちタイムアウト: 残量{}bytes",
                    $name, bytes_after
                );
            }

            // 受信（タイムアウト2秒）
            let raw = match $manager.receive_ubx(std::time::Duration::from_millis(2000)) {
                Ok(r) => r,
                Err(crate::device::manager::DeviceManagerError::Timeout) => {
                    tracing::warn!("[GNSS-STATE:{}] 受信タイムアウト", $name);
                    return Err(format!("{}: タイムアウト", $name));
                }
                Err(e) => {
                    tracing::error!("[GNSS-STATE:{}] 受信エラー: {}", $name, e);
                    return Err(format!("{}: 受信エラー - {}", $name, e));
                }
            };
            $parser(&raw).map_err(|e| format!("{}: パースエラー - {}", $name, e))
        })()
    }};
}

// ===========================================
// レスポンス型
// ===========================================

/// NAV-PVTレスポンス
#[derive(Debug, Clone, Serialize)]
pub struct NavPvtResponse {
    pub lat: f64,
    pub lon: f64,
    pub fix_type: u8,
    pub carr_soln: u8,
    pub num_sv: u8,
    pub h_acc: u32,
    pub v_acc: u32,
    pub is_rtk_fixed: bool,
    pub is_rtk_float: bool,
}

/// NAV-STATUSレスポンス
#[derive(Debug, Clone, Serialize)]
pub struct NavStatusResponse {
    pub ttff: u32,
    pub msss: u32,
    pub gps_fix: u8,
    pub gps_fix_ok: bool,
    pub is_fix_valid: bool,
    pub carr_soln: u8,
    pub is_rtk_fixed: bool,
    pub is_rtk_float: bool,
}

/// MON-RFブロックレスポンス
#[derive(Debug, Clone, Serialize)]
pub struct RfBlockResponse {
    pub block_id: u8,
    pub jamming_state: u8,
    pub ant_status: u8,
    pub ant_power: u8,
    pub noise_per_ms: u16,
    pub agc_cnt: u16,
    pub cw_suppression: u8,
    pub is_jamming_detected: bool,
    pub is_antenna_ok: bool,
}

/// MON-RFレスポンス
#[derive(Debug, Clone, Serialize)]
pub struct MonRfResponse {
    pub blocks: Vec<RfBlockResponse>,
    pub has_jamming: bool,
    pub has_critical_jamming: bool,
}

/// 統合APIレスポンス
#[derive(Debug, Clone, Serialize)]
pub struct GnssStateResponse {
    pub nav_pvt: Option<NavPvtResponse>,
    pub nav_status: Option<NavStatusResponse>,
    pub nav_sat: Option<NavSatResponse>,
    pub nav_sig: Option<NavSigResponse>,
    pub mon_span: Option<MonSpanResponse>,
    pub mon_rf: Option<MonRfResponse>,
    pub errors: Vec<String>,
}

// ===========================================
// UBX poll構築
// ===========================================

fn calculate_checksum(data: &[u8]) -> (u8, u8) {
    let mut ck_a: u8 = 0;
    let mut ck_b: u8 = 0;
    for &byte in data {
        ck_a = ck_a.wrapping_add(byte);
        ck_b = ck_b.wrapping_add(ck_a);
    }
    (ck_a, ck_b)
}

fn build_poll(class: u8, id: u8) -> Vec<u8> {
    let payload: &[u8] = &[];
    let len = payload.len() as u16;
    let mut frame = vec![
        0xB5, 0x62,
        class, id,
        (len & 0xFF) as u8,
        (len >> 8) as u8,
    ];
    frame.extend_from_slice(payload);
    let (ck_a, ck_b) = calculate_checksum(&frame[2..]);
    frame.push(ck_a);
    frame.push(ck_b);
    frame
}

// ===========================================
// APIハンドラー
// ===========================================

/// GET /api/gnss-state - 統合API
pub async fn get_gnss_state(data: web::Data<AppState>) -> impl Responder {
    tracing::debug!("[GNSS-STATE] API呼び出し開始");
    let api_start = std::time::Instant::now();

    let lock_start = std::time::Instant::now();
    let mut manager = match data.device_manager.lock() {
        Ok(m) => {
            let lock_ms = lock_start.elapsed().as_millis();
            tracing::debug!("[GNSS-STATE] ロック取得 ({}ms)", lock_ms);
            if lock_ms > 100 {
                tracing::warn!("[GNSS-STATE] ⚠️ ロック待機が長い！({}ms) - NTRIP転送と競合の可能性", lock_ms);
            }
            m
        }
        Err(_) => {
            tracing::error!("[GNSS-STATE] ロック取得失敗（poisoned）");
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: ロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            })
        }
    };

    // 接続確認
    if manager.get_connected_device().is_none() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "装置が接続されていません".to_string(),
            code: "DEVICE_NOT_CONNECTED".to_string(),
        });
    }

    // ロック取得直後にバッファをドレイン（NTRIP転送後の残データ対策）
    if let Err(e) = manager.drain_buffer() {
        tracing::warn!("[GNSS-STATE] ドレインエラー（続行）: {}", e);
    }

    // バッファ空待ち（送信バッファが空になるまでポーリング）
    let stabilize_start = std::time::Instant::now();
    let timeout_ms: u128 = 500;
    let poll_interval_ms = 10;

    while stabilize_start.elapsed().as_millis() < timeout_ms {
        match manager.get_bytes_to_write() {
            Ok(0) => break,
            Ok(_) => std::thread::sleep(std::time::Duration::from_millis(poll_interval_ms)),
            Err(e) => {
                tracing::warn!("[GNSS-STATE] バッファ残量取得エラー（続行）: {}", e);
                break;
            }
        }
    }
    if stabilize_start.elapsed().as_millis() >= timeout_ms {
        let bytes_after = manager.get_bytes_to_write().unwrap_or(0);
        tracing::warn!(
            "[GNSS-STATE] バッファ空待ちタイムアウト（続行）: 残量{}bytes",
            bytes_after
        );
    }

    // NMEA出力を無効化（屋外検査用）
    if let Err(e) = send_disable_nmea_output(&mut manager) {
        tracing::warn!("[GNSS-STATE] NMEA OFF送信失敗（続行）: {}", e);
    }

    let mut response = GnssStateResponse {
        nav_pvt: None,
        nav_status: None,
        nav_sat: None,
        nav_sig: None,
        mon_span: None,
        mon_rf: None,
        errors: vec![],
    };

    // 6メッセージを順次取得
    // NAV-PVT (class=0x01, id=0x07)
    let msg_start = std::time::Instant::now();
    match poll_and_parse!(manager, 0x01, 0x07, "NAV-PVT", |raw: &[u8]| -> Result<NavPvtResponse, Box<dyn std::error::Error>> {
        let parsed = nav_pvt::parse(raw)?;
        Ok(NavPvtResponse {
            lat: parsed.lat,
            lon: parsed.lon,
            fix_type: parsed.fix_type,
            carr_soln: parsed.carr_soln,
            num_sv: parsed.num_sv,
            h_acc: parsed.h_acc,
            v_acc: parsed.v_acc,
            is_rtk_fixed: parsed.is_rtk_fixed(),
            is_rtk_float: parsed.is_rtk_float(),
        })
    }) {
        Ok(r) => {
            tracing::debug!("[GNSS-STATE] NAV-PVT成功 ({}ms)", msg_start.elapsed().as_millis());
            // 位置更新（NTRIP GGA送信用）
            if r.fix_type >= 2 {
                if let Ok(mut pos) = data.current_position.lock() {
                    pos.lat = r.lat;
                    pos.lon = r.lon;
                    pos.valid = true;
                    tracing::debug!("[GNSS-STATE] 位置更新: lat={}, lon={}", r.lat, r.lon);
                }
            }
            response.nav_pvt = Some(r);
        }
        Err(e) => {
            tracing::warn!("[GNSS-STATE] NAV-PVT失敗: {} ({}ms)", e, msg_start.elapsed().as_millis());
            response.errors.push(e);
        }
    }

    // NAV-STATUS (class=0x01, id=0x03)
    let msg_start = std::time::Instant::now();
    match poll_and_parse!(manager, 0x01, 0x03, "NAV-STATUS", |raw: &[u8]| -> Result<NavStatusResponse, Box<dyn std::error::Error>> {
        let parsed = nav_status::parse(raw)?;
        Ok(NavStatusResponse {
            ttff: parsed.ttff,
            msss: parsed.msss,
            gps_fix: parsed.gps_fix,
            gps_fix_ok: parsed.gps_fix_ok,
            is_fix_valid: parsed.is_fix_valid(),
            carr_soln: parsed.carr_soln,
            is_rtk_fixed: parsed.is_rtk_fixed(),
            is_rtk_float: parsed.is_rtk_float(),
        })
    }) {
        Ok(r) => {
            tracing::debug!("[GNSS-STATE] NAV-STATUS成功 ({}ms)", msg_start.elapsed().as_millis());
            response.nav_status = Some(r);
        }
        Err(e) => {
            tracing::warn!("[GNSS-STATE] NAV-STATUS失敗: {} ({}ms)", e, msg_start.elapsed().as_millis());
            response.errors.push(e);
        }
    }

    // NAV-SAT (class=0x01, id=0x35)
    let msg_start = std::time::Instant::now();
    match poll_and_parse!(manager, 0x01, 0x35, "NAV-SAT", |raw: &[u8]| -> Result<NavSatResponse, Box<dyn std::error::Error>> {
        let parsed = nav_sat::parse(raw)?;
        let used_count = parsed.used_satellites().len();
        let gnss_counts = GnssCountsResponse {
            gps: parsed.satellites_by_gnss(0).len(),
            sbas: parsed.satellites_by_gnss(1).len(),
            galileo: parsed.satellites_by_gnss(2).len(),
            beidou: parsed.satellites_by_gnss(3).len(),
            qzss: parsed.satellites_by_gnss(5).len(),
            glonass: parsed.satellites_by_gnss(6).len(),
        };
        let stats = SatelliteStatsResponse {
            total_count: parsed.satellites.len(),
            used_count,
            gnss_counts,
        };
        let satellites: Vec<SatelliteInfoResponse> =
            parsed.satellites.iter().map(|s| s.into()).collect();
        Ok(NavSatResponse { satellites, stats })
    }) {
        Ok(r) => {
            tracing::debug!("[GNSS-STATE] NAV-SAT成功 ({}ms)", msg_start.elapsed().as_millis());
            response.nav_sat = Some(r);
        }
        Err(e) => {
            tracing::warn!("[GNSS-STATE] NAV-SAT失敗: {} ({}ms)", e, msg_start.elapsed().as_millis());
            response.errors.push(e);
        }
    }

    // NAV-SIG (class=0x01, id=0x43)
    let msg_start = std::time::Instant::now();
    match poll_and_parse!(manager, 0x01, 0x43, "NAV-SIG", |raw: &[u8]| -> Result<NavSigResponse, Box<dyn std::error::Error>> {
        let parsed = nav_sig::parse(raw)?;
        let gps_l1_visible: Vec<_> = parsed.signals.iter()
            .filter(|s| s.gnss_id == 0 && s.is_l1())
            .collect();
        let gps_l2_receiving: Vec<_> = parsed.signals.iter()
            .filter(|s| s.gnss_id == 0 && s.is_l2() && s.cno > 0)
            .collect();
        let gps_l2_rate = if gps_l1_visible.is_empty() {
            0.0
        } else {
            gps_l2_receiving.len() as f64 / gps_l1_visible.len() as f64
        };
        let stats = SignalStatsResponse {
            gps_visible_count: gps_l1_visible.len(),
            gps_l2_reception_count: gps_l2_receiving.len(),
            gps_l2_reception_rate: gps_l2_rate,
        };
        let signals: Vec<SignalInfoResponse> =
            parsed.signals.iter().map(|s| s.into()).collect();
        Ok(NavSigResponse { signals, stats })
    }) {
        Ok(r) => {
            tracing::debug!("[GNSS-STATE] NAV-SIG成功 ({}ms)", msg_start.elapsed().as_millis());
            response.nav_sig = Some(r);
        }
        Err(e) => {
            tracing::warn!("[GNSS-STATE] NAV-SIG失敗: {} ({}ms)", e, msg_start.elapsed().as_millis());
            response.errors.push(e);
        }
    }

    // MON-SPAN (class=0x0A, id=0x31)
    let msg_start = std::time::Instant::now();
    match poll_and_parse!(manager, 0x0A, 0x31, "MON-SPAN", |raw: &[u8]| -> Result<MonSpanResponse, Box<dyn std::error::Error>> {
        let parsed = mon_span::parse(raw)?;
        let blocks: Vec<SpanBlockResponse> = parsed.blocks.iter().map(|b| {
            SpanBlockResponse {
                spectrum: b.spectrum.to_vec(),
                span: b.span,
                res: b.res,
                center: b.center,
                pga: b.pga,
                max_amplitude: *b.spectrum.iter().max().unwrap_or(&0),
                avg_amplitude: b.spectrum.iter().map(|&x| x as f32).sum::<f32>() / 256.0,
            }
        }).collect();
        Ok(MonSpanResponse { blocks })
    }) {
        Ok(r) => {
            tracing::debug!("[GNSS-STATE] MON-SPAN成功 ({}ms)", msg_start.elapsed().as_millis());
            response.mon_span = Some(r);
        }
        Err(e) => {
            tracing::warn!("[GNSS-STATE] MON-SPAN失敗: {} ({}ms)", e, msg_start.elapsed().as_millis());
            response.errors.push(e);
        }
    }

    // MON-RF (class=0x0A, id=0x38)
    let msg_start = std::time::Instant::now();
    match poll_and_parse!(manager, 0x0A, 0x38, "MON-RF", |raw: &[u8]| -> Result<MonRfResponse, Box<dyn std::error::Error>> {
        let parsed = mon_rf::parse(raw)?;
        let blocks: Vec<RfBlockResponse> = parsed.blocks.iter().map(|b| {
            RfBlockResponse {
                block_id: b.block_id,
                jamming_state: b.jamming_state,
                ant_status: b.ant_status,
                ant_power: b.ant_power,
                noise_per_ms: b.noise_per_ms,
                agc_cnt: b.agc_cnt,
                cw_suppression: b.cw_suppression,
                is_jamming_detected: b.is_jamming_detected(),
                is_antenna_ok: b.is_antenna_ok(),
            }
        }).collect();
        let has_jamming = blocks.iter().any(|b| b.is_jamming_detected);
        let has_critical_jamming = blocks.iter().any(|b| b.jamming_state == 3);
        Ok(MonRfResponse { blocks, has_jamming, has_critical_jamming })
    }) {
        Ok(r) => {
            tracing::debug!("[GNSS-STATE] MON-RF成功 ({}ms)", msg_start.elapsed().as_millis());
            response.mon_rf = Some(r);
        }
        Err(e) => {
            tracing::warn!("[GNSS-STATE] MON-RF失敗: {} ({}ms)", e, msg_start.elapsed().as_millis());
            response.errors.push(e);
        }
    }

    tracing::debug!("[GNSS-STATE] 全メッセージ取得完了 (総時間: {}ms, エラー数: {})",
        api_start.elapsed().as_millis(), response.errors.len());

    // 全失敗チェック
    if response.nav_pvt.is_none()
        && response.nav_status.is_none()
        && response.nav_sat.is_none()
        && response.nav_sig.is_none()
        && response.mon_span.is_none()
        && response.mon_rf.is_none()
    {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: "全メッセージの取得に失敗".to_string(),
            code: "ALL_MESSAGES_FAILED".to_string(),
        });
    }

    HttpResponse::Ok().json(response)
}

/// APIルートを設定
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/gnss-state")
            .route("", web::get().to(get_gnss_state)),
    );
}
