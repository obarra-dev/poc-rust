cargo fmt
cargo add tokio 
cargo add tokio -F macros -F rt-multi-thread


cargo install cargo-watch
cargo watch -x run

cargo doc
cargo doc --open

CREATE TABLE tasks (
task_id SERIAL PRIMARY KEY,
name VARCHAR NOT NULL,
priority INT
);


