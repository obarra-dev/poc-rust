use std::io;
use std::io::Write;
use std::net::TcpStream;

fn main() -> Result<(), String> {
    let addr = format!("{}:{}", "localhost", 7878);
    let mut client_stream = TcpStream::connect(addr.as_str())
        .map_err(|_| format!("Failed to connect to {}", addr))?;

    client_stream.write(b"Initial message")
        .map_err(|_| format!("Failed to write to client: {}", addr))?;

    let mut user_buffer = String::new();
    loop {
        io::stdin().read_line(&mut user_buffer).unwrap();
        if "END\n" == user_buffer {
            break;
        }
        client_stream.write(user_buffer.as_bytes()).unwrap();
        client_stream.flush().unwrap();
        user_buffer.clear();
    }

    println!("Omar rules!");
    Ok(())
}