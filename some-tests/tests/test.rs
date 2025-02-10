use some_tests::type_of;
use std::{i32, ops::RangeInclusive};


#[test]
fn compound_data_type_tuple() {
    // can have different types
    let tuple = ('o', true, 4);
    // member can be extracted by index
    assert_eq!(tuple.2, 4);
    assert_eq!(tuple.1, true);
    assert_eq!(tuple.0, 'o');

    let mut tuple = ('o', false, 5);
    tuple.0 = 'd';
    assert_eq!(tuple.2, 5);
    assert_eq!(tuple.1, false);
    assert_eq!(tuple.0, 'd');

    let tuple = (1u8, 3i64, (1, 2), "omar", String::from("barra"));
    assert_eq!(tuple.0, 1);
    assert_eq!(tuple.1, 3);
    // tuples can be tuple's member
    assert_eq!(tuple.2, (1, 2));
    assert_eq!(tuple.3, "omar");
    assert_eq!(tuple.4, "barra");

    // seems 12 is the max number of elements in a tuple
    let max_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    assert_eq!(max_long_tuple, (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12));

    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // does not compile, Long tuples cannot be printed or asserted
    // assert_eq!(too_long_tuple, (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13));
    // println!("{:?}", too_long_tuple);

    // tuples can used as arguments and return values
    fn sum_multiply(t: (i32, i32)) -> (i32, i32) {
        let (a, b) = t;
        (a + b, a - b)
    }

    let tuple = sum_multiply((4, 2));
    assert_eq!(tuple, (6, 2));

    // tuple from 1 to 4 inclusive
    assert_eq!((1..=4), RangeInclusive::new(1, 4));
}

#[test]
fn size_of_val() {
    let c = 'o';
    let s = std::mem::size_of_val(&c);
    assert_eq!(s, 4);

    let c = '🙀';
    let s = std::mem::size_of_val(&c);
    assert_eq!(s, 4);
}

#[test]
fn unit_type() {
    fn implicitly_ret_unit() {};
    fn explicitly_ret_unit() -> () {};

    // unit represents empty tuple of 0 bytes
    // used to return  nothing in expressions or functions
    let unit = ();
    let s = std::mem::size_of_val(&unit);
    assert_eq!(s, 0);
    assert_eq!(unit, ());
    assert_eq!(explicitly_ret_unit(), ());
    assert_eq!(implicitly_ret_unit(), ());
    assert_eq!(explicitly_ret_unit(), implicitly_ret_unit());
}

struct Point {
    x: i32,
    y: i32,
}

#[test]
fn tuple_destructuring() {
    let (x, y, z) = ('o', true, 4);
    assert_eq!(z, 4);
    assert_eq!(y, true);
    assert_eq!(x, 'o');

    let (mut x, y, z) = ('o', true, 4);
    x = 'd';
    assert_eq!(z, 4);
    assert_eq!(y, true);
    assert_eq!(x, 'd');

    // can change the order
    let (z, y, x) = ('o', true, 4);
    assert_eq!(x, 4);
    assert_eq!(y, true);
    assert_eq!(z, 'o');

    // Destructuring assignments
    // one line instead of 2 lines
    let (a, b);
    // let is no neded, destructuring the first one
    (a, ..) = (3, 4);
    // destructuring the second one
    [.., b] = [1, 2];
    assert_eq!([a, b], [3, 2]);
}

#[test]
fn destructuring_struct() {
    let p = Point { x: 1, y: 2 };
    // destructuring the struct
    let Point { x, y } = p;
    assert_eq!(x, 1);
    assert_eq!(y, 2);
}

#[test]
fn compound_data_type_array() {
    let array = [2, 3, 4];
    assert_eq!(array[2], 4);

    // the size is mandatory
    // it cannot be emtpy
    // you cannot add or remove elements
    let array_explicit: [u32; 4] = [2, 3, 4, 5];
    assert_eq!(array_explicit[3], 5);

    let mut array = [2, 3, 4];
    array[0] = 5;
    assert_eq!(array[0], 5);
}

