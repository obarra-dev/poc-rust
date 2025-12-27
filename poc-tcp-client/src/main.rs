use std::env;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::process;
use std::thread;

type Port = u16;

fn main() {
    let mut args = env::args();
    let program_name = args.next().unwrap_or("test".to_string());

    let host = args.next().unwrap_or_else(|| {
        println!("usage: {} HOST PORT", program_name);
        process::exit(-1);
    });

    let port = args
        .next()
        .unwrap_or_else(|| {
            println!("usage: {} HOST PORT", program_name);
            process::exit(-1);
        })
        .parse::<Port>()
        .unwrap_or_else(|error| {
            writeln!(io::stderr(), "{}: error: {}", program_name,
                format!("invalid port number: {}", error)
            );
            println!("usage: {} HOST PORT", program_name);
            process::exit(-1);
        });

    let mut client_stream = TcpStream::connect((host.as_str(), port))
        .unwrap_or_else(|error| {
        writeln!(io::stderr(), "{}: error: {}", program_name, error);
        process::exit(-1);
    });

    let mut input_stream = client_stream.try_clone().unwrap();
    thread::spawn(move || {
        let mut client_buffer = [0u8; 1024];
        loop {
            match input_stream.read(&mut client_buffer) {
                Ok(n) => {
                    if n == 0 {
                        process::exit(0);
                    } else {
                        io::stdout().write(&client_buffer).unwrap();
                        io::stdout().flush().unwrap();
                    }
                }
                Err(error) => {
                    writeln!(io::stderr(), "{}: error: {}", program_name, error);
                    process::exit(-1);
                }
            }
        }
    });

    let output_stream = &mut client_stream;
    let mut user_buffer = String::new();

    loop {
        io::stdin().read_line(&mut user_buffer).unwrap();

        output_stream.write(user_buffer.as_bytes()).unwrap();
        output_stream.flush().unwrap();
    }
}
