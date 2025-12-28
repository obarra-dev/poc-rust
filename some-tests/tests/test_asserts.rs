#[test]
fn assert_with_error_message() {
    let actual = String::from("Hello Barra!");
    assert!(
        actual.contains("Barra"),
        "It does not contain name, value was {}",
        actual
    )
}

#[test]
#[should_panic]
fn assert_panic() {
    panic!("Guess value must not be 0, got {}", 0)
}

#[test]
#[should_panic(expected = "Guess value must be less than 100")]
fn assert_panic_with_message() {
    panic!("Guess value must be less than 100")
}

// test returns a result type, TODO how to check this?
// #[test]
fn it_works() -> Result<(), String> {
    if 2 + 3 == 4 {
        Ok(())
    } else {
        Err(String::from("Some silly error"))
    }
}