#[test]
fn arithmetic_type_casting() {
    // 1024 + 255 + 63 + 255
    // perform math operation on diff number systemdd
    let v = 1_024 + 0xFF + 0o77 + 0b1111_1111;
    assert_eq!(v, 1597);

    let v: u16 = 38_u8 as u16;
    assert_eq!(v, 38);

    // the result of any arithmetic operation we perform has to be always the same type as the two operators
    let x = (i32::MAX as i64) + 1;
    let y = 10_i32;
    let z = x as i32 / y;
    // there is an overflow but it is not catched
    assert_eq!(z, -214_748_364);

    let cond = 3.2 > (8 as f32);
    assert_eq!(cond, false);

    // string to int
    let input = "8";
    let int_input: i64 = input.trim().parse().unwrap();
    assert_eq!(int_input, 8);

    let value: u16 = 5;
    let value_type = type_of(&value);
    assert_eq!(value_type, "u16");

    // ASCII
    assert_eq!('o' as u8, 111);
}

#[test]
fn checked_add() {
    // both would overflow, they do not compile
    // 251_u8 + 8;
    // i8::checked_add(251, 8).unwrap();

    let v = 251_u16 + 8;
    assert_eq!(v, 259);

    // this is safer
    let v = i16::checked_add(251, 8).unwrap();
    assert_eq!(v, 259);
}

#[test]
#[should_panic(expected = "assertion failed: 0.1 + 0.2 == 0.3")]
fn arithmetic_overflow() {
    // it is 0.3333...
    assert!(0.1 + 0.2 == 0.3)
}

#[test]
fn arithmetic_overflow_fixed() {
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32)
}

#[test]
fn function_and_expression() {
    // expression is a function, macro, {} due to they retuns something
    let a = {
        let x = 3;
        x + 1
    };
    assert_eq!(a, 4);

    // semicolon suppresses this expression, so it returns unit type, nothing
    let b = {
        let x = 3;
        x + 1;
    };
    assert_eq!(b, ());

    let b: i32 = my_function(4);
    assert_eq!(b, 14)
}

fn my_function(x: i32) -> i32 {
    if x > 0 {
        return x + 10;
    }

    if x < 0 {
        return x - 10;
    }

    0
}

#[test]
#[should_panic]
fn diverging_function() {
    never_return();
    // the next line is unrechable
}

fn never_return() -> ! {
    // all of them are equivalent
    unimplemented!()
    // panic!(), todo!()
}

#[test]
fn diverging_function_1() {
    // TODO
    //get_option()
}

fn get_option(a: u8) -> Option<i32> {
    let r = match a {
        1 => Some(32),
        _ => None,
    };

    return r;
    // Rather that return None, we use a divergin function instead
    // TODO check it out
    never_return();
}

#[test]
fn get_address_test() {
    let v = 4;
    println!("{:p}", &v);
    let addr = format!("{:p}", &v);
    println!("{}", addr);
    // question: how to test it is valid addres or it is the same address?
}

#[test]
fn array() {
    // infers the type is [i32; 3]
    let arr = [1, 2, 3];
    assert_eq!(arr.len(), 3);
    //size_of_val  retuns the bytes which an array occupies in memory
    assert_eq!(std::mem::size_of_val(&arr), 12);

    // all elements in an array can be inisialized with the same value at once
    let arr = [4; 4];
    assert_eq!(arr.len(), 4);
    assert_eq!(arr[0], 4);
    assert_eq!(arr[3], 4);

    // does not compile, as the array is not mutable
    // arr[0] = 4;

    let arr = ["omar", "barra"];
    // panic if the index is out of bounds
    // assert_eq!(arr[7], "omar");

    // we can use index to access the elements but is not safe
    // option is a safe way to access the elements
    // question: how to test it?
    let optional = arr.get(7);

    // a is mutable, so we can change the values
    let mut a = [1, 2, 3];
    a[0] = 4;
    assert_eq!(a[0], 4);

    // other way
    let mut a = [1, 2, 3];
    let x = &mut a[0];
    *x = 4;
    assert_eq!(a[0], 4);

    let a = [1, 2, 3];
    let x = &a[0];
    // question: &1 what is it?
    assert_eq!(x, &1);

    let mut a = [1, 2, 3];
    let x = &a[0];
    assert_eq!(x, &1);
    let y = &mut a[0];
    *y = 4;
    assert_eq!(a[0], 4);
}

