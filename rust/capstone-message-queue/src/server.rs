//! Async TCP server: one tokio task per connection, newline-delimited JSON.

use std::sync::Arc;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

use crate::concurrent::SharedRegistry;
use crate::error::Result;
use crate::protocol::{Request, Response, TopicMeta};

// ── Request processing ────────────────────────────────────────────────────────

/// Processes one request against `registry` and returns the response(s) to send.
///
/// All errors are converted to `Response::Error` so the connection stays open.
pub fn process_request(registry: &SharedRegistry, request: Request) -> Vec<Response> {
    match request {
        Request::CreateTopic { topic, partitions } => {
            match registry.create_topic(&topic, partitions) {
                Ok(()) => vec![Response::TopicCreated { topic, partitions }],
                Err(e) => vec![Response::Error { message: e.to_string() }],
            }
        }

        Request::Produce { topic, key, payload } => {
            let bytes = match STANDARD.decode(&payload) {
                Ok(b) => b,
                Err(_) => {
                    return vec![Response::Error {
                        message: "payload: invalid base64".to_owned(),
                    }]
                }
            };
            let key_bytes: Option<Vec<u8>> = match key {
                None => None,
                Some(ref k) => match STANDARD.decode(k) {
                    Ok(b) => Some(b),
                    Err(_) => {
                        return vec![Response::Error {
                            message: "key: invalid base64".to_owned(),
                        }]
                    }
                },
            };
            match registry.produce(&topic, &bytes, key_bytes.as_deref()) {
                Ok((partition, offset)) => vec![Response::Produced { partition, offset }],
                Err(e) => vec![Response::Error { message: e.to_string() }],
            }
        }

        Request::Fetch { topic, partition, offset } => {
            match registry.fetch(&topic, partition, offset) {
                Ok(bytes) => vec![
                    Response::Record { offset, payload: STANDARD.encode(&bytes) },
                    Response::End,
                ],
                Err(e) => vec![Response::Error { message: e.to_string() }],
            }
        }

        Request::FetchBatch { topic, partition, offset, max_count } => {
            match registry.fetch_batch(&topic, partition, offset, max_count) {
                Ok(records) => {
                    let mut out: Vec<Response> = records
                        .into_iter()
                        .map(|r| Response::Record {
                            offset: r.offset,
                            payload: STANDARD.encode(&r.payload),
                        })
                        .collect();
                    out.push(Response::End);
                    out
                }
                Err(e) => vec![Response::Error { message: e.to_string() }],
            }
        }

        Request::Metadata => {
            match registry.topic_names() {
                Ok(names) => {
                    let topics = names
                        .into_iter()
                        .filter_map(|name| {
                            registry.num_partitions(&name).ok().map(|n| TopicMeta {
                                name,
                                partitions: n as u32,
                            })
                        })
                        .collect();
                    vec![Response::Metadata { topics }]
                }
                Err(e) => vec![Response::Error { message: e.to_string() }],
            }
        }
    }
}

// ── Connection handler ────────────────────────────────────────────────────────

/// Handles one TCP connection: reads newline-delimited JSON requests, writes
/// newline-delimited JSON responses.
///
/// The connection is closed when the client disconnects or an I/O error occurs.
pub async fn handle_connection(
    stream: TcpStream,
    registry: Arc<SharedRegistry>,
) -> std::io::Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    while let Some(line) = lines.next_line().await? {
        let line = line.trim().to_owned();
        if line.is_empty() {
            continue;
        }

        let responses = match serde_json::from_str::<Request>(&line) {
            Ok(req) => process_request(&registry, req),
            Err(e) => vec![Response::Error {
                message: format!("parse error: {e}"),
            }],
        };

        for resp in &responses {
            let json = serde_json::to_string(resp).expect("response serialization is infallible");
            writer.write_all(json.as_bytes()).await?;
            writer.write_all(b"\n").await?;
        }
        writer.flush().await?;
    }
    Ok(())
}

// ── Server ────────────────────────────────────────────────────────────────────

/// Runs the broker server on a pre-bound `listener`.
///
/// Spawns one tokio task per accepted connection. Never returns unless the
/// listener errors.
pub async fn run_server(listener: TcpListener, registry: Arc<SharedRegistry>) -> Result<()> {
    loop {
        let (stream, peer) = listener.accept().await?;
        let reg = Arc::clone(&registry);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, reg).await {
                eprintln!("connection {peer} error: {e}");
            }
        });
    }
}
