// This works with tcp and returns an ssh banner
// Could be better with socket as there are issues with TcpListener here (see below)

use std::{thread, time};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};


fn handler(mut stream: TcpStream) {
    let mut data = [0 as u8; 64]; 
    while match stream.read(&mut data) {
        Ok(size) => {
            let mut faker: &[u8] = b"SSH-2.0-OpenSSH_8.4p1 Ubuntu-5ubuntu1.2";
            stream.write(faker);
            // println!("{:#?}", &data[0..size]); // Save to log?

            let tiem_pls = time::Duration::from_millis(10000); // Let the attacker wait
            let now = time::Instant::now();

            thread::sleep(tiem_pls);

            assert!(now.elapsed() >= tiem_pls);

            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server listening...");
    println!("{:#?}", listener.incoming());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // println!("Found an attacker: {}", stream.peer_addr().unwrap()); // This breaks if TCP connection is only syn-ed, not built. kek : ))
                thread::spawn(move|| {
                    handler(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}
