//! Consumer CLI — fetches messages from the broker.
//!
//! Usage:
//!   consumer --broker <host:port> --topic <name> --partition <id>
//!            [--offset <n>] [--count <n>]
//!
//! Fetches up to `count` records (default: 100) from the given
//! topic/partition starting at `offset` (default: 0). Decodes each payload
//! from base64 and prints it as a UTF-8 string (with a hex fallback for
//! binary data).

use std::io::{BufRead, Write};

use base64::{engine::general_purpose::STANDARD, Engine as _};

fn usage() -> ! {
    eprintln!(
        "Usage: consumer --broker <host:port> --topic <name> \
         --partition <id> [--offset <n>] [--count <n>]"
    );
    std::process::exit(1);
}

fn parse_args() -> (String, String, u32, u64, usize) {
    let mut broker = None::<String>;
    let mut topic = None::<String>;
    let mut partition = None::<u32>;
    let mut offset: u64 = 0;
    let mut count: usize = 100;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--broker" => broker = Some(args.next().unwrap_or_else(|| usage())),
            "--topic" => topic = Some(args.next().unwrap_or_else(|| usage())),
            "--partition" => {
                partition = Some(
                    args.next()
                        .unwrap_or_else(|| usage())
                        .parse()
                        .unwrap_or_else(|_| usage()),
                )
            }
            "--offset" => {
                offset = args
                    .next()
                    .unwrap_or_else(|| usage())
                    .parse()
                    .unwrap_or_else(|_| usage())
            }
            "--count" => {
                count = args
                    .next()
                    .unwrap_or_else(|| usage())
                    .parse()
                    .unwrap_or_else(|_| usage())
            }
            _ => usage(),
        }
    }
    (
        broker.unwrap_or_else(|| usage()),
        topic.unwrap_or_else(|| usage()),
        partition.unwrap_or_else(|| usage()),
        offset,
        count,
    )
}

fn main() {
    let (broker, topic, partition, offset, count) = parse_args();

    let mut stream = std::net::TcpStream::connect(&broker)
        .unwrap_or_else(|e| panic!("cannot connect to {broker}: {e}"));

    // Send a fetch_batch request.
    let req = format!(
        "{{\"type\":\"fetch_batch\",\"topic\":\"{topic}\",\
         \"partition\":{partition},\"offset\":{offset},\"max_count\":{count}}}\n"
    );
    stream.write_all(req.as_bytes()).unwrap();
    stream.flush().unwrap();

    // Read responses until we see {"type":"end"} or {"type":"error",...}.
    let reader = std::io::BufReader::new(stream);
    let mut record_count = 0usize;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        let v: serde_json::Value = serde_json::from_str(&line).unwrap_or_else(|_| {
            eprintln!("malformed response: {line}");
            std::process::exit(1);
        });
        match v["type"].as_str() {
            Some("record") => {
                let off = v["offset"].as_u64().unwrap_or(0);
                let payload_b64 = v["payload"].as_str().unwrap_or("");
                let bytes = STANDARD.decode(payload_b64).unwrap_or_default();
                let text = std::str::from_utf8(&bytes)
                    .map(|s| s.to_owned())
                    .unwrap_or_else(|_| format!("<binary: {}>", hex(&bytes)));
                println!("offset={off}  {text}");
                record_count += 1;
            }
            Some("end") => {
                eprintln!("{record_count} record(s) received.");
                break;
            }
            Some("error") => {
                eprintln!("error: {}", v["message"].as_str().unwrap_or("?"));
                std::process::exit(1);
            }
            _ => eprintln!("unexpected: {line}"),
        }
    }
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect::<Vec<_>>().join("")
}
