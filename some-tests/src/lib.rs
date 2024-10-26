pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// TODO to check
pub fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[cfg(test)]
mod tests {
    use core::{panic, slice};
    use std::{arch::x86_64, fmt::format, i32, ops::RangeInclusive, slice::Split};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn virables() {
        // it is immutalbe, so cannot assign twice to immutable variable
        let x = 4;
        assert_eq!(x, 4);

        // with mut, it is mutable
        let mut y = 4;
        assert_eq!(y, 4);
        y = 10;
        assert_eq!(y, 10);

        // rust allows to overrides variables, it is named shadowing
        // the first variable is shadowed by the second one
        let z = 4;
        assert_eq!(z, 4);
        let z = 20;
        assert_eq!(z, 20);

        // rust allows to overrides variables changing the type
        let a = 4;
        assert_eq!(a, 4);
        let a = "omar";
        assert_eq!(a, "omar");

        // underscore is to prevent warnings, #[allow(unused_variables)] also can be used
        let _v = 4;
        assert_eq!(_v, 4);
    }

    #[test]
    fn virables_shadowing() {
        let x = 4;
        assert_eq!(x, 4);

        {
            // interior SCOPE, this x valuse only lives here
            let x = x - 2;
            assert_eq!(x, 2);
        }

        let x = x + 1;
        assert_eq!(x, 5);
    }

