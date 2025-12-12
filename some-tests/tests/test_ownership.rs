use some_tests::type_of;

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
fn ownership_dereferencing() {
    let v = 4;
    // x is a reference to v
    let x = &v;
    // dereferencing x to get the value of v
    assert_eq!(*x, 4);

    // Box allows to store data on the heap
    let mut v = Box::new(4);
    assert_eq!(*v, 4);

    // dereferencing v to set a new value
    *v = 44;
    assert_eq!(*v, 44);
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
fn new_reference_from_reference() {
    let v = 4;
    // x is a reference to v
    let x = &v;
    let pointer_x = format!("{:p}", x);
    assert_eq!(*x, 4);
    assert_eq!(type_of(x), "i32");
    assert_eq!(type_of(&x), "&i32");


    // &* takes a reference to that value, so you get a new reference to the same data
    // The "new reference" is just a new variable, not a new memory location.
    let y = &*x;
    let pointer_y = format!("{:p}", y);
    println!("{}", pointer_y);
    assert_eq!(*y, 4);
    assert_eq!(type_of(y), "i32");
    assert_eq!(type_of(&y), "&i32");
    assert_eq!(type_of(y), type_of(x));
    assert_eq!(type_of(&y), type_of(&x));
    // y and x are two reference variables, but both point to the same location in memory
    assert_eq!(pointer_y, pointer_x); 

    // a new reference to the same data
    let z = &*y;
    let pointer_z = format!("{:p}", z);
    assert_eq!(pointer_x, pointer_z); 
}
