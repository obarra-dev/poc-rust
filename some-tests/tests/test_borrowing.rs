use some_tests::{get_address, type_of};

#[test]
fn immutable_borrowed() {
    let count = 0;
    // count can be borrowed as immutable many times
    let borrow = &count;
    let reborrow = &count;

    assert_eq!(*borrow, 0);
    assert_eq!(*reborrow, 0);
    assert_eq!(count, 0);
    assert_eq!(type_of(&count), "i32");
    assert_eq!(type_of(&borrow), "&i32");
    assert_eq!(type_of(&reborrow), "&i32");
}

#[test]
fn mutable_borrowed() {
    let mut count = 0;
    let mutable_borrow = &mut count;
    // it does not compile since count can be borrowed as mutable only once
    // let remutable_borrow = &mut count;

    // it does not compile, count can not be borrowed as mutable and then as immutable at the same time
    //let reborrow = &count;
    assert_eq!(*mutable_borrow, 0);
    assert_eq!(type_of(&mutable_borrow), "&mut i32");

    // since it is a mutable reference
    *mutable_borrow = 4;
    assert_eq!(*mutable_borrow, 4);
    assert_eq!(count, 4);
}

#[test]
fn borrowing() {
    // s is an owned string, it is allocated on the heap
    let s = String::from("omar");
    // & is to pass the immutable reference
    // ownership is not transferred, it is borrowed
    // I can have any number of immutable references from the same variable
    // x is a reference to s, it means x is a pointer to s
    // in other words, references and borrowing refer to the same concept:
    // a mechanism that allows you to use a value without taking ownership of it.
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
    // s is a mutable variable of type String
    let mut s = String::from("omar");
    // x is a mutable reference to s
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

    // this will cause the mutatable reference z does not compile, as x and y were used before
    // assert_eq!(x, "omar");
    // assert_eq!(y, "omar");
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
    // TODO why &String?
    let x = &s;
    assert_eq!(x, "omar");
    assert_eq!(type_of(&x), "&alloc::string::String");

    // ref is similar to &, but there are some minor differences in pattern matching
    let ref y = s;
    assert_eq!(y, "omar");
    assert_eq!(y, x);
    assert_eq!(type_of(&y), "&alloc::string::String");

    // both hold the same memory address
    // TODO if I use generic in get_address this assert fails why?
    assert_eq!(get_address(&x), get_address(&y));
}

#[test]
fn reference_using_as_ref() {
    let s = String::from("omar");
    // & does that via deref coercions. That works only for things that implement the Deref trait.
    // TODO  why does it need explicit type &str?
    let x:&str = s.as_ref();

    assert_eq!(x, "omar");
    assert_eq!(type_of(&x), "&str");
}