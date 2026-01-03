cargo add clap --features derive
cargo add strum strum_macros

cargo add strum --features derive

cargo add serde --features derive serde_json


cargo build
cd /Users/obarra/all-repos/poc-rust/poc-ls/target/debug
./poc-ls /Users/obarra/all-repos