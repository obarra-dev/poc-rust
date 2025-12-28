use std::io;
use std::io::Write;
use std::net::TcpStream;

fn main() -> Result<(), String> {
    let addr = format!("{}:{}", "localhost", 7878);
    let mut client_stream = TcpStream::connect(addr.as_str())
        .map_err(|_| format!("Failed to connect to {}", addr))?;

    client_stream.write(b"Initial message")
        .map_err(|_| format!("Failed to write to client: {}", addr))?;
    client_stream.flush().unwrap();

    let mut user_input = String::new();
    loop {
        io::stdin().read_line(&mut user_input).unwrap();
        if "END\n" == user_input {
            break;
        }
        client_stream.write(user_input.as_bytes()).unwrap();
        client_stream.flush().unwrap();
        user_input.clear();
    }

    println!("Omar rules!");
    Ok(())
}