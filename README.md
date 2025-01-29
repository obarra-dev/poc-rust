# poc-rust

## commands
rustc --version
cargo --version
rustup --version

cargo new hello
cargno new --lib my_rust_proyect
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


cargo init rustLib --lib
cargo login
cargo publish




generic
trais
lifetimes