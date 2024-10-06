pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::i32;

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

        // rust allows to overrides variables
        let z = 4;
        assert_eq!(z, 4);
        let z = 20;
        assert_eq!(z, 20);

        // rust allows to overrides variables changing the type
        let a = 4;
        assert_eq!(a, 4);
        let a = "omar";
        assert_eq!(a, "omar");
    }

    #[test]
    fn virables_shadowing() {
        let x = 4;
        assert_eq!(x, 4);

        {
            // interior scope, this x valuse only lives here
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
}
