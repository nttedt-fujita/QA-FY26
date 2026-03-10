use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

/// ヘルスチェック用レスポンス
#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

/// ヘルスチェックエンドポイント
async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}

/// ダミーのGNSSデータ（後でUBXパースに置き換え）
#[derive(Serialize)]
struct GnssStatus {
    /// 可視衛星数
    satellites_visible: u8,
    /// RTK FIX状態（0: No fix, 1: Fix, 2: Float）
    rtk_status: u8,
    /// 緯度（度）
    latitude: f64,
    /// 経度（度）
    longitude: f64,
}

/// GNSSステータス取得エンドポイント（ダミーデータ）
async fn gnss_status() -> impl Responder {
    // TODO: 実機からUBXデータを取得して返す
    let dummy = GnssStatus {
        satellites_visible: 12,
        rtk_status: 1,
        latitude: 35.6812,
        longitude: 139.7671,
    };
    HttpResponse::Ok().json(dummy)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ログ初期化
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    log::info!("GNSS評価ツールを起動します...");
    log::info!("http://localhost:8080 でアクセス可能");

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health))
            .route("/api/gnss/status", web::get().to(gnss_status))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
