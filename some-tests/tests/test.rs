use some_tests::{get_address_i32, type_of};
use std::{i32, ops::RangeInclusive};

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

#[test]
fn arithmetic_type_casting() {
    // 1024 + 255 + 63 + 255
    // perform math operation on diff number system
    let v = 1_024 + 0xFF + 0o77 + 0b1111_1111;
    assert_eq!(v, 1597);

    let v: u16 = 38_u8 as u16;
    assert_eq!(v, 38);

    // the result of any arithmetic operation we perform has to be always the same type as the two operators
    let x = (i32::MAX as i64) + 1;
    let y = 10_i32;
    let z = x as i32 / y;
    // there is an overflow but it is not caught
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
    // expression is a function, macro, {} due to they return something
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
    // the next line is unreachable
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
    // Rather that return None, we use a diverging function instead
    // TODO check it out
    never_return();
}

#[test]
fn get_address_test() {
    let v = 4;
    println!("{:p}", &v);
    let addr = get_address_i32(&v);
    println!("{}", addr);
    // TODO question: how to test it is valid addres or it is the same address?
}
