use std::ops::RangeInclusive;

#[test]
fn compound_data_type_tuple() {
    // can have different types
    let tuple = ('o', true, 4);
    // member can be extracted by index
    assert_eq!(tuple.2, 4);
    assert_eq!(tuple.1, true);
    assert_eq!(tuple.0, 'o');

    let mut tuple = ('o', false, 5);
    assert_eq!(tuple.0, 'o');
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
}

#[test]
fn tuple_as_argument_and_return_value() {
    // tuples can be used as arguments and return values
    fn sum_multiply(t: (i32, i32)) -> (i32, i32) {
        let (a, b) = t;
        (a + b, a - b)
    }

    let tuple = sum_multiply((4, 2));
    assert_eq!(tuple, (6, 2));

    // tuple from 1 to 4 inclusive
    assert_eq!((1..=4), RangeInclusive::new(1, 4));
}