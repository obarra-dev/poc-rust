# poc-rust

## commands
rustc --version
cargo --version
rustup --version

cargo new hello
cargo new --lib my_rust_proyect
cargo init rustLib2 --lib
cargo run
cargo test
cargo test --quiet
cargo test --package my_rust_lib --lib -- tests --show-output 
cargo test --package my_rust_lib --lib -- tests::it_works_assuredly --exact --show-output 
cargo test --package my_rust_lib --lib -- tests::adder --show-output 

cargo test --package some-tests --test test_pattern_matching -- if_let_test --exact --show-output 
cargo test --package some-tests --test test_pattern_matching --  --show-output 
cargo test --package some-tests --test test_methods --  --show-output 

rustfmt .\src\operations.rs 

cargo install chrono@0.4.39 ??

cargo add chrono@0.4.39
cargo add serde@1.0.217
cargo add anyhow@1.0.95
cargo add uuid --feature v4,v6

cargo add mylibrary@0.1.0
cargo add marionette@0.6.0
## yanked lib
cargo add marionette@0.2.1 

cargo yank --vers 0.1.0
cargo yank --vers 0.1.0 --registry cargo-hosted-my --undo

cargo yank rustLib@0.1.0 --registry=cargo-hosted-my
cargo yank rustLib@0.1.0 --registry=cargo-hosted-my --undo


## build by default is dev
cargo build
cargo build --release

cargo doc --open


cargo login
cargo login basic mytoken  --registry cargo-hosted-my
cargo login  --registry cargo-hosted-my

cargo publish
cargo publish --allow-dirty --registry cargo-hosted-my


## clean all
cargo clean                                           
rm Cargo.lock                                         
rm -r ~/.cargo/registry                               


generic
trais
lifetimes


---

[registries]
cargo-proxy = { index = "sparse+http://localhost:8081/repository/cargo-proxy/" }
cargo-hosted = { index = "sparse+http://localhost:8081/repository/cargo-hosted/" }
cargo-group = { index = "sparse+http://localhost:8081/repository/cargo-group/" }

cargo-proxy-my = { index = "sparse+http://localhost:8081/repository/cargo-proxy-my/" }
cargo-hosted-my = { index = "sparse+http://localhost:8081/repository/cargo-hosted-my/" }
cargo-group-my = { index = "sparse+http://localhost:8081/repository/cargo-group/" }



[source.crates-io]
replace-with = "cargo-group-my" # Change it dependening of the test case

[registry]
global-credential-providers = ["cargo:token", "cargo:libsecret", "cargo:macos-keychain", "cargo:wincred"]

----

https://index.crates.io


cat config.json 
{
  "dl": "https://static.crates.io/crates",
  "api": "https://crates.io"
}
