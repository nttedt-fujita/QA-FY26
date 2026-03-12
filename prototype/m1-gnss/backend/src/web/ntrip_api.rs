//! NTRIP API
//!
//! - POST /api/ntrip/connect - NTRIP接続開始
//! - POST /api/ntrip/disconnect - NTRIP切断
//! - GET /api/ntrip/status - 接続状態取得
//!
//! ## 設計判断
//!
//! ntrip-clientクレートを検討したが、以下の理由で独自実装を採用:
//! - ntrip-clientはRTCMパース済みデータ(rtcm_rs::Message)を返す
//! - ZED-F9Pには生のRTCMバイナリをそのまま転送する必要がある
//! - 独自実装でHTTP/1.0ベースのNTRIP Rev1プロトコルを直接実装
//!
//! 参照: docs/adr/m1/ADR-011-ntrip-implementation.md (要作成)

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::{broadcast, Mutex as TokioMutex};
use tokio::time::{interval, Duration};

use super::device_api::AppState;

// ===========================================
// リクエスト/レスポンス型
// ===========================================

/// NTRIP接続リクエスト
#[derive(Debug, Deserialize)]
pub struct NtripConnectRequest {
    /// キャスターURL（例: ntrip.jenoba.jp）
    pub caster_url: String,
    /// ポート（例: 2101）
    pub port: u16,
    /// マウントポイント（例: TOKYO_RTCM3）
    pub mountpoint: String,
    /// ユーザー名
    pub username: String,
    /// パスワード
    pub password: String,
}

/// NTRIP接続状態
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NtripConnectionState {
    /// 未接続
    Disconnected,
    /// 接続中
    Connecting,
    /// 接続済み
    Connected,
    /// エラー
    Error(String),
}

/// NTRIP状態レスポンス
#[derive(Debug, Serialize, Deserialize)]
pub struct NtripStatusResponse {
    /// 接続状態
    pub state: NtripConnectionState,
    /// 受信バイト数
    pub bytes_received: u64,
    /// ZED-F9Pへの転送バイト数
    pub bytes_forwarded: u64,
    /// 最後のエラー（あれば）
    pub last_error: Option<String>,
}

/// エラーレスポンス
#[derive(Debug, Serialize, Deserialize)]
pub struct NtripErrorResponse {
    pub error: String,
    pub code: String,
}

// ===========================================
// NTRIP接続管理
// ===========================================

/// NTRIP接続マネージャー
///
/// NTRIPサーバーからRTCMデータを受信し、ZED-F9Pに転送する
pub struct NtripManager {
    /// 接続状態
    state: NtripConnectionState,
    /// 受信バイト数
    bytes_received: u64,
    /// 転送バイト数
    bytes_forwarded: u64,
    /// 最後のエラー
    last_error: Option<String>,
    /// 切断シグナル送信側
    exit_tx: Option<broadcast::Sender<()>>,
}

impl NtripManager {
    pub fn new() -> Self {
        Self {
            state: NtripConnectionState::Disconnected,
            bytes_received: 0,
            bytes_forwarded: 0,
            last_error: None,
            exit_tx: None,
        }
    }

    pub fn state(&self) -> &NtripConnectionState {
        &self.state
    }

    pub fn set_state(&mut self, state: NtripConnectionState) {
        self.state = state;
    }

    pub fn set_exit_tx(&mut self, tx: broadcast::Sender<()>) {
        self.exit_tx = Some(tx);
    }

    pub fn take_exit_tx(&mut self) -> Option<broadcast::Sender<()>> {
        self.exit_tx.take()
    }

    pub fn add_bytes_received(&mut self, bytes: u64) {
        self.bytes_received += bytes;
    }

    pub fn add_bytes_forwarded(&mut self, bytes: u64) {
        self.bytes_forwarded += bytes;
    }

    pub fn set_last_error(&mut self, error: String) {
        self.last_error = Some(error);
    }

