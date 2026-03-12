//! 屋外検査結果API
//!
//! - POST /api/outdoor-inspection-results - 検査結果を保存
//! - GET /api/outdoor-inspection-results - 検査結果一覧を取得
//! - GET /api/outdoor-inspection-results/{id} - 検査結果を取得

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::repository::OutdoorInspectionResult;
use super::device_api::{AppState, ErrorResponse};

// ===========================================
// リクエスト/レスポンス型
// ===========================================

/// 検査結果保存リクエスト
#[derive(Debug, Deserialize)]
pub struct SaveOutdoorResultRequest {
    /// シリアル番号（device_id解決用）
    pub serial_number: Option<String>,
    /// デバイスID（任意、serial_numberがあれば上書き）
    pub device_id: Option<i64>,
    /// ロットID（任意）
    pub lot_id: Option<i64>,
    /// 検査日時（ISO 8601）
    pub inspected_at: String,
    /// 検査時間（秒）
    pub duration_sec: i32,
    /// 総サンプル数
    pub sample_count: i32,
    /// RTK FIX率 (0.0-1.0)
    pub rtk_fix_rate: f64,
    /// RTK FIX時間（ms）
    pub rtk_fix_time_ms: Option<i32>,
    /// L2受信率 (0.0-1.0)
    pub l2_reception_rate: f64,
    /// L1最小C/N0（dBHz）
    pub l1_min_cno: f64,
    /// 総合判定
    pub is_pass: bool,
    /// L1受信感度判定
    pub l1_cno_pass: bool,
    /// L2受信率判定
    pub l2_rate_pass: bool,
    /// RTK FIX時間判定
    pub rtk_fix_time_pass: bool,
    /// RTK FIX率判定
    pub rtk_fix_rate_pass: bool,
    /// 不合格理由（JSON配列）
    pub failure_reasons: Option<Vec<String>>,
}

/// 検査結果レスポンス
#[derive(Debug, Serialize)]
pub struct OutdoorResultResponse {
    pub id: i64,
    pub device_id: Option<i64>,
    pub lot_id: Option<i64>,
    pub inspected_at: String,
    pub duration_sec: i32,
    pub sample_count: i32,
    pub rtk_fix_rate: f64,
    pub rtk_fix_time_ms: Option<i32>,
    pub l2_reception_rate: f64,
    pub l1_min_cno: f64,
    pub is_pass: bool,
    pub l1_cno_pass: bool,
    pub l2_rate_pass: bool,
    pub rtk_fix_time_pass: bool,
    pub rtk_fix_rate_pass: bool,
    pub failure_reasons: Vec<String>,
}

/// 検査結果一覧レスポンス
#[derive(Debug, Serialize)]
pub struct OutdoorResultListResponse {
    pub results: Vec<OutdoorResultResponse>,
}

// ===========================================
// ヘルパー関数
// ===========================================

/// OutdoorInspectionResult を OutdoorResultResponse に変換
fn to_response(result: &OutdoorInspectionResult) -> OutdoorResultResponse {
    let failure_reasons = result.failure_reasons
        .as_ref()
        .map(|s| serde_json::from_str::<Vec<String>>(s).unwrap_or_default())
        .unwrap_or_default();

    OutdoorResultResponse {
        id: result.id.unwrap_or(0),
        device_id: result.device_id,
        lot_id: result.lot_id,
        inspected_at: result.inspected_at.clone(),
        duration_sec: result.duration_sec,
        sample_count: result.sample_count,
        rtk_fix_rate: result.rtk_fix_rate,
        rtk_fix_time_ms: result.rtk_fix_time_ms,
        l2_reception_rate: result.l2_reception_rate,
        l1_min_cno: result.l1_min_cno,
        is_pass: result.is_pass,
        l1_cno_pass: result.l1_cno_pass,
        l2_rate_pass: result.l2_rate_pass,
        rtk_fix_time_pass: result.rtk_fix_time_pass,
        rtk_fix_rate_pass: result.rtk_fix_rate_pass,
        failure_reasons,
    }
}

