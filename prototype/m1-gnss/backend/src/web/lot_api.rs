//! ロット管理API
//!
//! - GET /api/lots - ロット一覧取得
//! - POST /api/lots - ロット作成
//! - GET /api/lots/{id} - ロット詳細取得
//! - PUT /api/lots/{id} - ロット更新

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::repository::Lot;
use super::device_api::{AppState, ErrorResponse};

// ===========================================
// リクエスト/レスポンス型
// ===========================================

/// ロット作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateLotRequest {
    pub lot_number: String,
    pub expected_rate_ms: Option<i32>,
    pub expected_port_in_proto: Option<String>,
    pub expected_port_out_proto: Option<String>,
    pub memo: Option<String>,
}

/// ロット更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateLotRequest {
    pub expected_rate_ms: Option<i32>,
    pub expected_port_in_proto: Option<String>,
    pub expected_port_out_proto: Option<String>,
    pub memo: Option<String>,
}

/// ロットレスポンス
#[derive(Debug, Serialize)]
pub struct LotResponse {
    pub id: i64,
    pub lot_number: String,
    pub expected_rate_ms: Option<i32>,
    pub expected_port_in_proto: Option<String>,
    pub expected_port_out_proto: Option<String>,
    pub memo: Option<String>,
}

impl From<Lot> for LotResponse {
    fn from(lot: Lot) -> Self {
        Self {
            id: lot.id.unwrap_or(0),
            lot_number: lot.lot_number,
            expected_rate_ms: lot.expected_rate_ms,
            expected_port_in_proto: lot.expected_port_in_proto,
            expected_port_out_proto: lot.expected_port_out_proto,
            memo: lot.memo,
        }
    }
}

/// ロット一覧レスポンス
#[derive(Debug, Serialize)]
pub struct LotListResponse {
    pub lots: Vec<LotResponse>,
}

// ===========================================
// APIハンドラー
// ===========================================

/// GET /api/lots - ロット一覧取得
pub async fn list_lots(data: web::Data<AppState>) -> impl Responder {
    let repo = match data.repository.lock() {
        Ok(r) => r,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: ロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            })
        }
    };

    match repo.get_all_lots() {
        Ok(lots) => {
            let response: Vec<LotResponse> = lots.into_iter().map(LotResponse::from).collect();
            HttpResponse::Ok().json(LotListResponse { lots: response })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
            code: "DB_ERROR".to_string(),
        }),
    }
}

/// POST /api/lots - ロット作成
pub async fn create_lot(
    data: web::Data<AppState>,
    body: web::Json<CreateLotRequest>,
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

    // ロット番号の重複チェックはDB制約で行う
    let mut lot = Lot::new(body.lot_number.clone());
    if let Some(rate) = body.expected_rate_ms {
        lot = lot.with_expected_rate(rate);
    }
    if let (Some(in_proto), Some(out_proto)) = (&body.expected_port_in_proto, &body.expected_port_out_proto) {
        lot = lot.with_expected_port_proto(in_proto, out_proto);
    }
    lot.memo = body.memo.clone();

    match repo.insert_lot(&lot) {
        Ok(id) => {
            let mut created = lot;
            created.id = Some(id);
            HttpResponse::Created().json(LotResponse::from(created))
        }
        Err(e) => {
            let error_str = e.to_string();
            if error_str.contains("UNIQUE constraint failed") {
                HttpResponse::Conflict().json(ErrorResponse {
                    error: format!("ロット番号 '{}' は既に存在します", body.lot_number),
                    code: "DUPLICATE_LOT_NUMBER".to_string(),
                })
            } else {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: error_str,
                    code: "DB_ERROR".to_string(),
                })
            }
        }
    }
}

/// GET /api/lots/{id} - ロット詳細取得
pub async fn get_lot(
    data: web::Data<AppState>,
    path: web::Path<i64>,
) -> impl Responder {
    let lot_id = path.into_inner();

    let repo = match data.repository.lock() {
        Ok(r) => r,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: ロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            })
        }
    };

    match repo.get_lot(lot_id) {
        Ok(lot) => HttpResponse::Ok().json(LotResponse::from(lot)),
        Err(crate::repository::RepositoryError::NotFound(_)) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: format!("ロット (id={}) が見つかりません", lot_id),
                code: "NOT_FOUND".to_string(),
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
            code: "DB_ERROR".to_string(),
        }),
    }
}

/// PUT /api/lots/{id} - ロット更新
pub async fn update_lot(
    data: web::Data<AppState>,
    path: web::Path<i64>,
    body: web::Json<UpdateLotRequest>,
) -> impl Responder {
    let lot_id = path.into_inner();

    let repo = match data.repository.lock() {
        Ok(r) => r,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "内部エラー: ロック取得に失敗".to_string(),
                code: "LOCK_ERROR".to_string(),
            })
        }
    };

    // 更新実行
    match repo.update_lot(
        lot_id,
        body.expected_rate_ms,
        body.expected_port_in_proto.as_deref(),
        body.expected_port_out_proto.as_deref(),
        body.memo.as_deref(),
    ) {
        Ok(()) => {
            // 更新後のデータを取得して返す
            match repo.get_lot(lot_id) {
                Ok(lot) => HttpResponse::Ok().json(LotResponse::from(lot)),
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

/// APIルートを設定
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/lots")
            .route("", web::get().to(list_lots))
            .route("", web::post().to(create_lot))
            .route("/{id}", web::get().to(get_lot))
            .route("/{id}", web::put().to(update_lot)),
    );
}
