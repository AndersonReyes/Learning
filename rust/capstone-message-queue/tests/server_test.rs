use std::sync::Arc;
use std::time::Duration;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use mini_mq::broker::Registry;
use mini_mq::concurrent::SharedRegistry;
use mini_mq::protocol::{Request, Response, TopicMeta};
use mini_mq::server::process_request;
use tempfile::TempDir;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

// ── helpers ───────────────────────────────────────────────────────────────────

fn make_registry(dir: &TempDir) -> Arc<SharedRegistry> {
    Arc::new(SharedRegistry::new(
        Registry::open(dir.path()).unwrap(),
        Duration::from_millis(100),
    ))
}

fn b64(s: &str) -> String {
    STANDARD.encode(s.as_bytes())
}

fn from_b64(s: &str) -> String {
    String::from_utf8(STANDARD.decode(s).unwrap()).unwrap()
}

// ── process_request unit tests ────────────────────────────────────────────────

#[test]
fn create_topic_response() {
    let dir = TempDir::new().unwrap();
    let reg = make_registry(&dir);
    let resp = process_request(
        &reg,
        Request::CreateTopic { topic: "events".into(), partitions: 3 },
    );
    assert_eq!(
        resp,
        vec![Response::TopicCreated { topic: "events".into(), partitions: 3 }]
    );
}

#[test]
fn create_topic_idempotent() {
    let dir = TempDir::new().unwrap();
    let reg = make_registry(&dir);
    process_request(&reg, Request::CreateTopic { topic: "t".into(), partitions: 2 });
    let resp =
        process_request(&reg, Request::CreateTopic { topic: "t".into(), partitions: 2 });
    assert!(matches!(resp[0], Response::TopicCreated { .. }));
}

#[test]
fn produce_returns_partition_and_offset() {
    let dir = TempDir::new().unwrap();
    let reg = make_registry(&dir);
    process_request(&reg, Request::CreateTopic { topic: "t".into(), partitions: 1 });

    let resp = process_request(
        &reg,
        Request::Produce { topic: "t".into(), key: None, payload: b64("hello") },
    );
    assert_eq!(resp, vec![Response::Produced { partition: 0, offset: 0 }]);

    let resp2 = process_request(
        &reg,
        Request::Produce { topic: "t".into(), key: None, payload: b64("world") },
    );
    assert_eq!(resp2, vec![Response::Produced { partition: 0, offset: 1 }]);
}

#[test]
fn produce_unknown_topic_returns_error() {
    let dir = TempDir::new().unwrap();
    let reg = make_registry(&dir);
    let resp = process_request(
        &reg,
        Request::Produce { topic: "nope".into(), key: None, payload: b64("x") },
    );
    assert!(matches!(resp[0], Response::Error { .. }));
}

#[test]
fn produce_invalid_base64_returns_error() {
    let dir = TempDir::new().unwrap();
    let reg = make_registry(&dir);
    process_request(&reg, Request::CreateTopic { topic: "t".into(), partitions: 1 });
    let resp = process_request(
        &reg,
        Request::Produce { topic: "t".into(), key: None, payload: "!!!not_b64".into() },
    );
    assert!(matches!(resp[0], Response::Error { .. }));
}

#[test]
fn fetch_roundtrip() {
    let dir = TempDir::new().unwrap();
    let reg = make_registry(&dir);
    process_request(&reg, Request::CreateTopic { topic: "t".into(), partitions: 1 });
    process_request(
        &reg,
        Request::Produce { topic: "t".into(), key: None, payload: b64("ping") },
    );

    let resp = process_request(
        &reg,
        Request::Fetch { topic: "t".into(), partition: 0, offset: 0 },
    );
    assert_eq!(resp.len(), 2);
    assert!(matches!(&resp[1], Response::End));
    if let Response::Record { offset, payload } = &resp[0] {
        assert_eq!(*offset, 0);
        assert_eq!(from_b64(payload), "ping");
    } else {
        panic!("expected Record, got {:?}", resp[0]);
    }
}

