//! 装置管理API
//!
//! - GET /api/devices - 装置一覧取得
//! - POST /api/devices/{path}/connect - 接続（自動検出）
//! - POST /api/devices/{path}/disconnect - 切断

use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use std::sync::Mutex;

use crate::device::filter::PortInfo;
use crate::device::manager::{DeviceManager, DeviceManagerError};
use crate::device::status::DeviceStatus;

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
    /// シリアル番号
    pub serial_number: Option<String>,
    /// 状態
    pub status: String,
    /// ボーレート（接続時のみ）
    pub baud_rate: Option<u32>,
}

impl DeviceResponse {
    /// PortInfoとDeviceStatusからAPIレスポンス用に変換
    pub fn from_port_info(port: &PortInfo, status: &DeviceStatus, baud_rate: Option<u32>) -> Self {
        Self {
            path: port.path.clone(),
            vid: port.vid.map(|v| format!("{:04X}", v)),
            pid: port.pid.map(|p| format!("{:04X}", p)),
            serial_number: port.serial_number.clone(),
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

        Ok(Box::new(RealSerialPort { port }))
    }
}

/// シリアルポート実装（本番用）
struct RealSerialPort {
    port: Box<dyn serialport::SerialPort>,
}

impl crate::device::manager::SerialPort for RealSerialPort {
    fn write(&mut self, data: &[u8]) -> Result<usize, std::io::Error> {
        std::io::Write::write(&mut self.port, data)
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        std::io::Read::read(&mut self.port, buf)
    }

    fn set_timeout(&mut self, timeout: std::time::Duration) -> Result<(), std::io::Error> {
        self.port.set_timeout(timeout).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, e.description)
        })
    }
}

/// APIで共有するアプリケーション状態
pub struct AppState {
    pub device_manager: Mutex<DeviceManager<RealSerialPortProvider>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            device_manager: Mutex::new(DeviceManager::new(RealSerialPortProvider)),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

// ===========================================
// APIハンドラー
// ===========================================

/// GET /api/devices - 装置一覧取得
pub async fn list_devices(data: web::Data<AppState>) -> impl Responder {
    let mut manager = match data.device_manager.lock() {
        Ok(m) => m,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: ロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            })
        }
    };

    match manager.list_devices() {
        Ok(devices) => {
            let connected_device = manager.get_connected_device();
            let detected_baud_rate = manager.detected_baud_rate();

            let response: Vec<DeviceResponse> = devices
                .iter()
                .map(|d| {
                    // 接続中のデバイスにはボーレートを含める
                    let baud_rate = if connected_device.map(|c| c.port.path.as_str()) == Some(&d.port.path) {
                        detected_baud_rate
                    } else {
                        None
                    };
                    DeviceResponse::from_port_info(&d.port, &d.status, baud_rate)
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

    let mut manager = match data.device_manager.lock() {
        Ok(m) => m,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: ロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            })
        }
    };

    // デバイスをスキャン（まだリストにない場合）
    if let Err(e) = manager.list_devices() {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
            code: "SCAN_ERROR".to_string(),
        });
    }

    // 自動検出で接続
    match manager.connect_auto_detect(&port_path) {
        Ok(baud_rate) => HttpResponse::Ok().json(ConnectResponse {
            path: port_path,
            baud_rate,
            message: format!("接続成功（ボーレート: {} bps）", baud_rate),
        }),
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
    _path: web::Path<String>,
) -> impl Responder {
    let mut manager = match data.device_manager.lock() {
        Ok(m) => m,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: ロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            })
        }
    };

    match manager.disconnect() {
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

/// APIルートを設定
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/devices")
            .route("", web::get().to(list_devices))
            .route("/{path}/connect", web::post().to(connect_device))
            .route("/{path}/disconnect", web::post().to(disconnect_device)),
    );
}
