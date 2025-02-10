
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
