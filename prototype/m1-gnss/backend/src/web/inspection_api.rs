//! 検査API
//!
//! - POST /api/inspections - 検査実行
//! - GET /api/inspections - 検査履歴取得

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::inspection::types::{ExpectedValue, ItemType, Verdict as EngineVerdict};
use crate::service::InspectionService;
use super::device_api::{AppState, ErrorResponse};

// ===========================================
// リクエスト/レスポンス型
// ===========================================

/// 検査実行リクエスト
#[derive(Debug, Deserialize)]
pub struct RunInspectionRequest {
    /// ロットID（任意）
    pub lot_id: Option<i64>,
}

/// 検査項目結果レスポンス
#[derive(Debug, Serialize)]
pub struct ItemResultResponse {
    pub item_name: String,
    pub verdict: String,
    pub actual_value: Option<String>,
    pub expected_value: Option<String>,
}

/// 検査実行レスポンス
#[derive(Debug, Serialize)]
pub struct InspectionResponse {
    pub inspection_id: i64,
    pub device_id: i64,
    pub serial_number: String,
    pub overall_result: String,
    pub items: Vec<ItemResultResponse>,
}

/// 検査履歴レスポンス（一覧用）
#[derive(Debug, Serialize)]
pub struct InspectionSummaryResponse {
    pub id: i64,
    pub device_id: i64,
    pub serial_number: Option<String>,
    pub inspected_at: String,
    pub overall_result: Option<String>,
}

/// 検査一覧レスポンス
#[derive(Debug, Serialize)]
pub struct InspectionListResponse {
    pub inspections: Vec<InspectionSummaryResponse>,
}

// ===========================================
// 一括検査用型（Phase 3: 複数台対応）
// ===========================================

/// 一括検査リクエスト
#[derive(Debug, Deserialize)]
pub struct BatchInspectionRequest {
    /// ロットID（任意）
    pub lot_id: Option<i64>,
}

/// 一括検査レスポンス
#[derive(Debug, Serialize)]
pub struct BatchInspectionResponse {
    pub results: Vec<DeviceInspectionResult>,
    pub summary: BatchSummary,
}

/// 個別デバイスの検査結果
#[derive(Debug, Serialize)]
pub struct DeviceInspectionResult {
    /// ポートパス（例: /dev/ttyUSB0）
    pub path: String,
    /// F9Pシリアル番号
    pub serial_number: String,
    /// 総合結果（Pass / Fail / Error）
    pub overall_result: String,
    /// 検査ID
    pub inspection_id: i64,
    /// 検査項目結果
    pub items: Vec<ItemResultResponse>,
}

/// 一括検査サマリー
#[derive(Debug, Serialize)]
pub struct BatchSummary {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
}

// ===========================================
// ヘルパー関数
// ===========================================

/// ItemType を文字列に変換
fn item_type_to_str(item_type: &ItemType) -> &'static str {
    match item_type {
        ItemType::Connectivity => "communication",
        ItemType::FwVersion => "fw",
        ItemType::SerialNumber => "serial",
        ItemType::OutputRate => "rate",
        ItemType::PortConfig => "port",
    }
}

/// EngineVerdict を文字列に変換
fn verdict_to_str(verdict: &EngineVerdict) -> &'static str {
    match verdict {
        EngineVerdict::Pass => "Pass",
        EngineVerdict::Fail => "Fail",
        EngineVerdict::Error(_) => "Error",
    }
}

/// ExpectedValue を Option<String> に変換
fn expected_to_string(expected: &ExpectedValue) -> Option<String> {
    match expected {
        ExpectedValue::None => None,
        ExpectedValue::String(s) => Some(s.clone()),
        ExpectedValue::Integer(i) => Some(i.to_string()),
    }
}

// ===========================================
// APIハンドラー
// ===========================================

