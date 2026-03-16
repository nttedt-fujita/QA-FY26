//! 装置管理API
//!
//! - GET /api/devices - 装置一覧取得
//! - POST /api/devices/{path}/connect - 接続（自動検出）
//! - POST /api/devices/{path}/disconnect - 切断

use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use std::sync::Mutex;
use std::time::Duration;

use crate::device::filter::PortInfo;
use crate::device::manager::{DeviceManager, DeviceManagerError, SerialPortProvider};
use crate::device::multi_manager::MultiDeviceManager;
use crate::device::status::DeviceStatus;
use crate::repository::SqliteRepository;
use crate::ubx::cfg_valset::{disable_periodic_output, set_uart1_nmea_output, Layer};
use crate::ubx::ack;

// ===========================================
// API レスポンス型
// ===========================================

/// 装置情報（API用）
#[derive(Debug, Clone, Serialize)]
pub struct DeviceResponse {
    /// ポートパス（例: /dev/ttyACM0）
    pub path: String,
    /// Vendor ID（16進表記）
    pub vid: Option<String>,
    /// Product ID（16進表記）
    pub pid: Option<String>,
    /// USBシリアル番号（FTDIチップ等）- 参考用
    pub serial_number: Option<String>,
    /// F9Pチップのシリアル番号（UBX-SEC-UNIQID）- DB紐付け用
    pub f9p_serial: Option<String>,
    /// 状態
    pub status: String,
    /// ボーレート（接続時のみ）
    pub baud_rate: Option<u32>,
}

impl DeviceResponse {
    /// PortInfoとDeviceStatusからAPIレスポンス用に変換
    pub fn from_port_info(
        port: &PortInfo,
        status: &DeviceStatus,
        baud_rate: Option<u32>,
        f9p_serial: Option<String>,
    ) -> Self {
        Self {
            path: port.path.clone(),
            vid: port.vid.map(|v| format!("{:04X}", v)),
            pid: port.pid.map(|p| format!("{:04X}", p)),
            serial_number: port.serial_number.clone(),
            f9p_serial,
            status: status.to_string(),
            baud_rate,
        }
    }
}

/// 装置一覧レスポンス
#[derive(Debug, Serialize)]
pub struct DeviceListResponse {
    pub devices: Vec<DeviceResponse>,
}

/// 接続成功レスポンス
#[derive(Debug, Serialize)]
pub struct ConnectResponse {
    pub path: String,
    pub baud_rate: u32,
    pub message: String,
}

/// エラーレスポンス
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
}

// ===========================================
// アプリケーション状態
// ===========================================

/// シリアルポートプロバイダー（本番用）
///
/// 状態を持たないため Clone 可能（MultiDeviceManager で必要）
#[derive(Clone)]
pub struct RealSerialPortProvider;

impl crate::device::manager::SerialPortProvider for RealSerialPortProvider {
    fn available_ports(&self) -> Result<Vec<PortInfo>, DeviceManagerError> {
        let ports = serialport::available_ports().map_err(|e| {
            DeviceManagerError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.description))
        })?;

        Ok(ports
            .into_iter()
            .map(|p| {
                let (vid, pid, serial_number) = match &p.port_type {
                    serialport::SerialPortType::UsbPort(usb) => {
                        (Some(usb.vid), Some(usb.pid), usb.serial_number.clone())
                    }
                    _ => (None, None, None),
                };
                PortInfo {
                    path: p.port_name,
                    vid,
                    pid,
                    serial_number,
                }
            })
            .collect())
    }

    fn open(
        &self,
        path: &str,
        baud_rate: u32,
    ) -> Result<Box<dyn crate::device::manager::SerialPort>, DeviceManagerError> {
        // Session 163: RealSerialPortラッパーを削除し、serialportクレートのBoxを直接返す
        let port = serialport::new(path, baud_rate)
            .timeout(std::time::Duration::from_millis(100))
            .open()
            .map_err(|e| match e.kind {
                serialport::ErrorKind::NoDevice => {
                    DeviceManagerError::PortNotFound(path.to_string())
                }
                serialport::ErrorKind::Io(std::io::ErrorKind::PermissionDenied) => {
                    DeviceManagerError::PortBusy(path.to_string())
                }
                _ => DeviceManagerError::IoError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    e.description,
                )),
            })?;

        Ok(port)
    }
}

