#![feature(ip_addr)]

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Write;

fn handle_client(mut stream: TcpStream) {
    let ip = stream.peer_addr().unwrap().ip().to_string() + "\n";
    let _ = stream.write(ip.as_bytes());
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    for stream in listener.incoming() {
        match stream {
            Err(e) => { println!("failed: {}", e) }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
        }
    }
}
