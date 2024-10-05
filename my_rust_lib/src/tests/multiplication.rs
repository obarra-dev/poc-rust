use crate::multi;

#[test]
fn it_multi() {
    let result = multi(4, 4);
    assert_eq!(result, 16)
}

#[test]
fn failing_test() {
    panic!("throw a panic by itself");
}