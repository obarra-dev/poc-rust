use postgres::{ Client, NoTls};
use postgres::Error as PostgresError;
use std::net::{TcpListener, TcpStream};
use std::io::{ Read, Write};
use std::string::ToString;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

// TODO how to use env?
//const DB_URL: &str = env!("DATABASE_URL");

const DB_URL: &str = "postgres://postgres:password@localhost:5432/for-rust";
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR_RESPONSE: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

fn main() {
    if let Err(e) = set_database() {
        println!("Failed to set database: {}", e);
        return;
    }

    let listener = TcpListener::bind("0.0.0.0:8088").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
                println!("OK incoming stream");
            }
            Err(e) => {
                println!("Error incoming stream: {}", e);
            },
        }
    }

    println!("Omar rules!");
}

fn set_database() -> Result<(), PostgresError> {
    // TODO how to return the error

    // if connect has error, it returns it
    let mut client = Client::connect(DB_URL, NoTls)?;

    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )")?;

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            // into request concat the content of buffer
            request.push_str(&String::from_utf8_lossy(&buffer[..size]).as_ref());

            // I think &*request is unnecessary, only &request works but I keep it
            let (status_line, content) = match &*request {
                // match guard
                r if r.starts_with("POST /users") => handle_post_request(r),
                _ => (NOT_FOUND_RESPONSE.to_string(), "404 not found".to_string()), // TODO to_string is necessar?
            };

            stream.write_all(format!("{}\n{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Failed to read from stream to buffer: {}", e);
        }, // TODO why coma?
    }
}

fn handle_post_request(request: &str) -> (String, String) {
    match (get_user_request_body(&request), Client::connect(DB_URL, NoTls)) {
        (Ok(user), Ok(mut client)) => {
            client
                .execute(
                    "INSERT INTO users (name, email) VALUES ($1, $2)",
                    &[&user.name, &user.email]
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "User created".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(), "Error parsing user or getting db connection".to_string()),
    }
}

fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    // request has first header and then lastly body
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}