/// 現在位置（緯度、経度）
#[derive(Debug, Clone, Copy, Default)]
pub struct CurrentPosition {
    pub lat: f64,
    pub lon: f64,
    /// 有効フラグ（NAV-PVTが取得できた場合にtrue）
    pub valid: bool,
}

/// APIで共有するアプリケーション状態
pub struct AppState {
    /// 複数デバイスマネージャー（Phase 3: 複数台同時対応）
    pub multi_device_manager: Mutex<MultiDeviceManager<RealSerialPortProvider>>,
    pub repository: Mutex<SqliteRepository>,
    /// 現在位置（NTRIP GGA送信用）
    pub current_position: Mutex<CurrentPosition>,
}

impl AppState {
    pub fn new() -> Self {
        // DBファイルはカレントディレクトリに作成
        let repo = SqliteRepository::new("gnss_evaluation.db")
            .expect("データベース初期化に失敗");
        Self {
            multi_device_manager: Mutex::new(MultiDeviceManager::new(RealSerialPortProvider)),
            repository: Mutex::new(repo),
            current_position: Mutex::new(CurrentPosition::default()),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

use std::sync::Arc;

impl AppState {
    /// 最初に接続したデバイスのマネージャーを取得（後方互換性用）
    ///
    /// Phase 3以前のAPIは1台のみ対応だったため、
    /// 複数台対応後も既存APIは最初の接続デバイスを使用する
    pub fn get_first_device_manager(
        &self,
    ) -> Result<Option<Arc<Mutex<DeviceManager<RealSerialPortProvider>>>>, DeviceManagerError> {
        let multi_manager = self.multi_device_manager.lock().map_err(|_| {
            DeviceManagerError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "ロック取得に失敗",
            ))
        })?;

        let paths = multi_manager.connected_paths();
        if let Some(first_path) = paths.first() {
            Ok(multi_manager.get_manager(first_path))
        } else {
            Ok(None)
        }
    }

    /// パス指定でデバイスマネージャーを取得（Phase 3: 複数台対応）
    ///
    /// 指定したパスのデバイスが接続されていない場合はNoneを返す
    pub fn get_device_manager_by_path(
        &self,
        path: &str,
    ) -> Result<Option<Arc<Mutex<DeviceManager<RealSerialPortProvider>>>>, DeviceManagerError> {
        let multi_manager = self.multi_device_manager.lock().map_err(|_| {
            DeviceManagerError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "ロック取得に失敗",
            ))
        })?;

        Ok(multi_manager.get_manager(path))
    }
}

// ===========================================
// APIハンドラー
// ===========================================

