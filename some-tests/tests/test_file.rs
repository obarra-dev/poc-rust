use std::{
    fs::File,
    io::{self, Read},
};

#[test]
fn read_file() {
    // using pattern matching
    let s = read_file_by_pattern_matching();
    // using ? operator
    let s1 = read_file_by_question_mark_op();
    let result = s.unwrap_err().to_string();
    assert_eq!(result, s1.unwrap_err().to_string());
    assert_eq!(
        result,
        "The system cannot find the file specified. (os error 2)"
    );
}

fn read_file_by_pattern_matching() -> Result<String, io::Error> {
    // TODO where should I put the test.txt file?
    let f = File::open("test.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error),
    }
}

fn read_file_by_question_mark_op() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("test.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
