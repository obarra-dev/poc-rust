#[test]
fn type_alias() {
    type MyTypeU8 = u8;
    let x: MyTypeU8 = 4;
    assert_eq!(x, 4);

    let y: u8 = 4;
    assert_eq!(x, y);

    // type alias for Result
    type MyRes<i32> = Result<i32, std::num::ParseIntError>;
    let r: MyRes<i32> =  "omar".parse::<i32>();
    assert_eq!(r.unwrap_err().to_string(), "invalid digit found in string");
    
    let r: MyRes<i32> =  "4".parse::<i32>();
    assert_eq!(r.unwrap(), 4);
    
}