// ===========================================
// APIハンドラー
// ===========================================

/// POST /api/outdoor-inspection-results - 検査結果を保存
pub async fn save_outdoor_result(
    data: web::Data<AppState>,
    body: web::Json<SaveOutdoorResultRequest>,
) -> impl Responder {
    let repo = match data.repository.lock() {
        Ok(r) => r,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: ロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            })
        }
    };

    // serial_number から device_id を解決
    let resolved_device_id = if let Some(serial) = &body.serial_number {
        match repo.get_device_by_serial(serial) {
            Ok(device) => device.id,
            Err(crate::repository::RepositoryError::NotFound(_)) => {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: format!("装置が見つかりません: serial_number={}。先に屋内検査を実行してください。", serial),
                    code: "DEVICE_NOT_FOUND".to_string(),
                })
            }
            Err(e) => {
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    error: e.to_string(),
                    code: "DB_ERROR".to_string(),
                })
            }
        }
    } else {
        body.device_id
    };

    // failure_reasons を JSON文字列に変換
    let failure_reasons_json = body.failure_reasons
        .as_ref()
        .map(|v| serde_json::to_string(v).unwrap_or_default());

    // エンティティを構築
    let mut result = OutdoorInspectionResult::new(
        body.inspected_at.clone(),
        body.duration_sec,
        body.sample_count,
    )
    .with_metrics(
        body.rtk_fix_rate,
        body.rtk_fix_time_ms,
        body.l2_reception_rate,
        body.l1_min_cno,
    )
    .with_judgment(
        body.is_pass,
        body.l1_cno_pass,
        body.l2_rate_pass,
        body.rtk_fix_time_pass,
        body.rtk_fix_rate_pass,
        failure_reasons_json,
    );

    if let Some(device_id) = resolved_device_id {
        result = result.with_device(device_id);
    }
    if let Some(lot_id) = body.lot_id {
        result = result.with_lot(lot_id);
    }

    // 保存
    match repo.insert_outdoor_inspection_result(&result) {
        Ok(id) => {
            // 保存したレコードを取得して返却
            match repo.get_outdoor_inspection_result(id) {
                Ok(saved) => HttpResponse::Created().json(to_response(&saved)),
                Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
                    error: e.to_string(),
                    code: "DB_ERROR".to_string(),
                }),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
            code: "DB_ERROR".to_string(),
        }),
    }
}

/// GET /api/outdoor-inspection-results - 検査結果一覧を取得
pub async fn list_outdoor_results(data: web::Data<AppState>) -> impl Responder {
    let repo = match data.repository.lock() {
        Ok(r) => r,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: ロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            })
        }
    };

    match repo.get_all_outdoor_inspection_results() {
        Ok(results) => {
            let response_list: Vec<OutdoorResultResponse> = results.iter()
                .map(to_response)
                .collect();

            HttpResponse::Ok().json(OutdoorResultListResponse {
                results: response_list,
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
            code: "DB_ERROR".to_string(),
        }),
    }
}

/// GET /api/outdoor-inspection-results/{id} - 検査結果を取得
pub async fn get_outdoor_result(
    data: web::Data<AppState>,
    path: web::Path<i64>,
) -> impl Responder {
    let id = path.into_inner();

    let repo = match data.repository.lock() {
        Ok(r) => r,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: ロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            })
        }
    };

    match repo.get_outdoor_inspection_result(id) {
        Ok(result) => HttpResponse::Ok().json(to_response(&result)),
        Err(crate::repository::RepositoryError::NotFound(_)) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: format!("検査結果が見つかりません: id={}", id),
                code: "NOT_FOUND".to_string(),
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
        web::scope("/api/outdoor-inspection-results")
            .route("", web::post().to(save_outdoor_result))
            .route("", web::get().to(list_outdoor_results))
            .route("/{id}", web::get().to(get_outdoor_result)),
    );
}
