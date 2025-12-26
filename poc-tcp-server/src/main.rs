use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    // creating server socket
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("Failed to bind to address");

    println!("Server listening on port 7878");
    // blocking operation
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            },
            Err(e) => {
                // stderr
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read from stream");
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);

    let response = "Omar rules!!".as_bytes();
    stream.write(response).expect("Failed to write to stream");
}
