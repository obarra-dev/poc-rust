#[test]
fn if_test() {
    // if/else expression can be used in assignment
    let number = if true { 5 } else { 6 };
    assert!(number == 5);
}

#[test]
fn control_flow() {
    let a = "golang";
    let actual = if a == "java" {
        2
    } else if a == "golang" {
        4
    } else {
        1
    };

    assert_eq!(actual, 4);
}
