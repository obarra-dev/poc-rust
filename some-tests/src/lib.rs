pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::{fmt::format, i32, ops::RangeInclusive};

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
        let tuple = ('o', true, 4);
        assert_eq!(tuple.2, 4);
        assert_eq!(tuple.1, true);
        assert_eq!(tuple.0, 'o');

        let mut tuple = ('o', false, 5);
        tuple.0 = 'd';
        assert_eq!(tuple.2, 5);
        assert_eq!(tuple.1, false);
        assert_eq!(tuple.0, 'd');

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

    // TODO to check
    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>()) //
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
    fn string_test() {
        let a = String::from("omar");
        assert_eq!(a, "omar");
    }

    #[test]
    fn for_test() {
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
    fn borrowing() {
        let s = String::from("omar");
        // x and z are read only
        let x = &s;
        let z = &s;

        assert_eq!(s, "omar");
        assert_eq!(x, "omar");
        assert_eq!(z, "omar");

        let mut s = String::from("omar");
        {
            //   let x: = &mut s;
            // x.push_str("barra");
            //x.clear();
        }
        // x and z are read only
    }
}
