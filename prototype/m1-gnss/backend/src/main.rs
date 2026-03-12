use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use m1_gnss::web::device_api::{self, AppState};
use m1_gnss::web::lot_api;
use m1_gnss::web::inspection_api;
use m1_gnss::web::outdoor_inspection_api;
use m1_gnss::web::nav_sat_api;
use m1_gnss::web::nav_sig_api;
use m1_gnss::web::mon_span_api;
use m1_gnss::web::nav_status_api;
use m1_gnss::web::ntrip_api::{self, SharedNtripManager, NtripManager};
use m1_gnss::web::gnss_state_api;

use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

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
    // ログ初期化（コンソール + ファイル出力）
    let log_dir = std::path::Path::new("logs");
    std::fs::create_dir_all(log_dir)?;

    let file_appender = tracing_appender::rolling::daily(log_dir, "gnss-backend.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // コンソール出力レイヤー
    let console_layer = fmt::layer()
        .with_target(true)
        .with_level(true);

    // ファイル出力レイヤー（JSON形式ではなく人間が読みやすい形式）
    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(true)
        .with_level(true);

    // フィルター設定（RUST_LOG環境変数、デフォルトはinfo）
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    tracing::info!("GNSS評価ツールを起動します...");
    tracing::info!("http://localhost:8080 でアクセス可能");
    tracing::info!("ログファイル: logs/gnss-backend.log.YYYY-MM-DD");

    // アプリケーション状態（DeviceManagerを共有）
    let app_state = web::Data::new(AppState::new());

    // NTRIP状態（非同期Mutex）
    let ntrip_state: web::Data<SharedNtripManager> =
        web::Data::new(Arc::new(TokioMutex::new(NtripManager::new())));

    HttpServer::new(move || {
        // CORS設定: フロントエンド（localhost:3000）からのリクエストを許可
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec!["Content-Type"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .app_data(ntrip_state.clone())
            .route("/health", web::get().to(health))
            .route("/api/gnss/status", web::get().to(gnss_status))
            .configure(device_api::configure)
            .configure(lot_api::configure)
            .configure(inspection_api::configure)
            .configure(outdoor_inspection_api::configure)
            .configure(nav_sat_api::configure)
            .configure(nav_sig_api::configure)
            .configure(mon_span_api::configure)
            .configure(nav_status_api::configure)
            .configure(ntrip_api::configure)
            .configure(gnss_state_api::configure)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