#[test]
fn fetch_out_of_range_returns_error() {
    let dir = TempDir::new().unwrap();
    let reg = make_registry(&dir);
    process_request(&reg, Request::CreateTopic { topic: "t".into(), partitions: 1 });
    let resp = process_request(
        &reg,
        Request::Fetch { topic: "t".into(), partition: 0, offset: 99 },
    );
    assert!(matches!(resp[0], Response::Error { .. }));
}

#[test]
fn fetch_batch_returns_records_and_end() {
    let dir = TempDir::new().unwrap();
    let reg = make_registry(&dir);
    process_request(&reg, Request::CreateTopic { topic: "t".into(), partitions: 1 });
    for i in 0..5_u32 {
        process_request(
            &reg,
            Request::Produce {
                topic: "t".into(),
                key: None,
                payload: b64(&i.to_string()),
            },
        );
    }

    let resp = process_request(
        &reg,
        Request::FetchBatch { topic: "t".into(), partition: 0, offset: 1, max_count: 3 },
    );
    // 3 records + End
    assert_eq!(resp.len(), 4);
    assert!(matches!(resp[3], Response::End));
    for (i, r) in resp[..3].iter().enumerate() {
        if let Response::Record { offset, payload } = r {
            assert_eq!(*offset, (i + 1) as u64);
            assert_eq!(from_b64(payload), (i + 1).to_string());
        } else {
            panic!("expected Record at index {i}");
        }
    }
}

#[test]
fn fetch_batch_empty_returns_just_end() {
    let dir = TempDir::new().unwrap();
    let reg = make_registry(&dir);
    process_request(&reg, Request::CreateTopic { topic: "t".into(), partitions: 1 });
    let resp = process_request(
        &reg,
        Request::FetchBatch { topic: "t".into(), partition: 0, offset: 0, max_count: 10 },
    );
    assert_eq!(resp, vec![Response::End]);
}

#[test]
fn metadata_lists_all_topics() {
    let dir = TempDir::new().unwrap();
    let reg = make_registry(&dir);
    process_request(&reg, Request::CreateTopic { topic: "beta".into(), partitions: 2 });
    process_request(&reg, Request::CreateTopic { topic: "alpha".into(), partitions: 1 });

    let resp = process_request(&reg, Request::Metadata);
    assert_eq!(resp.len(), 1);
    if let Response::Metadata { topics } = &resp[0] {
        let mut names: Vec<&str> = topics.iter().map(|t| t.name.as_str()).collect();
        names.sort();
        assert_eq!(names, vec!["alpha", "beta"]);
        let alpha = topics.iter().find(|t| t.name == "alpha").unwrap();
        assert_eq!(alpha.partitions, 1);
        let beta = topics.iter().find(|t| t.name == "beta").unwrap();
        assert_eq!(beta.partitions, 2);
    } else {
        panic!("expected Metadata response");
    }
}

#[test]
fn metadata_empty_registry() {
    let dir = TempDir::new().unwrap();
    let reg = make_registry(&dir);
    let resp = process_request(&reg, Request::Metadata);
    assert_eq!(resp, vec![Response::Metadata { topics: vec![] }]);
}

#[test]
fn produce_with_key_routes_deterministically() {
    let dir = TempDir::new().unwrap();
    let reg = make_registry(&dir);
    process_request(&reg, Request::CreateTopic { topic: "t".into(), partitions: 4 });

    let key = b64("user-42");
    let r1 = process_request(
        &reg,
        Request::Produce { topic: "t".into(), key: Some(key.clone()), payload: b64("a") },
    );
    let r2 = process_request(
        &reg,
        Request::Produce { topic: "t".into(), key: Some(key.clone()), payload: b64("b") },
    );

    // Same key → same partition.
    if let (Response::Produced { partition: p1, .. }, Response::Produced { partition: p2, .. }) =
        (&r1[0], &r2[0])
    {
        assert_eq!(p1, p2);
    } else {
        panic!("expected Produced responses");
    }
}

