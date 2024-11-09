#[test]
fn devide_0() {
    // TODO why &'static str?
    fn devide(x: i32, y: i32) -> Result<i32, &'static str> {
        if y == 0 {
            return Err("attempt to divide by zero");
        }

        Ok(x / y)
    }
    let r = devide(4, 0);
    match r {
        Ok(_) => panic!("this will not happen"),
        Err(e) => assert_eq!(e, "attempt to divide by zero"),
    }

    let r = devide(4, 2);
    match r {
        Ok(v) => assert_eq!(v, 2),
        Err(_) => panic!("this will not happen"),
    }
}

#[test]
fn question_mark_operator() {
    // ? automatically returns the error if there is one
    fn parse_with_question(s: &str) -> Result<i32, std::num::ParseIntError> {
        let n = s.parse::<i32>()?;
        Ok(n + 1)
    }
    let r = parse_with_question("4");
    match r {
        Ok(v) => assert_eq!(v, 5),
        Err(_) => panic!("this will not happen"),
    }

    // if we do not want to use ? we can use match, it is the same but more verbose
    fn parse_with_match(s: &str) -> Result<i32, std::num::ParseIntError> {
        let n = match s.parse::<i32>() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok(n + 1)
    }
    let r = parse_with_match("4");
    match r {
        Ok(v) => assert_eq!(v, 5),
        Err(_) => panic!("this will not happen"),
    }

    // nothing special, just return the same result
    fn parse_simple(s: &str) -> Result<i32, std::num::ParseIntError> {
        s.parse::<i32>()
    }
    let r = parse_simple("4");
    match r {
        Ok(v) => assert_eq!(v, 4),
        Err(_) => panic!("this will not happen"),
    }
}

#[test]
fn map_and_and_then() {
    // map is used to transform the value of a Result
    let r = "4".parse::<i32>().map(|v| v + 2);
    assert_eq!(r.unwrap(), 6);

    // and_then returns in a closure itself
    // and_then is used to transform the value of a Result and return a Result
    // TODO investigate more
    let r = "4".parse::<i32>().and_then(|v| Ok(v + 2));
    assert_eq!(r.unwrap(), 6);

    // map and and_then can be chained
    // it is better to use than pattern matching since pattern matching is more verbose
    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, std::num::ParseIntError> {
        n1_str
            .parse::<i32>()
            .and_then(|n1| n2_str.parse::<i32>().map(|n2| n1 * n2))
    }

    let r = multiply("4", "2");
    assert_eq!(r.unwrap(), 8);

    let r = multiply("4", "a");
    assert_eq!(r.unwrap_err().to_string(), "invalid digit found in string");

    let r = multiply("o", "4");
    assert_eq!(r.unwrap_err().to_string(), "invalid digit found in string");
}

#[test]
fn type_alias_for_errors() {
    // type alias for a simple custom type
    type MyRes<i32> = Result<i32, std::num::ParseIntError>;

    fn parse(n1_str: &str) -> MyRes<i32> {
        n1_str.parse::<i32>()
    }

    let r: MyRes<i32> = parse("4");
    assert_eq!(r.unwrap(), 4);

    let r: MyRes<i32> = parse("a");
    assert_eq!(r.unwrap_err().to_string(), "invalid digit found in string");
}