/// GET /api/devices - 装置一覧取得
pub async fn list_devices(data: web::Data<AppState>) -> impl Responder {
    let multi_manager = match data.multi_device_manager.lock() {
        Ok(m) => m,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: ロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            })
        }
    };

    match multi_manager.list_devices() {
        Ok(devices) => {
            let response: Vec<DeviceResponse> = devices
                .iter()
                .map(|d| {
                    // 接続状態に応じてステータスを設定
                    let status = if d.connected {
                        DeviceStatus::Connected
                    } else {
                        DeviceStatus::Detected
                    };

                    // F9Pシリアルは接続中のデバイスマネージャーから取得
                    let f9p_serial = if d.connected {
                        if let Some(manager_arc) = multi_manager.get_manager(&d.path) {
                            if let Ok(manager) = manager_arc.lock() {
                                manager.f9p_serial().map(|s| s.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    DeviceResponse::from_port_info(&d.port, &status, d.baud_rate, f9p_serial)
                })
                .collect();

            HttpResponse::Ok().json(DeviceListResponse { devices: response })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
            code: "DEVICE_LIST_ERROR".to_string(),
        }),
    }
}

/// POST /api/devices/{path}/connect - 接続（自動検出）
///
/// パスはURLエンコードされている前提（例: %2Fdev%2FttyACM0）
pub async fn connect_device(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let port_path = urlencoding::decode(&path.into_inner())
        .unwrap_or_else(|_| std::borrow::Cow::Borrowed(""))
        .to_string();

    // Phase 3: MultiDeviceManagerを使用
    let mut multi_manager = match data.multi_device_manager.lock() {
        Ok(m) => m,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: ロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            })
        }
    };

    // MultiDeviceManagerで接続（内部でスキャン+自動検出接続を行う）
    match multi_manager.connect(&port_path) {
        Ok(baud_rate) => {
            // 接続後、個別のDeviceManagerを取得して追加設定を行う
            let device_manager_arc = multi_manager.get_manager(&port_path);
            if let Some(device_manager_arc) = device_manager_arc {
                let mut manager = match device_manager_arc.lock() {
                    Ok(m) => m,
                    Err(_) => {
                        return HttpResponse::InternalServerError().json(ErrorResponse {
                            error: "内部エラー: デバイスロック取得に失敗".to_string(),
                            code: "LOCK_ERROR".to_string(),
                        })
                    }
                };

                // ボーレートを115200に統一（Session 165）
                use crate::device::manager::DEFAULT_BAUD_RATE;
                let final_baud_rate = if baud_rate != DEFAULT_BAUD_RATE {
                    tracing::info!(
                        "ボーレートを{}bps → {}bpsに変更します",
                        baud_rate, DEFAULT_BAUD_RATE
                    );
                    match manager.upgrade_baud_rate(&port_path, DEFAULT_BAUD_RATE) {
                        Ok(()) => {
                            tracing::info!("ボーレート変更完了: {}bps", DEFAULT_BAUD_RATE);
                            DEFAULT_BAUD_RATE
                        }
                        Err(e) => {
                            tracing::warn!("ボーレート変更に失敗: {} (継続)", e);
                            baud_rate
                        }
                    }
                } else {
                    baud_rate
                };

                // 定期出力を無効化（ポーリング専用）
                if let Err(e) = send_disable_periodic_output(&mut manager) {
                    tracing::warn!("定期出力無効化に失敗: {}", e);
                } else {
                    tracing::debug!("定期出力を無効化しました");
                }

                // NMEA出力を無効化
                if let Err(e) = send_disable_nmea_output(&mut manager) {
                    tracing::warn!("NMEA出力無効化に失敗: {}", e);
                } else {
                    tracing::debug!("NMEA出力を無効化しました");
                }

                // F9Pシリアル番号を取得（定期出力無効化後に実行）
                if let Err(e) = manager.query_f9p_serial() {
                    tracing::warn!("F9Pシリアル番号の取得に失敗: {}", e);
                }

                return HttpResponse::Ok().json(ConnectResponse {
                    path: port_path,
                    baud_rate: final_baud_rate,
                    message: format!("接続成功（ボーレート: {} bps）", final_baud_rate),
                });
            }

            // マネージャー取得失敗（通常は到達しない）
            HttpResponse::Ok().json(ConnectResponse {
                path: port_path,
                baud_rate,
                message: format!("接続成功（ボーレート: {} bps）", baud_rate),
            })
        }
        Err(e) => {
            let code = match &e {
                DeviceManagerError::PortNotFound(_) => "PORT_NOT_FOUND",
                DeviceManagerError::PortBusy(_) => "PORT_BUSY",
                DeviceManagerError::MaxDevicesReached => "MAX_DEVICES",
                DeviceManagerError::Timeout => "TIMEOUT",
                _ => "CONNECT_ERROR",
            };
            let response = ErrorResponse {
                error: e.to_string(),
                code: code.to_string(),
            };
            match &e {
                DeviceManagerError::PortNotFound(_) => HttpResponse::NotFound().json(response),
                DeviceManagerError::PortBusy(_) => HttpResponse::Conflict().json(response),
                DeviceManagerError::MaxDevicesReached => HttpResponse::Conflict().json(response),
                DeviceManagerError::Timeout => HttpResponse::GatewayTimeout().json(response),
                _ => HttpResponse::InternalServerError().json(response),
            }
        }
    }
}

