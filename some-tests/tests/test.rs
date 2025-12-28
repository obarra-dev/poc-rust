use some_tests::{get_address_i32, type_of};
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
    let addr = get_address_i32(&v);
    println!("{}", addr);
    // question: how to test it is valid addres or it is the same address?
}
