use std::net::TcpListener;
use std::thread;
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();

    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            stream.write(b"Hello, that's an answer.\r\n").unwrap();
        });
    }
 }