    #[test]
    fn constant() {
        // compiler forces to be UPERCASE and define a type
        // a const cannot be override or redifine
        const X: u8 = 4;
        assert_eq!(X, 4);
    }

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
    fn control_flow() {
        let a = "golang";
        let actual = if a == "java" {
            2
        } else if a == "golang" {
            4
        } else {
            1
        };

        assert_eq!(actual, 4);
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

        let b = my_function(4);
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
    fn ownership() {
        // owner and other_owner are saved on stack
        let owner = 4;
        // Rust makes a copy of the value and set on other_owner, because these types have a know size (fized size) and it is cheap
        let other_owner = owner;

        assert_eq!(owner, 4);
        assert_eq!(other_owner, 4);

        // it is saved on HEAP
        let s = String::from("omar");
        // only the pointers is copied, move
        let x = s;
        assert_eq!(x, "omar");
        // it does not compile, rule 2 is violated, error is: borrow of moved value: `s`
        // so s will be dropped (ptr, len, capacity of s will be droped) and cannot be used after assigning it to X to avoid dangling pointers
        // assert_eq!(s, "omar");
    }

    #[test]
    fn ownership_deep_copy() {
        let s = String::from("omar");
        // allocate a new heap memory
        let x = s.clone();
        assert_eq!(x, "omar");
        assert_eq!(s, "omar");
    }

    #[test]
    fn ownership_heap() {
        let s = String::from("omar");
        let x = drop_string(s);
        assert_eq!(x, ());
        // does not compile
        // as s's value moves into drop_string function, that function take the ownership
        // plus drop_string has finished, all the values were dropped. So "omar" was removed from HEAP, the baking memory is freed
        // assert_eq!(s, "omar");

        // gives_owrnership moves its return value into s, so the bakin momemory is not fred
        let s = gives_owrnership();
        assert_eq!(s, "omar");

        let s = String::from("omar");
        let x = return_ownership(s);
        assert_eq!(x, "omar");
        // it does not compile, rule 2 and 3 are violated, error is: borrow of moved value: `s`
        // does not compile. As return_ownership returns the ownership to x
        // assert_eq!(s, "omar");

        // NOTE when the main funcion ends, ALL the variables will be dropped.

        // question, when re-assing a heap variable when the previos one is removed?
    }

    // does not drop string and return ownership
    fn gives_owrnership() -> String {
        String::from("omar")
    }

    // does not drop string and return ownership
    fn return_ownership(some_string: String) -> String {
        some_string
    }

    // does not drop string and return ownership
    fn drop_string(some_string: String) {}

    #[test]
    fn ownership_change_mutability() {
        // s cannot be changed, it is immutable
        let s = String::from("omar");

        // it does not compile, error: cannot borrow `s` as mutable, as it is not declared as mutable
        // s.push_str(" barra");

        // when ownership is transferred, the variable can be mutable
        let mut s1 = s;
        s1.push_str(" barra");
        assert_eq!(s1, "omar barra");
    }

    #[test]
    fn ownership_derefering() {
        // Box allows to store data on the heap
        let mut v = Box::new(4);

        // derefering
        *v = 44;
        assert_eq!(*v, 44);

        let v = 4;
        // x is a reference to v
        let x = &v;
        // derefering x to get the value of v
        assert_eq!(*x, 4);
    }

    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    #[test]
    fn ownership_complete_move() {
        let p = Person {
            name: String::from("omar"),
            age: Box::new(4),
        };

        // question how can to it?
        // assert_eq!(p, Person { name: String::from("omar"), age: Box::new(4) });

        println!("the person struct is: {:?}", p);

        let Person { name, age } = p;

        // does not compile as p is moved to name and age
        // println!("the person struct is: {:?}", p);

        assert_eq!(name, "omar");
        assert_eq!(*age, 4);
    }

    #[test]
    fn ownership_partial_move() {
        let p = Person {
            name: String::from("omar"),
            age: Box::new(4),
        };

        println!("the person struct is: {:?}", p);

        // name is moved out of person, but age is referced
        let Person { name, ref age } = p;

        // does not compile as borrow of partially moved value: `p` partial moves occurs
        // println!("the person struct is: {:?}", p);

        assert_eq!(name, "omar");
        // it works as age is a reference
        assert_eq!(*p.age, 4);
        // does not compile, as p is moved
        // assert_eq!(p.name, "omar");

        // age can be used for read only
        println!("age is: {}", age);
        // does not compile, question, why cannot assert age?
        //  assert_eq!(age, 4);

        let t = (String::from("omar"), String::from("barra"));
        // t.0 is moved to _s, so it canot be used
        let _s = t.0;
        assert_eq!(t.1, "barra");
    }

    #[test]
    fn borrowing() {
        // inmutable reference
        let s = String::from("omar");
        // & is to pass the inmutable reference, ownership is not transferred, it is borrowed
        // I can have any number of immutable references from the same variable
        // x and z are read only
        let x = &s;
        let z = &s;

        // cannot mutate immutable variable, does not compile
        // let y =&mut s;

        // I can use s, x and z without any problem because they borrowed the ownership
        let size = z.len() + x.len() + s.len();
        assert_eq!(size, 12);
        assert_eq!(s, "omar");
        assert_eq!(x, "omar");
        assert_eq!(z, "omar");
    }

    #[test]
    fn borrowing_mutable_reference() {
        // mutable reference
        let mut s = String::from("omar");
        let x = &mut s;
        x.push_str(" barra");
        assert_eq!(x, "omar barra");
        assert_eq!(s, "omar barra");

        // borrow a mutable variable as immutable, mutable to immutable reference
        // I can immutable a mutable variable
        let y = &s;
        assert_eq!(y, "omar barra");

        let mut s = String::from("omar");
        let x = &mut s;
        let y = &mut s;
        assert_eq!(s, "omar");

        // Seems I can initialize borrowing a mutable variable more that one time, but I cannot use it
        // does not compile, it violates rule 1 of borrowing
        // rule 1: we can have  one single mutable references
        // x.push_str(" barra");
        // assert_eq!(x, "omar");
        // assert_eq!(y, "omar");

        let mut s = String::from("omar");

        // x goes out of scope, so I can make a new mutable reference with no problem
        // x is dropped, but the allocated heap memory is not freed
        {
            let x = &mut s;
            x.push_str(" barra");
            assert_eq!(x, "omar barra");
        }

        let y = &mut s;
        y.push_str(" barra");
        assert_eq!(y, "omar barra barra");
    }

    #[test]
    fn borrowing_mutable_and_immutable_reference() {
        // mutable reference and immutable reference
        let mut s = String::from("omar");
        let x = &s;
        let y = &s;

        // does not compile, it violates rule 1 of borrowing
        // let z = &mut s;
        // z.push_str(" barra");

        assert_eq!(x, "omar");
        assert_eq!(y, "omar");

        // I can have multiple immutable references and one mutable reference
        // but if I use the immutable references after the mutable reference was initialized, the mutable reference does not compile
        // in this case, z is a mutable reference and it is used after x and y were used. X and y do not have to be used after this point
        let z = &mut s;
        z.push_str(" barra");
        assert_eq!(z, "omar barra");
    }

    #[test]
    fn borrowing_reference_rule_2() {
        let s = {
            let s = String::from("omar");
            // &s does not compile, it violates rule 2 of borrowing
            // rule 2: states that references must ALWAYS be valid
            // it is a dangling reference, s will be dropped, so there is not point to have a reference to it
            // borrowed value does not live long enough
            // &s

            // it is valid, as the ownership is transferred to the other s
            s
        };

        assert_eq!(s, "omar");
    }

    #[test]
    fn borrowing_reference_using_ref() {
        let s = String::from("omar");
        let x = &s;
        // ref is similar to &, but there are some minior differences in pattern matching
        let ref y = s;
        assert_eq!(x, "omar");
        assert_eq!(y, "omar");
        assert_eq!(y, x);
        // both hold the same memory address
        assert_eq!(get_address(&x), get_address(&y));
    }

    fn get_address(s: &String) -> String {
        // it returns the address of the string
        format!("{:p}", s)
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

    #[test]
    fn if_test() {
        // if/else expression can be used in assignment
        let number = if true { 5 } else { 6 };
        assert!(number == 5);
    }

    #[test]
    fn for_test() {
        let mut sum = 0;
        for i in 0..5 {
            if i == 1 {
                continue;
            }
            if i == 3 {
                break;
            }
            sum += i;
        }
        assert_eq!(sum, 2);

        let mut sum = 0;
        // from 3 to 2, 2 is not included
        for i in -3..2 {
            sum += i;
        }
        assert_eq!(sum, -5);

        // from 3 to 2, 2 is  included
        let mut sum = 0;
        for i in -3..=2 {
            sum += i;
        }
        assert_eq!(sum, -3);

        let mut sum = 0;
        for i in 'a'..='c' {
            sum += i as i32;
        }
        assert_eq!(sum, 294);

        let names = [String::from("omar"), String::from("barra")];
        assert_eq!(names.len(), 2);
        for name in names {
            assert!(name == "omar" || name == "barra");
        }
        // does not compile, the for tool the ownership of the values in the array
        // assert_eq!(names.len(), 2);

        let names = [String::from("omar"), String::from("barra")];
        // to avoid taking the ownership, we can use iter()
        for name in names.iter() {
            assert!(name == "omar" || name == "barra");
        }
        assert_eq!(names.len(), 2);

        // other way: to avoid taking the ownership, we can use &
        for name in &names {
            assert!(name == "omar" || name == "barra");
        }
        assert_eq!(names.len(), 2);

        let a = [3, 2, 3];

        let mut index: usize = 0;
        let mut sum = 0;
        // enumerate returns a tuple with the index and the value
        for (i, v) in a.iter().enumerate() {
            index = index + i + 1;
            sum = sum + v;
        }
        assert_eq!(index, 6);
        assert_eq!(sum, 8);
    }

    #[test]
    fn while_test() {
        let mut index: u8 = 0;
        while index < 10 {
            index += 1;
        }
        assert_eq!(index, 10);
    }

    #[test]
    fn loop_test() {
        let mut count: u8 = 0;

        // infinite loop, usually used together with break and continue
        loop {
            count += 1;

            if count == 3 || count == 7 {
                continue;
            }

            if count == 10 {
                break;
            }
        }
        assert_eq!(count, 10);

        // loop is an expression, so it can return a value
        let mut count: u8 = 0;
        let r = loop {
            count += 1;
            if count == 10 {
                break count + 1;
            }
        };
        assert_eq!(r, 11);

        // Nesting and labels with loop
        let mut count = 0;
        'outer: loop {
            'inner1: loop {
                if count >= 20 {
                    break 'inner1;
                }
                count += 2;
            }

            count += 5;

            'inner2: loop {
                if count >= 30 {
                    break 'outer;
                }

                continue 'outer;
            }
        }
        assert_eq!(count, 30);
    }
}
