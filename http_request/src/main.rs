use std::io::{Read, Write};
use std::net::*;

fn handle_client(mut stream: &TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", request);

    let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
    Vec::from([2,2]);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(&stream);
    }
}