/// POST /api/inspections - 検査実行
///
/// 接続中の装置に対して検査を実行し、結果をDBに保存する
pub async fn run_inspection(
    data: web::Data<AppState>,
    body: web::Json<RunInspectionRequest>,
) -> impl Responder {
    // Phase 3: MultiDeviceManager経由で最初の接続デバイスを取得
    let device_manager_arc = match data.get_first_device_manager() {
        Ok(Some(arc)) => arc,
        Ok(None) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "装置が接続されていません。先に接続してください。".to_string(),
                code: "DEVICE_NOT_CONNECTED".to_string(),
            });
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: デバイスマネージャーのロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            });
        }
    };

    let mut manager = match device_manager_arc.lock() {
        Ok(m) => m,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: デバイスマネージャーのロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            });
        }
    };

    let repo = match data.repository.lock() {
        Ok(r) => r,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: リポジトリのロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            });
        }
    };

    // InspectionServiceで検査実行
    let service = InspectionService::new(&repo);
    match service.run_and_save(&mut manager, body.lot_id) {
        Ok(report) => {
            // シリアル番号を取得（検査結果から）
            let serial_number = report.item_results.iter()
                .find(|r| matches!(r.item_type, ItemType::SerialNumber))
                .and_then(|r| r.actual_value.clone())
                .unwrap_or_else(|| "不明".to_string());

            // 項目結果をレスポンス用に変換
            let items: Vec<ItemResultResponse> = report.item_results.iter()
                .map(|r| {
                    ItemResultResponse {
                        item_name: item_type_to_str(&r.item_type).to_string(),
                        verdict: verdict_to_str(&r.verdict).to_string(),
                        actual_value: r.actual_value.clone(),
                        expected_value: expected_to_string(&r.expected_value),
                    }
                })
                .collect();

            HttpResponse::Created().json(InspectionResponse {
                inspection_id: report.inspection_id,
                device_id: report.device_id,
                serial_number,
                overall_result: report.overall_result,
                items,
            })
        }
        Err(e) => {
            let (code, mut status) = match &e {
                crate::service::ServiceError::DeviceNotConnected => {
                    ("DEVICE_NOT_CONNECTED", HttpResponse::BadRequest())
                }
                crate::service::ServiceError::SerialNumberNotFound => {
                    ("SERIAL_NOT_FOUND", HttpResponse::InternalServerError())
                }
                crate::service::ServiceError::Inspection(_) => {
                    ("INSPECTION_ERROR", HttpResponse::InternalServerError())
                }
                crate::service::ServiceError::Repository(_) => {
                    ("DB_ERROR", HttpResponse::InternalServerError())
                }
            };
            status.json(ErrorResponse {
                error: e.to_string(),
                code: code.to_string(),
            })
        }
    }
}

/// POST /api/inspections/batch - 一括検査実行（Phase 3: 複数台対応）
///
/// 接続中の全装置に対して検査を実行し、結果をDBに保存する
pub async fn run_batch_inspection(
    data: web::Data<AppState>,
    body: web::Json<BatchInspectionRequest>,
) -> impl Responder {
    // MultiDeviceManagerから接続中の全デバイスパスを取得
    let paths = {
        let multi_manager = match data.multi_device_manager.lock() {
            Ok(m) => m,
            Err(_) => {
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "内部エラー: ロック取得に失敗".to_string(),
                    code: "LOCK_ERROR".to_string(),
                });
            }
        };
        multi_manager.connected_paths()
    };

    if paths.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "装置が接続されていません。先に接続してください。".to_string(),
            code: "NO_DEVICES_CONNECTED".to_string(),
        });
    }

    let mut results: Vec<DeviceInspectionResult> = Vec::new();
    let mut passed = 0;
    let mut failed = 0;

    // 各デバイスに対して順次検査実行
    for path in &paths {
        let result = run_single_device_inspection(&data, path, body.lot_id).await;
        match result {
            Ok(device_result) => {
                if device_result.overall_result == "Pass" {
                    passed += 1;
                } else {
                    failed += 1;
                }
                results.push(device_result);
            }
            Err(error_result) => {
                failed += 1;
                results.push(error_result);
            }
        }
    }

    HttpResponse::Ok().json(BatchInspectionResponse {
        results,
        summary: BatchSummary {
            total: paths.len(),
            passed,
            failed,
        },
    })
}

