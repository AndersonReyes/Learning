//! Broker binary.
//!
//! Usage:
//!   broker [--data-dir <path>] [--port <port>] [--flush-ms <ms>]
//!
//! Defaults: data-dir=./data, port=9092, flush-ms=500

use std::sync::Arc;
use std::time::Duration;

use mini_mq::broker::Registry;
use mini_mq::concurrent::SharedRegistry;
use mini_mq::server::{run_server, BrokerHandle};

fn usage() -> ! {
    eprintln!("Usage: broker [--data-dir <path>] [--port <port>] [--flush-ms <ms>]");
    std::process::exit(1);
}

fn parse_args() -> (String, u16, u64) {
    let mut data_dir = String::from("./data");
    let mut port: u16 = 9092;
    let mut flush_ms: u64 = 500;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--data-dir" => data_dir = args.next().unwrap_or_else(|| usage()),
            "--port" => {
                port = args
                    .next()
                    .unwrap_or_else(|| usage())
                    .parse()
                    .unwrap_or_else(|_| usage())
            }
            "--flush-ms" => {
                flush_ms = args
                    .next()
                    .unwrap_or_else(|| usage())
                    .parse()
                    .unwrap_or_else(|_| usage())
            }
            _ => usage(),
        }
    }
    (data_dir, port, flush_ms)
}

#[tokio::main]
async fn main() {
    let (data_dir, port, flush_ms) = parse_args();
    let addr = format!("0.0.0.0:{port}");

    let registry = Registry::open(std::path::Path::new(&data_dir)).expect("failed to open data directory");
    let shared = Arc::new(SharedRegistry::new(
        registry,
        Duration::from_millis(flush_ms),
    ));

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("failed to bind {addr}: {e}"));

    eprintln!("mini-mq broker listening on {addr}  (data-dir={data_dir})");

    run_server(listener, BrokerHandle::new(shared)).await.expect("server error");
}
