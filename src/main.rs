use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;

fn handle_client(mut stream: TcpStream) {
    // Read the request
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            // Send the response
            let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
            match stream.write(response.as_bytes()) {
                Ok(_) => {
                    stream.flush().unwrap();
                }
                Err(e) => {
                    eprintln!("Failed to write to stream: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}