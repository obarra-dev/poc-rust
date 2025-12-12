cargo --version
rustc --version
rustup update
rustup self uninstall
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

cargo add serde


export DATABASE_URL="postgres://postgres:password@localhost:5432/for-rust"
cargo build