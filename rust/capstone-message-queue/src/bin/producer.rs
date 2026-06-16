//! Producer CLI — sends messages to the broker.
//!
//! Usage:
//!   producer --broker <host:port> --topic <name> [--message <text>]
//!
//! If --message is omitted, reads lines from stdin and produces each as a
//! separate message.
//!
//! Each message is base64-encoded for the JSON protocol. The raw text bytes
//! are sent as the payload.

use std::io::{BufRead, Write};

use base64::{engine::general_purpose::STANDARD, Engine as _};

fn usage() -> ! {
    eprintln!("Usage: producer --broker <host:port> --topic <name> [--message <text>]");
    std::process::exit(1);
}

fn parse_args() -> (String, String, Option<String>) {
    let mut broker = None::<String>;
    let mut topic = None::<String>;
    let mut message = None::<String>;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--broker" => broker = Some(args.next().unwrap_or_else(|| usage())),
            "--topic" => topic = Some(args.next().unwrap_or_else(|| usage())),
            "--message" => message = Some(args.next().unwrap_or_else(|| usage())),
            _ => usage(),
        }
    }
    (
        broker.unwrap_or_else(|| usage()),
        topic.unwrap_or_else(|| usage()),
        message,
    )
}

fn main() {
    let (broker, topic, message) = parse_args();

    let mut stream = std::net::TcpStream::connect(&broker)
        .unwrap_or_else(|e| panic!("cannot connect to {broker}: {e}"));

    let produce_line = |stream: &mut std::net::TcpStream, text: &str| {
        let payload = STANDARD.encode(text.as_bytes());
        let req = format!("{{\"type\":\"produce\",\"topic\":\"{topic}\",\"payload\":\"{payload}\"}}\n");
        stream.write_all(req.as_bytes()).unwrap();
        stream.flush().unwrap();

        // Read the single response line.
        let mut reader = std::io::BufReader::new(stream.try_clone().unwrap());
        let mut resp = String::new();
        reader.read_line(&mut resp).unwrap();
        print!("{resp}");
    };

    if let Some(msg) = message {
        produce_line(&mut stream, &msg);
    } else {
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            let text = line.unwrap();
            produce_line(&mut stream, &text);
        }
    }
}
