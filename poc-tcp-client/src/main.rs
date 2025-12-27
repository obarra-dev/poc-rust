use std::io::Write;
use std::net::TcpStream;

fn main() {
    run_client("localhost", &7878).unwrap();
    println!("Message sent!");
}

fn run_client(host: &str, port: &u16) -> Result<(), String> {
    let addr = format!("{}:{}", host, port);
    let mut client = TcpStream::connect(addr.as_str())
        .map_err(|_| format!("Failed to connect to {}", addr))?;

    client.write(b"Omar rules from client")
        .map_err(|_| format!("Failed to write to client: {}", addr))?;
    Ok(())
}