#[test]
fn slice() {
    let arr = [1, 2, 3, 4, 5];
    // does not compile, error the size for values of type `[{integer}]` cannot be known at compilation time
    // let sliced = arr[1..3];

    // type is &[i32] slice of i32
    // we cannot use slice directly, we have to use the reference of the slice
    let slice = &arr[1..3];

    assert_eq!(slice.len(), 2);
    // question: why both work?
    assert_eq!(slice, [2, 3]);
    assert_eq!(slice, &[2, 3]);

    let arr = ['学', '中'];
    // 8 = 4 bytes for each character * 2 characters
    assert_eq!(std::mem::size_of_val(&arr), 8);

    let slice = &arr[..2];
    println!("{:?}", slice);
    // 16 = (4 bytes for pointer + 4 bytes of lenght ) * 2 characters
    assert_eq!(std::mem::size_of_val(&slice), 16);

    // string slice is also a slice
    let s = String::from("omar");
    let slice = &s[0..2];
    assert_eq!(slice, "om");

    let s = "学中";
    // it takes 3 bytes
    let slice = &s[0..3];
    assert_eq!(slice, "学");

    // TODO example with string slice
    // https://doc.rust-lang.org/std/string/struct.String.html#method.clear
}

#[test]
fn enum_test() {
    enum IpAddrKind {
        V4(String),
        V6(String),
    };

    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    #[derive(PartialEq, Debug)]
    enum Number {
        Zero,
        One,
        Two,
    };

    let number = Number::One;
    assert_eq!(number, Number::One);
    assert_eq!(number as u8, 1);

    #[derive(PartialEq, Debug)]
    enum NumberOdd {
        Zero = 10,
        One,
        Two,
    };
    let number = NumberOdd::Two;
    assert_eq!(number, NumberOdd::Two);
    assert_eq!(number as u8, 12);

    // each enum varian can hold its own data
    #[derive(PartialEq, Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    };
    let m = Message::Write(String::from("omar"));
    // question: how to extract the value for non primitive types?
    // assert_eq!(m as String, "omar");
    assert_eq!(m, Message::Write(String::from("omar")));
    let s = format!("{:?}", m);
    assert_eq!(s, "Write(\"omar\")");

    let m = Message::Quit;
    let s = format!("{:?}", m);
    assert_eq!(s, "Quit");
    // assert_eq!(m as u8, 1);

    let m = Message::Move { x: 1, y: 2 };
    let s = format!("{:?}", m);
    assert_eq!(s, "Move { x: 1, y: 2 }");

    let m = Message::ChangeColor(1, 2, 3);
    let s = format!("{:?}", m);
    assert_eq!(s, "ChangeColor(1, 2, 3)");

    let m = Message::Move { x: 1, y: 2 };
    if let Message::Move { x, y } = m {
        assert_eq!(x, 1);
        assert_eq!(y, 2);
    } else {
        panic!("NEVER LET THIS RUN");
    }

    let messages = [
        Message::Quit,
        Message::Move { x: 1, y: 2 },
        Message::Write(String::from("omar")),
    ];
    for m in messages {
        match m {
            Message::Quit => assert_eq!(format!("{:?}", m), "Quit"),
            Message::Move { x, y } => {
                assert_eq!(format!("{:?}", m), "Move { x: 1, y: 2 }");
                assert_eq!(x, 1);
                assert_eq!(y, 2);
            }
            Message::Write(s) => {
                // question: why partionally moved?
                // assert_eq!(format!("{:?}", m), "Write(\"omar\")");
                assert_eq!(s, "omar");
            }
            _ => panic!("NEVER LET THIS RUN"),
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[test]
fn enum_option() {
    let five = Some(5);
    let six = plus_one(five);
    assert_eq!(six.unwrap(), 6);
    // othe way to get the value, destructuring
    if let Some(i) = six {
        assert_eq!(i, 6);
    }

    let x = plus_one(None);
    assert_eq!(x, None);
    // it will panic
    // assert_eq!(x.unwrap(), 4);

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

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y.unwrap_or(0);
    assert_eq!(sum, 10);

    let x: i8 = 5;
    let y: Option<i8> = None;
    let sum = x + y.unwrap_or(0);
    assert_eq!(sum, 5);

    let x: i8 = 5;
    let y: Option<i8> = None;
    // does not compile, error: the `?` operator can only be used in a function that returns `Result` or `Option`
    // let sum = x + y?;
    // assert_eq!(sum, 5);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y?;
    // assert_eq!(sum, 10);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn enum_option_panic() {
    let x = plus_one(None);
    assert_eq!(x.unwrap(), 4);
}


