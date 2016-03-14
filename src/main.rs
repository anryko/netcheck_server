use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Write;

extern crate libc;
use libc::funcs::posix88::unistd::{setuid, setgid};

fn handle_client(mut stream: TcpStream) {
    let ip = stream.peer_addr().unwrap().ip().to_string() + "\n";
    let _ = stream.write(ip.as_bytes());
}

unsafe fn drop_privileges() {
    setgid(65534);
    setuid(65534);
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();

    unsafe {
        drop_privileges();
    }

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
