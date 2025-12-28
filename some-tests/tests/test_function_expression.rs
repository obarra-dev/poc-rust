use some_tests::get_address_i32;

#[test]
fn expression_test() {
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
}

#[test]
fn function_test() {
    fn my_function(x: i32) -> i32 {
        if x > 0 {
            return x + 10;
        }

        if x < 0 {
            return x - 10;
        }

        0
    }

    let b: i32 = my_function(4);
    assert_eq!(b, 14)
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