// ── end-to-end TCP tests ──────────────────────────────────────────────────────

/// Starts a test broker on an OS-assigned port, returns the port and a
/// registry handle. The caller is responsible for keeping the TempDir alive.
async fn start_test_broker(dir: &TempDir) -> (u16, Arc<SharedRegistry>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let reg = Arc::new(SharedRegistry::new(
        Registry::open(dir.path()).unwrap(),
        Duration::from_millis(50),
    ));
    let reg_clone = Arc::clone(&reg);

    tokio::spawn(async move {
        mini_mq::server::run_server(listener, reg_clone).await.ok();
    });

    (port, reg)
}

async fn connect(port: u16) -> (
    tokio::io::BufReader<tokio::net::tcp::OwnedReadHalf>,
    tokio::net::tcp::OwnedWriteHalf,
) {
    let stream = tokio::net::TcpStream::connect(format!("127.0.0.1:{port}"))
        .await
        .unwrap();
    let (r, w) = stream.into_split();
    (BufReader::new(r), w)
}

async fn send(writer: &mut tokio::net::tcp::OwnedWriteHalf, line: &str) {
    writer.write_all(line.as_bytes()).await.unwrap();
    writer.write_all(b"\n").await.unwrap();
    writer.flush().await.unwrap();
}

async fn recv(reader: &mut tokio::io::BufReader<tokio::net::tcp::OwnedReadHalf>) -> String {
    let mut line = String::new();
    reader.read_line(&mut line).await.unwrap();
    line.trim().to_owned()
}

async fn recv_json(
    reader: &mut tokio::io::BufReader<tokio::net::tcp::OwnedReadHalf>,
) -> serde_json::Value {
    serde_json::from_str(&recv(reader).await).unwrap()
}