/// POST /api/devices/{path}/disconnect - 切断
pub async fn disconnect_device(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let port_path = urlencoding::decode(&path.into_inner())
        .unwrap_or_else(|_| std::borrow::Cow::Borrowed(""))
        .to_string();

    let mut multi_manager = match data.multi_device_manager.lock() {
        Ok(m) => m,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: ロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            })
        }
    };

    // Phase 3: パス指定で切断
    match multi_manager.disconnect(&port_path) {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({
            "message": "切断しました"
        })),
        Err(e) => {
            let code = match &e {
                DeviceManagerError::NotConnected => "NOT_CONNECTED",
                _ => "DISCONNECT_ERROR",
            };
            let response = ErrorResponse {
                error: e.to_string(),
                code: code.to_string(),
            };
            match &e {
                DeviceManagerError::NotConnected => HttpResponse::BadRequest().json(response),
                _ => HttpResponse::InternalServerError().json(response),
            }
        }
    }
}

// ===========================================
// 定期出力設定
// ===========================================

/// 定期出力を無効化
///
/// CFG-VALSETを送信して、各メッセージの定期出力を停止する
/// 統合APIは順次ポーリングで動作するため、定期出力は不要
fn send_disable_periodic_output<P: SerialPortProvider>(
    manager: &mut DeviceManager<P>,
) -> Result<(), DeviceManagerError> {
    // 全メッセージの定期出力を無効化（レート=0）
    // Session 149: BBRに書き込んで不揮発性に
    // Session 151: BBRのみでは即座に有効にならないため、RAM+BBRに変更
    let msg = disable_periodic_output(Layer::RamAndBbr);

    // バッファをクリア
    manager.drain_buffer()?;

    // 送信
    manager.send_ubx(&msg)?;

    // ACK待ち
    std::thread::sleep(Duration::from_millis(50));
    let response = manager.receive_ubx(Duration::from_secs(1))?;

    // ACK/NAK確認
    let ack_result = ack::parse_ack(&response);
    match ack_result {
        ack::AckResult::Ack { .. } => {
            tracing::debug!("CFG-VALSET ACK received");
            Ok(())
        }
        ack::AckResult::Nak { .. } => {
            tracing::warn!("CFG-VALSET NAK received");
            Err(DeviceManagerError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "CFG-VALSET rejected by device",
            )))
        }
        _ => {
            tracing::warn!("Unexpected response to CFG-VALSET: {:?}", ack_result);
            // ACK以外の応答も許容（定期出力が始まるとすぐにデータが来る）
            Ok(())
        }
    }
}

/// NMEA出力を無効化
///
/// CFG-VALSETを送信して、UART1のNMEA出力を停止する
/// Session 146: NMEAデータがUBXポーリングを妨害する問題を修正
/// Session 147: 統合APIからも呼び出せるようpubに変更
pub fn send_disable_nmea_output<P: SerialPortProvider>(
    manager: &mut DeviceManager<P>,
) -> Result<(), DeviceManagerError> {
    // NMEA出力を無効化
    let msg = set_uart1_nmea_output(false, Layer::Ram);

    // バッファをクリア
    manager.drain_buffer()?;

    // 送信
    manager.send_ubx(&msg)?;

    // ACK待ち
    std::thread::sleep(Duration::from_millis(50));
    let response = manager.receive_ubx(Duration::from_secs(1))?;

    // ACK/NAK確認
    let ack_result = ack::parse_ack(&response);
    match ack_result {
        ack::AckResult::Ack { .. } => {
            tracing::debug!("NMEA OFF CFG-VALSET ACK received");
            Ok(())
        }
        ack::AckResult::Nak { .. } => {
            tracing::warn!("NMEA OFF CFG-VALSET NAK received");
            Err(DeviceManagerError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "CFG-VALSET rejected by device",
            )))
        }
        _ => {
            tracing::warn!("Unexpected response to NMEA OFF CFG-VALSET: {:?}", ack_result);
            Ok(())
        }
    }
}

/// APIルートを設定
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/devices")
            .route("", web::get().to(list_devices))
            .route("/{path}/connect", web::post().to(connect_device))
            .route("/{path}/disconnect", web::post().to(disconnect_device))
            // Phase 3: パス指定版API（/api/devices/{path}/gnss-state）
            .route("/{path}/gnss-state", web::get().to(super::gnss_state_api::get_gnss_state_by_path))
            // メッセージスキャンAPI
            .route("/{path}/message-scan", web::get().to(super::message_scan_api::scan_messages))
            // LED点滅API
            .route("/{path}/blink", web::post().to(super::blink_api::blink_device)),
    );
}
