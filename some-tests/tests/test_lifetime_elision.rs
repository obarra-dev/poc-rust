#[test]
fn infer_lifetime_first_second_rule() {
    // the compiler will apply 2 rules in this function
    fn my_fn(x: &i32) -> &i32 {
        x
    }
    let x = 4;
    let r = my_fn(&x);
    assert_eq!(*r, 4);

    // this is the result after compiler applies the first rule
    // each parameter gets its own lifetime
    fn my_fn_first_rule<'a>(x: &'a i32) -> &i32 {
        x
    }
    let x = 4;
    let r = my_fn_first_rule(&x);
    assert_eq!(*r, 4);

    // this is the result after compiler applies the second rule
    // there is exactly one INPUT lifetime, so the lifetime of the input paramter gets assigned to the output lifetime
    fn my_fn_second_rule<'a>(x: &'a i32) -> &'a i32 {
        x
    }
    let x = 4;
    let r = my_fn_second_rule(&x);
    assert_eq!(*r, 4);
}

#[test]
fn infer_third_rule() {
    struct Foo<'a> {
        part: &'a str,
    }

    // TODO I do not understand the third rule

    /*
    impl<'a> Foo<'_> {
        fn build<'a>(&'a self, other_p: &'a str) -> &str {
            if other_p.len() > self.part.len() {
                other_p
            } else {
                self.part
            }
        }
    }

    let o = Foo { part: "omar" };
    let l = o.build("barra");
    assert_eq!(l, "barra");
     */
}

#[test]
fn infer_in_struct() {
    // the compiler cannot infer the lifetime of the reference in the struct
    struct Foo<'a> {
        age: &'a i32,
    }

    let x = 4;
    let foo: Foo<'_> = Foo { age: &x };
    assert_eq!(foo.age, &4);

    // TODO add more examples
}
