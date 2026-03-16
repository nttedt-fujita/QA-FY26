//! LED点滅テストバイナリ
//!
//! USB-UART基板のTX LEDを点滅させて、デバイスを物理的に識別するためのテストツール。
//! MON-VERポーリングコマンドを繰り返し送信することで、TX LEDが点滅する。
//!
//! 使用方法:
//!   cargo run --bin blink_test -- /dev/ttyACM0 3
//!
//! 作成: Session 204 (2026-03-16)

use m1_gnss::ubx::common::build_ubx_poll;
use std::io::Write;
use std::time::{Duration, Instant};

fn main() {
    // 引数パース
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("使用方法: blink_test <path> [duration_sec]");
        eprintln!("  path:         デバイスパス（例: /dev/ttyACM0）");
        eprintln!("  duration_sec: 点滅時間（秒）。デフォルト: 3");
        std::process::exit(1);
    }

    let path = &args[1];
    let duration: u64 = args
        .get(2)
        .map(|s| s.parse().unwrap_or(3))
        .unwrap_or(3);

    println!("========================================");
    println!("  LED点滅テスト");
    println!("========================================");
    println!("デバイス: {}", path);
    println!("点滅時間: {}秒", duration);
    println!();

    // シリアルポートを開く
    let mut port = match serialport::new(path, 115200)
        .timeout(Duration::from_millis(100))
        .open()
    {
        Ok(p) => p,
        Err(e) => {
            eprintln!("エラー: ポートを開けませんでした: {}", e);
            std::process::exit(1);
        }
    };

    // MON-VER poll (Class: 0x0A, ID: 0x04)
    let poll_cmd = build_ubx_poll(0x0A, 0x04);

    println!("点滅開始... (Ctrl+Cで中断)");
    println!();

    let start = Instant::now();
    let mut count = 0;

    while start.elapsed().as_secs() < duration {
        if let Err(e) = port.write_all(&poll_cmd) {
            eprintln!();
            eprintln!("送信エラー: {}", e);
            break;
        }
        count += 1;

        // 進捗表示（10回ごとにカウント表示）
        if count % 10 == 0 {
            print!("{}", count);
        } else {
            print!(".");
        }
        std::io::stdout().flush().ok();

        std::thread::sleep(Duration::from_millis(100));
    }

    println!();
    println!();
    println!("========================================");
    println!("  完了: {}回送信", count);
    println!("========================================");
    println!();
    println!("基板のTX LEDは点滅しましたか？");
    println!("  - 点滅した → LED点滅方式でデバイス識別可能");
    println!("  - 点滅しない → 抜き差し検知方式にフォールバック");
}