/// 単一デバイスの検査を実行（内部ヘルパー）
async fn run_single_device_inspection(
    data: &web::Data<AppState>,
    path: &str,
    lot_id: Option<i64>,
) -> Result<DeviceInspectionResult, DeviceInspectionResult> {
    // パス指定でデバイスマネージャーを取得
    let device_manager_arc = match data.get_device_manager_by_path(path) {
        Ok(Some(arc)) => arc,
        Ok(None) => {
            return Err(DeviceInspectionResult {
                path: path.to_string(),
                serial_number: "不明".to_string(),
                overall_result: "Error".to_string(),
                inspection_id: 0,
                items: vec![],
            });
        }
        Err(_) => {
            return Err(DeviceInspectionResult {
                path: path.to_string(),
                serial_number: "不明".to_string(),
                overall_result: "Error".to_string(),
                inspection_id: 0,
                items: vec![],
            });
        }
    };

    let mut manager = match device_manager_arc.lock() {
        Ok(m) => m,
        Err(_) => {
            return Err(DeviceInspectionResult {
                path: path.to_string(),
                serial_number: "不明".to_string(),
                overall_result: "Error".to_string(),
                inspection_id: 0,
                items: vec![],
            });
        }
    };

    let repo = match data.repository.lock() {
        Ok(r) => r,
        Err(_) => {
            return Err(DeviceInspectionResult {
                path: path.to_string(),
                serial_number: "不明".to_string(),
                overall_result: "Error".to_string(),
                inspection_id: 0,
                items: vec![],
            });
        }
    };

    // InspectionServiceで検査実行
    let service = InspectionService::new(&repo);
    match service.run_and_save(&mut manager, lot_id) {
        Ok(report) => {
            // シリアル番号を取得
            let serial_number = report.item_results.iter()
                .find(|r| matches!(r.item_type, ItemType::SerialNumber))
                .and_then(|r| r.actual_value.clone())
                .unwrap_or_else(|| "不明".to_string());

            // 項目結果をレスポンス用に変換
            let items: Vec<ItemResultResponse> = report.item_results.iter()
                .map(|r| ItemResultResponse {
                    item_name: item_type_to_str(&r.item_type).to_string(),
                    verdict: verdict_to_str(&r.verdict).to_string(),
                    actual_value: r.actual_value.clone(),
                    expected_value: expected_to_string(&r.expected_value),
                })
                .collect();

            Ok(DeviceInspectionResult {
                path: path.to_string(),
                serial_number,
                overall_result: report.overall_result,
                inspection_id: report.inspection_id,
                items,
            })
        }
        Err(e) => {
            Err(DeviceInspectionResult {
                path: path.to_string(),
                serial_number: "不明".to_string(),
                overall_result: format!("Error: {}", e),
                inspection_id: 0,
                items: vec![],
            })
        }
    }
}

/// GET /api/inspections - 検査履歴取得
pub async fn list_inspections(data: web::Data<AppState>) -> impl Responder {
    let repo = match data.repository.lock() {
        Ok(r) => r,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: ロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            })
        }
    };

    match repo.get_all_inspections() {
        Ok(inspections) => {
            // 各検査の装置情報も取得
            let mut response_list = Vec::new();
            for inspection in inspections {
                let serial_number = repo.get_device(inspection.device_id)
                    .ok()
                    .map(|d| d.serial_number);

                response_list.push(InspectionSummaryResponse {
                    id: inspection.id.unwrap_or(0),
                    device_id: inspection.device_id,
                    serial_number,
                    inspected_at: inspection.inspected_at,
                    overall_result: inspection.overall_result,
                });
            }

            HttpResponse::Ok().json(InspectionListResponse {
                inspections: response_list,
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
            code: "DB_ERROR".to_string(),
        }),
    }
}

/// APIルートを設定
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/inspections")
            .route("", web::post().to(run_inspection))
            .route("", web::get().to(list_inspections))
            // Phase 3: 一括検査API
            .route("/batch", web::post().to(run_batch_inspection)),
    );
}
