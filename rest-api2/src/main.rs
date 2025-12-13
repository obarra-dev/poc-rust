use postgres::{ Client, NoTls};
use std::net::{TcpListener, TcpStream};
use std::io::{ Read, Write};
use std::string::ToString;
use serde::{Deserialize, Serialize};

// TODO how derive works?
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
const BAD_REQUEST_RESPONSE: &str = "HTTP/1.1 400 BAD REQUEST\r\n\r\n";
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

fn set_database() -> Result<(), postgres::Error> {
    // TODO how to return the error only?

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
                r if r.starts_with("POST /users") => handle_post(r),
                r if r.starts_with("GET /users/") => handle_get(r),
                r if r.starts_with("GET /users") => handle_get_all(),
                r if r.starts_with("PUT /users/") => handle_put(r),
                r if r.starts_with("DELETE /users/") => handle_delete(r),
                _ => (BAD_REQUEST_RESPONSE.to_string(), "400 method or resource not valid".to_string()), // TODO to_string is necessar?
            };

            stream.write_all(format!("{}\n{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Failed to read from stream to buffer: {}", e);
        },
    }
}

// TODO why sometimes return str other String?
fn handle_post(request: &str) -> (String, String) {
    match (get_user_request_body(&request), Client::connect(DB_URL, NoTls)) {
        // TODO why client is mut?
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

fn handle_get(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) =>
            match client.query_one("SELECT * FROM users WHERE id = $1", &[&id]) {
                Ok(row) => {
                    let user = User {
                        id: row.get(0),
                        name: row.get(1),
                        email: row.get(2),
                    };

                    (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
                }
                _ => (NOT_FOUND_RESPONSE.to_string(), "User not found".to_string()),
            }

        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(), "Error parsing user ID or getting db connection".to_string()),
    }
}

fn handle_get_all() -> (String, String) {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut users:Vec<User> = Vec::new();
            for row in client.query("SELECT id, name, email FROM users", &[]).unwrap() {
                users.push(User{
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                })
            }

            (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
        }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(), "Error getting db connection".to_string()),
    }
}

fn handle_put(request: &str) -> (String, String) {
    match
    (
        get_id(&request).parse::<i32>(),
        get_user_request_body(&request),
        Client::connect(DB_URL, NoTls),
    )
    {
        (Ok(id), Ok(user), Ok(mut client)) => {
            client
                .execute(
                    "UPDATE users SET name = $1, email = $2 WHERE id = $3",
                    &[&user.name, &user.email, &id]
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "User updated".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(), "Error parsing user ID or parsing user or getting db connection".to_string()),
    }
}

fn handle_delete(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) => {
            let rows_affected = client.execute("DELETE FROM users WHERE id = $1", &[&id]).unwrap();
            if rows_affected == 0 {
                return (NOT_FOUND_RESPONSE.to_string(), "User not found".to_string());
            }

            (OK_RESPONSE.to_string(), "User deleted".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(), "Error parsing user ID or getting db connection".to_string()),
    }
}

fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    // request has first header and then lastly body
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

fn get_id(request: &str) -> &str {
    // find the id and return it as &str
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}