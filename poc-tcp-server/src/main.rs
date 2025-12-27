use std::io::{stdout, ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    // creating server socket
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");
    println!("Server listening on port 7878");

    // blocking operation
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
               // std::thread::spawn(|| handle_client_loop(stream));
                handle_client_while(stream).unwrap()
            }
            Err(e) => {
                // stderr
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}

fn handle_client_while(mut stream: TcpStream) -> Result<(), String> {
    let mut buffer = [0; 1024];
    let mut read_bytes = 0;

    while read_bytes == 0 {
        read_bytes = stream.read(&mut buffer).map_err(|_| "failed to read from socket")?;
        println!("Read {} bytes", read_bytes);
    }

    stdout().write(&buffer[0..read_bytes]).map_err(|_| "failed to write to stdout")?;
    stdout().flush().map_err(|_| "failed to flush from stdout")?;

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream
        .read(&mut buffer)
        .expect("Failed to read from stream");
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);

    let response = "Omar rules!!".as_bytes();
    stream.write(response).expect("Failed to write to stream");
}

fn handle_client_loop(mut stream: TcpStream) {
    let peer_addr = stream
        .peer_addr()
        .map_or_else(|_| "Unknown".to_string(), |addr| addr.to_string());
    println!("Peer address: {}", peer_addr);

    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(bytes) => {
                if bytes == 0 {
                    println!("Client {} closed connection", peer_addr);
                    break;
                }

                let request = String::from_utf8_lossy(&buffer[..bytes]);
                println!("Received request: {}", request);

                if let Err(e) = stream.write_all(&buffer[..bytes]) {
                    eprintln!("Failed to write to client {}: {}", peer_addr, e);
                    break;
                }
            }
            Err(e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => {
                match e.kind() {
                    ErrorKind::ConnectionReset => {
                        println!("Client {} reset connection", peer_addr);
                    }
                    _ => {
                        eprintln!("Read error from client {}: {}", peer_addr, e);
                    }
                }
                break;
            }
        }
    }

    println!("Connection finished for client {}", peer_addr);
}