#[tokio::test]
async fn tcp_create_and_produce_fetch() {
    let dir = TempDir::new().unwrap();
    let (port, _reg) = start_test_broker(&dir).await;
    let (mut reader, mut writer) = connect(port).await;

    // Create topic
    send(&mut writer, r#"{"type":"create_topic","topic":"e","partitions":1}"#).await;
    let resp = recv_json(&mut reader).await;
    assert_eq!(resp["type"], "topic_created");
    assert_eq!(resp["topic"], "e");

    // Produce
    let payload = b64("hello tcp");
    send(
        &mut writer,
        &format!(r#"{{"type":"produce","topic":"e","payload":"{payload}"}}"#),
    )
    .await;
    let resp = recv_json(&mut reader).await;
    assert_eq!(resp["type"], "produced");
    assert_eq!(resp["offset"], 0);

    // Fetch
    send(&mut writer, r#"{"type":"fetch","topic":"e","partition":0,"offset":0}"#).await;
    let record = recv_json(&mut reader).await;
    let end = recv_json(&mut reader).await;
    assert_eq!(record["type"], "record");
    assert_eq!(from_b64(record["payload"].as_str().unwrap()), "hello tcp");
    assert_eq!(end["type"], "end");
}

#[tokio::test]
async fn tcp_fetch_batch_multiple_records() {
    let dir = TempDir::new().unwrap();
    let (port, _reg) = start_test_broker(&dir).await;
    let (mut reader, mut writer) = connect(port).await;

    send(&mut writer, r#"{"type":"create_topic","topic":"b","partitions":1}"#).await;
    recv(&mut reader).await; // topic_created

    for i in 0..5_u32 {
        let p = b64(&i.to_string());
        send(
            &mut writer,
            &format!(r#"{{"type":"produce","topic":"b","payload":"{p}"}}"#),
        )
        .await;
        recv(&mut reader).await; // produced
    }

    send(
        &mut writer,
        r#"{"type":"fetch_batch","topic":"b","partition":0,"offset":1,"max_count":3}"#,
    )
    .await;

    let r0 = recv_json(&mut reader).await;
    let r1 = recv_json(&mut reader).await;
    let r2 = recv_json(&mut reader).await;
    let end = recv_json(&mut reader).await;

    assert_eq!(r0["offset"], 1);
    assert_eq!(from_b64(r0["payload"].as_str().unwrap()), "1");
    assert_eq!(r1["offset"], 2);
    assert_eq!(r2["offset"], 3);
    assert_eq!(end["type"], "end");
}

#[tokio::test]
async fn tcp_metadata() {
    let dir = TempDir::new().unwrap();
    let (port, _reg) = start_test_broker(&dir).await;
    let (mut reader, mut writer) = connect(port).await;

    send(&mut writer, r#"{"type":"create_topic","topic":"x","partitions":2}"#).await;
    recv(&mut reader).await;

    send(&mut writer, r#"{"type":"metadata"}"#).await;
    let resp = recv_json(&mut reader).await;
    assert_eq!(resp["type"], "metadata");
    let topics = resp["topics"].as_array().unwrap();
    assert_eq!(topics.len(), 1);
    assert_eq!(topics[0]["name"], "x");
    assert_eq!(topics[0]["partitions"], 2);
}

#[tokio::test]
async fn tcp_error_on_unknown_topic() {
    let dir = TempDir::new().unwrap();
    let (port, _reg) = start_test_broker(&dir).await;
    let (mut reader, mut writer) = connect(port).await;

    send(
        &mut writer,
        &format!(r#"{{"type":"produce","topic":"nope","payload":"{}"}}"#, b64("x")),
    )
    .await;
    let resp = recv_json(&mut reader).await;
    assert_eq!(resp["type"], "error");
}

#[tokio::test]
async fn tcp_parse_error_does_not_close_connection() {
    let dir = TempDir::new().unwrap();
    let (port, _reg) = start_test_broker(&dir).await;
    let (mut reader, mut writer) = connect(port).await;

    // Send garbage.
    send(&mut writer, "not json at all").await;
    let resp = recv_json(&mut reader).await;
    assert_eq!(resp["type"], "error");

    // Connection must still be alive — we can send another request.
    send(&mut writer, r#"{"type":"create_topic","topic":"y","partitions":1}"#).await;
    let resp2 = recv_json(&mut reader).await;
    assert_eq!(resp2["type"], "topic_created");
}

#[tokio::test]
async fn tcp_multiple_concurrent_connections() {
    let dir = TempDir::new().unwrap();
    let (port, _reg) = start_test_broker(&dir).await;

    // Create the topic first via a dedicated connection.
    let (mut reader, mut writer) = connect(port).await;
    send(&mut writer, r#"{"type":"create_topic","topic":"c","partitions":1}"#).await;
    recv(&mut reader).await;
    drop((reader, writer));

    // Spawn 4 concurrent connections, each producing 10 messages.
    let mut handles = vec![];
    for i in 0..4_u32 {
        handles.push(tokio::spawn(async move {
            let (mut reader, mut writer) = connect(port).await;
            for j in 0..10_u32 {
                let payload = b64(&format!("{i}-{j}"));
                send(
                    &mut writer,
                    &format!(r#"{{"type":"produce","topic":"c","payload":"{payload}"}}"#),
                )
                .await;
                recv(&mut reader).await;
            }
        }));
    }
    for h in handles {
        h.await.unwrap();
    }

    // All 40 messages should be in partition 0.
    let (mut reader, mut writer) = connect(port).await;
    send(
        &mut writer,
        r#"{"type":"fetch_batch","topic":"c","partition":0,"offset":0,"max_count":50}"#,
    )
    .await;
    let mut count = 0usize;
    loop {
        let resp = recv_json(&mut reader).await;
        match resp["type"].as_str() {
            Some("record") => count += 1,
            Some("end") => break,
            _ => panic!("unexpected: {resp}"),
        }
    }
    assert_eq!(count, 40);
}
