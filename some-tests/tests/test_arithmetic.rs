use some_tests::type_of;

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
