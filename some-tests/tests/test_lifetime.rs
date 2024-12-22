#[test]
fn danglin_reference() {
    // lifetime of r is valid until the end of the test
    let r;
    {
        // life time of x is valid until the end of this scope
        let x = 1;
        r = &x;
    }

    // this does not compile, error `x` does not live long enough
    // since r points to something that is not valid anymore, dangling pointer/references
    // println!("r: {}", r);
}

#[test]
fn values_lives_on_its_scope() {
    // v has the longest lifetime because its scope enterily encloses both borrow and borrow2
    let v = 4;
    {
        let borrow_r = &v;
        // TODO why it does not need to dereference
        println!("borrow_r: {}", borrow_r);
        // TODO a better way to do it?
        assert_eq!(*borrow_r, 4);
    }

    {
        let borrow_r2 = &v;
        assert_eq!(*borrow_r2, 4);
    }
}

#[test]
fn lifetime_annotation() {
    // one input reference with lifetime wich must live at least as long as the function
    fn checker<'a>(x: &'a i32) {
        assert_eq!(*x, 4);
    }
    let v: i32 = 4;
    checker(&v);

    // mutable reference with lifetime
    fn add_one<'a>(x: &'a mut i32) {
        *x += 1;
    }

    let mut v: i32 = 4;
    add_one(&mut v);
    assert_eq!(v, 5);

    // mutliple elements with different lifetiems
    // in this case, it would be fine for both the same lifetime but in more complex cases, differect lifetime may be required
    // returning refernces that have been passed in is acceptable but the correct lifetime  must be returned
    fn pass_x<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
        x
    }

    let v = 4;
    let v2 = 5;
    let r = pass_x(&v, &v2);
    assert_eq!(*r, 4);
}

#[test]
fn lifetime_annotation_is_necessary() {
    // in this case, the lifetime is necessary because the compiler does not know the lifetime of the return value
    // in this case the return value must live longer than the function
    // if the lifetime is not specified, the compiler will throw an error, missing lifetime specifier
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let s = "omar";
    let s2 = "barra";
    let r = longest(s, s2);
    assert_eq!(r, "barra");
}

#[test]
fn lifetime_annotation_bad_use() {
    // cannot return reference to temporary value
    /*
        fn invalid_output<'a>() -> &'a str {
          &String::from("omar")
    }
     */

    // alternative way to do it
    fn valid_output_return_ownership() -> String {
        String::from("omar")
    }
    let s = valid_output_return_ownership();
    assert_eq!(s, "omar");

    fn valid_output_return_string_literal() -> &'static str {
        "omar"
    }
    let s = valid_output_return_string_literal();
    assert_eq!(s, "omar");
}

#[test]
fn lifetime_annotation_bad_use_1() {
    fn invalid_doesnot_live_enough<'a>() {
        let x = 4;
        // does not compile, error `x` does not live long enough
        // let y: &'a i32 = &x;
    }
}

#[test]
fn lifetime_annotation_in_enum() {
    #[derive(Debug)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }
    let x = 4;
    let r = Either::Ref(&x);
    let s = format!("{:?}", r);
    assert_eq!(s, "Ref(4)");
    if let Either::Ref(y) = r {
        assert_eq!(*y, 4);
    }

    let r = Either::Num(x);
    let s = format!("{:?}", r);
    assert_eq!(s, "Num(4)");
    if let Either::Num(y) = r {
        assert_eq!(y, 4);
    }
}

#[test]
fn lifetime_annotation_in_struct() {
    struct Borrowed<'a>(&'a i32);
    let x = 4;
    let b = Borrowed(&x);
    assert_eq!(*b.0, 4);

    struct Foo<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    let x = 4;
    let y = 5;

    let f = Foo { x: &x, y: &y };
    assert_eq!(*f.x, 4);
    assert_eq!(*f.y, 5);
}

struct Borrowed<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

#[test]
fn lifetime_annotation_in_struct_bad_use() {
    let x = 4;
    let borrowed: Borrowed;
    {
        let y = 5;
        borrowed = Borrowed { x: &x, y: &y };
        // this works because the lifetime of y is valid until the end of this scope
        assert_eq!(*borrowed.x, 4);
        assert_eq!(*borrowed.y, 5);
    }

    // does not compile, error `y` does not live long enough
    // assert_eq!(*borrowed.x, 4);
}

#[test]
fn lifetime_annotation_in_fn_same_annotation_dif_types() {
    fn my_fun<'a>(b: &'a Borrowed) -> &'a i32 {
        &b.y
    }

    let x = 4;
    let y = 5;
    let borrowed = Borrowed { x: &x, y: &y };
    let r = my_fun(&borrowed);
    assert_eq!(*r, 5);
}

#[test]
fn lifetime_annotation_in_methods() {
    struct Owner(i32);

    impl Owner {
        fn add_one<'a>(&'a mut self) {
            self.0 += 1;
        }

        fn get_value<'a>(&'a self) -> &'a i32 {
            &self.0
        }
    }

    let mut o = Owner(4);
    o.add_one();
    assert_eq!(*o.get_value(), 5);

    // TODO: why do I have to use reference in methods if the struct does not hold a reference?
    // it seems that to execute add_one and then get_values, I need to use lifetime annotations but I do not fully understand
    // I tried to use it without reference but it does not work I got 'use of moved value'
}

#[test]
fn lifetime_annotation_in_methodsss() {
    struct Foo<'a> {
        part: &'a str,
    }

    // when the struct uses lifetime annotation, the implementation must also use it but you can put '_
    impl Foo<'_> {
        fn get_leng<'a>(&'a self) -> usize {
            self.part.len()
        }
    }

    let o = Foo { part: "omar" };
    let l = o.get_leng();
    assert_eq!(l, 4);
}