    pub fn status(&self) -> NtripStatusResponse {
        NtripStatusResponse {
            state: self.state.clone(),
            bytes_received: self.bytes_received,
            bytes_forwarded: self.bytes_forwarded,
            last_error: self.last_error.clone(),
        }
    }

    pub fn reset_stats(&mut self) {
        self.bytes_received = 0;
        self.bytes_forwarded = 0;
        self.last_error = None;
    }
}

impl Default for NtripManager {
    fn default() -> Self {
        Self::new()
    }
}

/// NTRIP状態を保持する共有状態
pub type SharedNtripManager = Arc<TokioMutex<NtripManager>>;

// ===========================================
// NTRIP接続処理
// ===========================================

/// Base64エンコード（Basic認証用）
fn base64_encode(input: &str) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let bytes = input.as_bytes();
    let mut result = String::with_capacity((bytes.len() + 2) / 3 * 4);
    let mut i = 0;

    while i < bytes.len() {
        let b0 = bytes[i];
        let b1 = bytes.get(i + 1).copied().unwrap_or(0);
        let b2 = bytes.get(i + 2).copied().unwrap_or(0);

        let n = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);

        result.push(ALPHABET[((n >> 18) & 0x3F) as usize] as char);
        result.push(ALPHABET[((n >> 12) & 0x3F) as usize] as char);

        if i + 1 < bytes.len() {
            result.push(ALPHABET[((n >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }

        if i + 2 < bytes.len() {
            result.push(ALPHABET[(n & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }

        i += 3;
    }
    result
}

/// GGAセンテンスを生成（テスト用固定位置）
///
/// VRS型NTRIPサービスでは、クライアントが位置情報を送信する必要がある。
/// 現在は東京近辺の固定座標を使用（テスト用）。
fn generate_gga_sentence() -> String {
    use chrono::Utc;

    let now = Utc::now();
    let time_str = now.format("%H%M%S.00").to_string();

    // 東京近辺の座標（テスト用固定値）
    // 緯度: 35.6762° N → 3540.572 (DDMM.MMM形式)
    // 経度: 139.6503° E → 13939.018 (DDDMM.MMM形式)
    let gga_body = format!(
        "GPGGA,{},3540.5720,N,13939.0180,E,1,08,1.0,50.0,M,0.0,M,,",
        time_str
    );

    // チェックサム計算（$と*の間のXOR）
    let checksum: u8 = gga_body.bytes().fold(0, |acc, b| acc ^ b);

    format!("${gga_body}*{checksum:02X}\r\n")
}

/// NTRIPサーバーに接続
///
/// NTRIP Rev1プロトコル（HTTP/1.0ベース）を使用。
/// 参照: docs/missions/m1-sensor-evaluation/gnss/20-ntrip-rtk-implementation.md
async fn connect_to_ntrip(
    caster_url: &str,
    port: u16,
    mountpoint: &str,
    username: &str,
    password: &str,
) -> Result<TcpStream, String> {
    use std::time::Duration;
    use tokio::time::timeout;
    use tokio::net::lookup_host;

    const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
    const READ_TIMEOUT: Duration = Duration::from_secs(10);

    log::info!("[NTRIP] ========== 接続開始 ==========");
    log::info!("[NTRIP] キャスター: {}:{}", caster_url, port);
    log::info!("[NTRIP] マウントポイント: {}", mountpoint);
    log::info!("[NTRIP] ユーザー: {}", username);

    // DNS解決チェック
    let addr = format!("{}:{}", caster_url, port);
    log::info!("[NTRIP] DNS解決中: {}", caster_url);
    match lookup_host(&addr).await {
        Ok(mut addrs) => {
            if let Some(resolved) = addrs.next() {
                log::info!("[NTRIP] DNS解決成功: {} -> {}", caster_url, resolved);
            } else {
                log::error!("[NTRIP] DNS解決失敗: アドレスが見つかりません");
                return Err(format!("DNS解決失敗: {} のアドレスが見つかりません", caster_url));
            }
        }
        Err(e) => {
            log::error!("[NTRIP] DNS解決失敗: {}", e);
            return Err(format!("DNS解決失敗: {} - {}", caster_url, e));
        }
    }

    log::info!("[NTRIP] TCP接続開始: {}", addr);

    let mut stream = match timeout(CONNECT_TIMEOUT, TcpStream::connect(&addr)).await {
        Ok(Ok(s)) => {
            log::info!("[NTRIP] TCP接続成功");
            s
        }
        Ok(Err(e)) => {
            log::error!("[NTRIP] TCP接続失敗: {}", e);
            return Err(format!("TCP接続失敗: {}", e));
        }
        Err(_) => {
            log::error!("[NTRIP] TCP接続タイムアウト ({}秒)", CONNECT_TIMEOUT.as_secs());
            return Err(format!("TCP接続タイムアウト ({}秒)", CONNECT_TIMEOUT.as_secs()));
        }
    };

    // NTRIP Rev1リクエストを送信
    // User-Agentは "NTRIP" で始める必要がある（仕様書 5.1節）
    let auth = base64_encode(&format!("{}:{}", username, password));
    let request = format!(
        "GET /{} HTTP/1.0\r\n\
         User-Agent: NTRIP GnssEvalTool/1.0\r\n\
         Authorization: Basic {}\r\n\
         \r\n",
        mountpoint, auth
    );

    log::info!("[NTRIP] リクエスト送信: GET /{}", mountpoint);
    log::debug!("[NTRIP] リクエスト全文:\n{}", request);

    stream
        .write_all(request.as_bytes())
        .await
        .map_err(|e| {
            log::error!("[NTRIP] リクエスト送信失敗: {}", e);
            format!("リクエスト送信失敗: {}", e)
        })?;

    log::info!("[NTRIP] リクエスト送信完了、レスポンス待機中...");

    // レスポンスを読む（タイムアウト付き）
    let mut reader = BufReader::new(&mut stream);
    let mut response_line = String::new();

    match timeout(READ_TIMEOUT, reader.read_line(&mut response_line)).await {
        Ok(Ok(_)) => {
            log::info!("[NTRIP] レスポンス受信: {}", response_line.trim());
        }
        Ok(Err(e)) => {
            log::error!("[NTRIP] レスポンス読み取り失敗: {}", e);
            return Err(format!("レスポンス読み取り失敗: {}", e));
        }
        Err(_) => {
            log::error!("[NTRIP] レスポンスタイムアウト ({}秒)", READ_TIMEOUT.as_secs());
            return Err(format!("レスポンスタイムアウト ({}秒)", READ_TIMEOUT.as_secs()));
        }
    }

    // NTRIP Rev1: "ICY 200 OK" または HTTP/1.x 200 OK
    if response_line.contains("200") {
        log::info!("[NTRIP] 接続成功 (200 OK)");
        // ヘッダーの残りを読み捨てる（空行まで）
        loop {
            let mut line = String::new();
            let bytes_read = reader
                .read_line(&mut line)
                .await
                .map_err(|e| format!("ヘッダー読み取り失敗: {}", e))?;
            if bytes_read == 0 || line.trim().is_empty() {
                break;
            }
            log::debug!("[NTRIP] ヘッダー: {}", line.trim());
        }
        log::info!("[NTRIP] ヘッダー読み取り完了、RTCMストリーム開始");
        Ok(stream)
    } else if response_line.contains("401") {
        log::error!("[NTRIP] 認証失敗 (401)");
        Err("認証失敗 (401 Unauthorized)".to_string())
    } else if response_line.contains("404") {
        log::error!("[NTRIP] マウントポイント不明 (404)");
        Err("マウントポイントが見つかりません (404)".to_string())
    } else {
        log::error!("[NTRIP] 予期しないレスポンス: {}", response_line.trim());
        Err(format!("予期しないレスポンス: {}", response_line.trim()))
    }
}

// ===========================================
// APIハンドラー
// ===========================================

/// POST /api/ntrip/connect - NTRIP接続開始
pub async fn connect_ntrip(
    ntrip_state: web::Data<SharedNtripManager>,
    app_state: web::Data<AppState>,
    req: web::Json<NtripConnectRequest>,
) -> impl Responder {
    // 現在の状態を確認
    {
        let manager = ntrip_state.lock().await;
        if matches!(
            manager.state(),
            NtripConnectionState::Connected | NtripConnectionState::Connecting
        ) {
            return HttpResponse::Conflict().json(NtripErrorResponse {
                error: "既にNTRIP接続中です。切断してから再接続してください".to_string(),
                code: "ALREADY_CONNECTED".to_string(),
            });
        }
    }

    // 状態を「接続中」に変更
    {
        let mut manager = ntrip_state.lock().await;
        manager.set_state(NtripConnectionState::Connecting);
        manager.reset_stats();
    }

    // NTRIPサーバーに接続
    let stream = match connect_to_ntrip(
        &req.caster_url,
        req.port,
        &req.mountpoint,
        &req.username,
        &req.password,
    )
    .await
    {
        Ok(s) => s,
        Err(e) => {
            let mut manager = ntrip_state.lock().await;
            manager.set_state(NtripConnectionState::Error(e.clone()));
            manager.set_last_error(e.clone());
            return HttpResponse::BadRequest().json(NtripErrorResponse {
                error: e,
                code: "NTRIP_CONNECT_ERROR".to_string(),
            });
        }
    };

    // 切断シグナル用チャンネル
    let (exit_tx, mut exit_rx) = broadcast::channel::<()>(1);

    // 状態にexit_txを保存
    {
        let mut manager = ntrip_state.lock().await;
        manager.set_exit_tx(exit_tx);
        manager.set_state(NtripConnectionState::Connected);
    }

    // RTCMデータ受信・転送タスクをスポーン
    let ntrip_state_clone = ntrip_state.clone();
    let app_state_clone = app_state.clone();
    tokio::spawn(async move {
        // ストリームを読み書き分割
        let (mut reader, mut writer) = tokio::io::split(stream);
        let mut buf = [0u8; 4096];

        // GGA定期送信タイマー（10秒間隔）
        let mut gga_interval = interval(Duration::from_secs(10));

        // 接続直後に最初のGGAを送信
        let initial_gga = generate_gga_sentence();
        log::info!("[NTRIP] 初回GGA送信: {}", initial_gga.trim());
        if let Err(e) = writer.write_all(initial_gga.as_bytes()).await {
            log::error!("[NTRIP] 初回GGA送信失敗: {}", e);
        }

        loop {
            tokio::select! {
                // 切断シグナル
                _ = exit_rx.recv() => {
                    log::info!("NTRIP切断シグナルを受信");
                    break;
                }
                // GGA定期送信（10秒ごと）
                _ = gga_interval.tick() => {
                    let gga = generate_gga_sentence();
                    log::debug!("[NTRIP] GGA送信: {}", gga.trim());
                    if let Err(e) = writer.write_all(gga.as_bytes()).await {
                        log::warn!("[NTRIP] GGA送信失敗: {}", e);
                    }
                }
                // RTCMデータ受信
                result = reader.read(&mut buf) => {
                    match result {
                        Ok(0) => {
                            // 接続終了
                            log::info!("NTRIPサーバーが接続を閉じました");
                            let mut manager = ntrip_state_clone.lock().await;
                            manager.set_state(NtripConnectionState::Disconnected);
                            break;
                        }
                        Ok(n) => {
                            // RTCMデータを受信
                            let rtcm_data = &buf[..n];

                            // 統計更新
                            {
                                let mut manager = ntrip_state_clone.lock().await;
                                manager.add_bytes_received(n as u64);
                            }

                            // ZED-F9Pに転送（DeviceManagerを使用）
                            let forwarded = {
                                let mut device_manager = match app_state_clone.device_manager.lock() {
                                    Ok(m) => m,
                                    Err(_) => {
                                        log::error!("DeviceManagerのロック取得に失敗");
                                        continue;
                                    }
                                };

                                // シリアルポートに書き込み
                                match device_manager.write_data(rtcm_data) {
                                    Ok(written) => {
                                        log::debug!("RTCM転送: {} bytes", written);
                                        written as u64
                                    }
                                    Err(e) => {
                                        log::warn!("RTCM転送失敗: {}", e);
                                        0
                                    }
                                }
                            };

                            // 統計更新（転送成功分）
                            if forwarded > 0 {
                                let mut manager = ntrip_state_clone.lock().await;
                                manager.add_bytes_forwarded(forwarded);
                            }
                        }
                        Err(e) => {
                            log::error!("RTCMデータ受信エラー: {}", e);
                            let mut manager = ntrip_state_clone.lock().await;
                            manager.set_state(NtripConnectionState::Error(e.to_string()));
                            manager.set_last_error(e.to_string());
                            break;
                        }
                    }
                }
            }
        }

        log::info!("NTRIPストリームタスクが終了しました");
    });

    HttpResponse::Ok().json(serde_json::json!({
        "message": format!("NTRIP接続を開始しました: {}:{}/{}", req.caster_url, req.port, req.mountpoint),
        "state": "Connected"
    }))
}

/// POST /api/ntrip/disconnect - NTRIP切断
pub async fn disconnect_ntrip(ntrip_state: web::Data<SharedNtripManager>) -> impl Responder {
    let mut manager = ntrip_state.lock().await;

    // 接続状態を確認
    if !matches!(
        manager.state(),
        NtripConnectionState::Connected | NtripConnectionState::Connecting
    ) {
        return HttpResponse::BadRequest().json(NtripErrorResponse {
            error: "NTRIPは接続されていません".to_string(),
            code: "NOT_CONNECTED".to_string(),
        });
    }

    // 切断シグナルを送信
    if let Some(exit_tx) = manager.take_exit_tx() {
        let _ = exit_tx.send(());
    }

    manager.set_state(NtripConnectionState::Disconnected);

    HttpResponse::Ok().json(serde_json::json!({
        "message": "NTRIP接続を切断しました"
    }))
}

/// GET /api/ntrip/status - 接続状態取得
pub async fn get_ntrip_status(ntrip_state: web::Data<SharedNtripManager>) -> impl Responder {
    let manager = ntrip_state.lock().await;
    HttpResponse::Ok().json(manager.status())
}

/// APIルートを設定
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/ntrip")
            .route("/connect", web::post().to(connect_ntrip))
            .route("/disconnect", web::post().to(disconnect_ntrip))
            .route("/status", web::get().to(get_ntrip_status)),
    );
}

// ===========================================
// テスト
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // -----------------------------------------
    // Base64エンコードのテーブルテスト
    // -----------------------------------------
    #[rstest]
    // 正常系: 標準的なケース
    #[case("Aladdin:open sesame", "QWxhZGRpbjpvcGVuIHNlc2FtZQ==", true)]
    #[case("user:pass", "dXNlcjpwYXNz", true)]
    // 正常系: パディングなし（3の倍数バイト）
    #[case("abc", "YWJj", true)]
    #[case("abcdef", "YWJjZGVm", true)]
    // 正常系: パディング1個（3n+2バイト）
    #[case("ab", "YWI=", true)]
    #[case("abcde", "YWJjZGU=", true)]
    // 正常系: パディング2個（3n+1バイト）
    #[case("a", "YQ==", true)]
    #[case("abcd", "YWJjZA==", true)]
    // 境界値: 空文字列
    #[case("", "", true)]
    // 正常系: 1文字
    #[case("x", "eA==", true)]
    fn test_base64_encode(
        #[case] input: &str,
        #[case] expected: &str,
        #[case] should_succeed: bool,
    ) {
        let result = base64_encode(input);
        if should_succeed {
            assert_eq!(result, expected, "入力: {}", input);
        }
    }

    // -----------------------------------------
    // NtripManager状態遷移のテーブルテスト
    // -----------------------------------------
    #[rstest]
    // 正常系: 初期状態
    #[case(NtripConnectionState::Disconnected, true)]
    // 正常系: 接続中への遷移
    #[case(NtripConnectionState::Connecting, true)]
    // 正常系: 接続済みへの遷移
    #[case(NtripConnectionState::Connected, true)]
    // 正常系: エラー状態への遷移
    #[case(NtripConnectionState::Error("test error".to_string()), true)]
    fn test_ntrip_manager_state_transition(
        #[case] target_state: NtripConnectionState,
        #[case] should_succeed: bool,
    ) {
        let mut manager = NtripManager::new();
        // 初期状態の確認
        assert_eq!(*manager.state(), NtripConnectionState::Disconnected);

        if should_succeed {
            manager.set_state(target_state.clone());
            assert_eq!(*manager.state(), target_state);
        }
    }

    // -----------------------------------------
    // NtripManager統計のテーブルテスト
    // -----------------------------------------
    #[rstest]
    // 正常系: 0バイト
    #[case(0, 0, true)]
    // 正常系: 典型的な値
    #[case(100, 100, true)]
    // 正常系: 受信のみ（転送失敗のケース）
    #[case(1000, 0, true)]
    // 正常系: 大きな値
    #[case(1_000_000, 999_000, true)]
    fn test_ntrip_manager_stats(
        #[case] bytes_received: u64,
        #[case] bytes_forwarded: u64,
        #[case] should_succeed: bool,
    ) {
        let mut manager = NtripManager::new();
        manager.add_bytes_received(bytes_received);
        manager.add_bytes_forwarded(bytes_forwarded);

        let status = manager.status();
        if should_succeed {
            assert_eq!(status.bytes_received, bytes_received);
            assert_eq!(status.bytes_forwarded, bytes_forwarded);
        }
    }

    // -----------------------------------------
    // NtripManagerリセットのテーブルテスト
    // -----------------------------------------
    #[rstest]
    // 正常系: 統計とエラーがリセットされる
    #[case(100, 100, Some("test error".to_string()), true)]
    // 正常系: エラーなしでもリセット可能
    #[case(500, 400, None, true)]
    // 境界値: 0でもリセット可能
    #[case(0, 0, None, true)]
    fn test_ntrip_manager_reset(
        #[case] initial_received: u64,
        #[case] initial_forwarded: u64,
        #[case] initial_error: Option<String>,
        #[case] should_succeed: bool,
    ) {
        let mut manager = NtripManager::new();
        manager.add_bytes_received(initial_received);
        manager.add_bytes_forwarded(initial_forwarded);
        if let Some(err) = initial_error {
            manager.set_last_error(err);
        }

        manager.reset_stats();

        let status = manager.status();
        if should_succeed {
            assert_eq!(status.bytes_received, 0);
            assert_eq!(status.bytes_forwarded, 0);
            assert_eq!(status.last_error, None);
        }
    }

    // -----------------------------------------
    // NtripManager初期化のテスト
    // -----------------------------------------
    #[test]
    fn test_ntrip_manager_new() {
        let manager = NtripManager::new();
        assert_eq!(*manager.state(), NtripConnectionState::Disconnected);
        assert_eq!(manager.bytes_received, 0);
        assert_eq!(manager.bytes_forwarded, 0);
        assert!(manager.last_error.is_none());
        assert!(manager.exit_tx.is_none());
    }

    #[test]
    fn test_ntrip_manager_default() {
        let manager = NtripManager::default();
        assert_eq!(*manager.state(), NtripConnectionState::Disconnected);
    }

    // -----------------------------------------
    // APIハンドラー統合テスト（GET /status）
    // -----------------------------------------
    mod api_tests {
        use super::*;
        use actix_web::{test, App};

        /// NtripManagerの共有状態を作成
        fn create_test_ntrip_state() -> SharedNtripManager {
            Arc::new(TokioMutex::new(NtripManager::new()))
        }

        #[actix_web::test]
        async fn test_get_status_returns_disconnected_initially() {
            let ntrip_state = create_test_ntrip_state();
            let app = test::init_service(
                App::new()
                    .app_data(web::Data::new(ntrip_state.clone()))
                    .route("/api/ntrip/status", web::get().to(get_ntrip_status)),
            )
            .await;

            let req = test::TestRequest::get()
                .uri("/api/ntrip/status")
                .to_request();
            let resp = test::call_service(&app, req).await;

            assert!(resp.status().is_success());

            let body: NtripStatusResponse = test::read_body_json(resp).await;
            assert_eq!(body.state, NtripConnectionState::Disconnected);
            assert_eq!(body.bytes_received, 0);
            assert_eq!(body.bytes_forwarded, 0);
        }

        #[actix_web::test]
        async fn test_get_status_returns_connected_state() {
            let ntrip_state = create_test_ntrip_state();

            // 接続状態に変更
            {
                let mut manager = ntrip_state.lock().await;
                manager.set_state(NtripConnectionState::Connected);
                manager.add_bytes_received(1000);
                manager.add_bytes_forwarded(800);
            }

            let app = test::init_service(
                App::new()
                    .app_data(web::Data::new(ntrip_state.clone()))
                    .route("/api/ntrip/status", web::get().to(get_ntrip_status)),
            )
            .await;

            let req = test::TestRequest::get()
                .uri("/api/ntrip/status")
                .to_request();
            let resp = test::call_service(&app, req).await;

            assert!(resp.status().is_success());

            let body: NtripStatusResponse = test::read_body_json(resp).await;
            assert_eq!(body.state, NtripConnectionState::Connected);
            assert_eq!(body.bytes_received, 1000);
            assert_eq!(body.bytes_forwarded, 800);
        }

        #[actix_web::test]
        async fn test_disconnect_when_not_connected_returns_bad_request() {
            let ntrip_state = create_test_ntrip_state();
            let app = test::init_service(
                App::new()
                    .app_data(web::Data::new(ntrip_state.clone()))
                    .route("/api/ntrip/disconnect", web::post().to(disconnect_ntrip)),
            )
            .await;

            let req = test::TestRequest::post()
                .uri("/api/ntrip/disconnect")
                .to_request();
            let resp = test::call_service(&app, req).await;

            // 接続していない状態でdisconnectはBadRequest
            assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);

            let body: NtripErrorResponse = test::read_body_json(resp).await;
            assert_eq!(body.code, "NOT_CONNECTED");
        }

        #[actix_web::test]
        async fn test_disconnect_when_connected_returns_ok() {
            let ntrip_state = create_test_ntrip_state();

            // 接続状態に変更（exit_txも設定）
            {
                let mut manager = ntrip_state.lock().await;
                manager.set_state(NtripConnectionState::Connected);
                let (tx, _rx) = broadcast::channel::<()>(1);
                manager.set_exit_tx(tx);
            }

            let app = test::init_service(
                App::new()
                    .app_data(web::Data::new(ntrip_state.clone()))
                    .route("/api/ntrip/disconnect", web::post().to(disconnect_ntrip)),
            )
            .await;

            let req = test::TestRequest::post()
                .uri("/api/ntrip/disconnect")
                .to_request();
            let resp = test::call_service(&app, req).await;

            assert!(resp.status().is_success());

            // 状態がDisconnectedになっていることを確認
            let manager = ntrip_state.lock().await;
            assert_eq!(*manager.state(), NtripConnectionState::Disconnected);
        }

        #[actix_web::test]
        async fn test_disconnect_when_connecting_returns_ok() {
            let ntrip_state = create_test_ntrip_state();

            // Connecting状態に変更
            {
                let mut manager = ntrip_state.lock().await;
                manager.set_state(NtripConnectionState::Connecting);
                let (tx, _rx) = broadcast::channel::<()>(1);
                manager.set_exit_tx(tx);
            }

            let app = test::init_service(
                App::new()
                    .app_data(web::Data::new(ntrip_state.clone()))
                    .route("/api/ntrip/disconnect", web::post().to(disconnect_ntrip)),
            )
            .await;

            let req = test::TestRequest::post()
                .uri("/api/ntrip/disconnect")
                .to_request();
            let resp = test::call_service(&app, req).await;

            assert!(resp.status().is_success());
        }
    }
}
