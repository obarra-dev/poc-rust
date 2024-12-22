use std::fmt::Debug;

use some_tests::type_of;

#[test]
fn static_lifetime_for_str() {
    let s = "omar";
    let ss: &'static str = "omar";
    assert_eq!(s, ss);
    assert_eq!(type_of(&s), type_of(&ss));
}

#[test]
fn static_lifetime_for_integer() {
    // static is similar to const, but static remains in the memory location for the entire duration of the program
    static NUM: i32 = 4;
    assert_eq!(NUM, 4);
    assert_eq!(type_of(&NUM), "i32");
}

#[test]
fn static_lifetime_is_only_for_values() {
    let s: &'static str = "omar";

    {
        let ss: &'static str = "barra";
        assert_eq!(ss, "barra");
    }

    // does not compile, ss has the reference to a static string, the value can live forever but not the reference
    // assert_eq!(ss, "barra");

    assert_eq!(s, "omar");
}

#[test]
fn static_lifetime_coerce() {
    let s: &'static str = "omar";
    // r lifetime is shorter than s
    fn coerce_static<'a>(r: &'a str) -> &'a str {
        r
    }

    let r = coerce_static(s);
    assert_eq!(r, "omar");
}

#[test]
fn static_lifetime_coerce_integer() {
    static NUM: i32 = 4;
    let r: &'static i32 = &NUM;
    assert_eq!(*r, 4);
}

#[test]
//  TODO is trait bound a correct name?
fn static_lifetime_trait_bound() {
    static NUM: i32 = 4;
    // the type canonot contain any non-static references
    fn print_it<T: Debug + 'static>(input: T) {
        println!("static value passed in is: {:?}", input);
    }
    print_it(NUM);
    print_it("omar");

    fn print_it_2<T: Debug + 'static>(input: &T) {
        println!("static value passed in is: {:?}", input);
    }
    print_it_2(&NUM);
    print_it_2(&"omar");

    // TODO find a way to build  a string and return it
}
