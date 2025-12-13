#[test]
fn enum_option() {
    let five = Some(5);
    let six = plus_one(five);
    assert_eq!(six.unwrap(), 6);
    // other way to get the value, destructuring
    if let Some(i) = six {
        assert_eq!(i, 6);
    }

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    assert_eq!(some_number, Some(5));
    assert_eq!(some_string, Some("a string"));
    assert_eq!(absent_number, None);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y.unwrap();
    assert_eq!(sum, 10);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn enum_option_unwrap_panic() {
    let x = plus_one(None);
    assert_eq!(x, None);
    assert_eq!(x.unwrap(), 4);
}

#[test]
fn enum_option_unwrap_or() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y.unwrap_or(0);
    assert_eq!(sum, 10);

    let x: i8 = 5;
    let y: Option<i8> = None;
    let sum = x + y.unwrap_or(0);
    assert_eq!(sum, 5);
}

#[test]
fn enum_option_with_question_mark() {
    fn sum_options(x: Option<i32>, y: Option<i32>) -> Option<i32> {
        Some(x? + y?)
    }

    assert_eq!(sum_options(Some(2), Some(3)), Some(5));
    assert_eq!(sum_options(None, Some(3)), None);
    assert_eq!(sum_options(Some(2), None), None);

    let x: i8 = 5;
    let y: Option<i8> = None;
    // does not compile, error: the `?` operator can only be used in a function that returns `Result` or `Option`
    //let sum = x + y?;
    // assert_eq!(sum, 5);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y?;
    // assert_eq!(sum, 10);
}

#[test]
fn enum_option_unwrap_or_default() {
    let maybe_number: Option<i32> = Some(4);
    let number_one = maybe_number.unwrap_or_default();
    assert_eq!(number_one, 4);

    let no_number: Option<i32> = None;
    let number_two = no_number.unwrap_or_default();
    // the default value of i32 is 0
    assert_eq!(number_two, 0);

    let maybe_name: Option<String> = None;
    let name: String = maybe_name.unwrap_or_default();
    // the default value of String is ""
    assert_eq!(name, "");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
