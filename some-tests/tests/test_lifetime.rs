#[test]
fn read_file() {
    // using pattern matching
    let s = read_file_by_pattern_matching();
    // using ? operator
    let s1 = read_file_by_question_mark_op();
    let result = s.unwrap_err().to_string();
    assert_eq!(result, s1.unwrap_err().to_string());
    assert_eq!(result, "The system cannot find the file specified. (os error 2)");

}
