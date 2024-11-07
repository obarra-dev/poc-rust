use core::num;
use std::{fs, io};

use some_tests::type_of;

#[test]
fn from_into_implementations() {
    let str = "omar";
    // From is implemented for String
    let s = String::from(str);
    // so we have automatic implementation of into
    let s2: String = str.into();
    assert_eq!(s, s2);

    // other way
    let s2 = str.to_string();
    assert_eq!(s, s2);

    // From is implemented for i32
    // false is 0 and true is 1
    let f = i32::from(false);
    assert_eq!(f, 0);
    let f: i32 = false.into();
    assert_eq!(f, 0);

    let f: u32 = 'a'.into();
    assert_eq!(f, 97);

    let f: String = 'a'.into();
    assert_eq!(f, "a");
}

#[test]
fn from_for_custom_type() {
    #[derive(Debug, PartialEq)]
    struct Number {
        value: i32,
    }
    let num1 = Number { value: 30 };
    let num2 = Number { value: 30 };

    assert_eq!(num1, num2);

    impl From<i32> for Number {
        fn from(n: i32) -> Self {
            Self { value: n }
        }
    }

    let num_from = Number::from(30);
    assert_eq!(30, num_from.value);
    assert_eq!(num1, num_from);

    // if from is implemented for a type, into is implemented for that type automatically
    // but we have to specify the type
    let num_into: Number = 30.into();
    assert_eq!(30, num_into.value);
    assert_eq!(num1, num_into);
}

#[test]
fn from_used_for_error_handling() {
    #[derive(Debug)]
    enum CliError {
        IoError(io::Error),
        ParseError(num::ParseIntError),
    }

    impl From<io::Error> for CliError {
        fn from(error: io::Error) -> Self {
            CliError::IoError(error)
        }
    }

    impl From<num::ParseIntError> for CliError {
        fn from(error: num::ParseIntError) -> Self {
            CliError::ParseError(error)
        }
    }

    fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
        // ? automatically converts io:Error to CliError
        let contents: String = fs::read_to_string(&file_name)?;
        // ? automatically converts num::ParseIntError to CliError
        let n: i32 = contents.trim().parse()?;
        Ok(n)
    }

    let r = open_and_parse_file("file_name");
    match r {
        Ok(n) => println!("n: {}", n),
        Err(e) => assert_eq!(
            type_of(&e),
            "test_traits_from_try_from::from_used_for_error_handling::CliError"
        ),
    }
}

#[test]
fn try_into() {
    use std::convert::TryInto;

    let x = 5;
    // try_into has a method into
    // hence TryInto has a method ?
    let y: Result<u8, _> = x.try_into();
    assert_eq!(y, Ok(5));

    let x = 256;
    let y: u8 = match x.try_into() {
        Ok(v) => v,
        Err(_) => 0,
    };
    assert_eq!(y, 0);
}

#[test]
fn try_from_for_custom_type() {
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let r: Result<EvenNumber, _> = 8.try_into();
    assert_eq!(r, Ok(EvenNumber(8)));
    let r: Result<EvenNumber, _> = 5.try_into();
    assert_eq!(r, Err(()));
}

#[test]
fn display_trait_to_convert_any_type_to_string() {
    use std::fmt;

    struct Circle {
        radius: f64,
    }

    // implementing fmt::Display trait automatically provides to_string method
    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "My own Circle has radius {}", self.radius)
        }
    }

    let c = Circle { radius: 5.0 };
    assert_eq!(format!("{}", c), "My own Circle has radius 5");
    assert_eq!(c.to_string(), "My own Circle has radius 5");
}

#[test]
fn parce_and_from_str() {
    // we can use parse method to convert string to a number, this is because FromStr is implemented for i32
    // also known as parse turbofish
    let parsed = "5".parse::<i32>();
    assert_eq!(parsed, Ok(5));
    let parsed = "5".parse::<i32>().unwrap();
    assert_eq!(parsed, 5);

    let parsed: i32 = "5".parse().unwrap();
    assert_eq!(parsed, 5);

    // FromStr is implemented for i32
    use std::str::FromStr;
    let parsed: i32 = i32::from_str("5").unwrap();
    assert_eq!(parsed, 5);
}
