use std::mem;

#[test]
fn capture_enclosing_environment_type_infered() {
    let x = 1;
    // compiler will caputure vairaibles in the least restrictive manner possible
    // a mutable reference of x is taken, rather than taking the ownership of x because it is less restrictive
    let closure = |val| val + x;
    assert_eq!(closure(2), 3);

    let closure_annotated = |x: i32| -> i32 { x + 1 };
    assert_eq!(closure_annotated(2), 3);

    // type is inferred
    let closure_infered = |x| x + 1;
    assert_eq!(closure_infered(2), 3);

    let closure_one = || 1;
    assert_eq!(closure_one(), 1);
}

#[test]
fn type_infered() {
    let closure = |val| val;
    assert_eq!(closure(2), 2);
    // does not compile, error mismatched types
    // compiler infers the type of the closure, but it is unique the first time the closure is called
    //    assert_eq!(closure("omar"), "omar");
}

#[test]
fn closure_with_inmmutable_borrowing() {
    // in this case, count is borrowed as inmmutable many times. That is allowed
    let count = 0;
    let inc = || count + 1;
    // assert holds an inmmutable reference of count
    assert_eq!(count, 0);

    // reborrow holds an inmmutable reference of count
    let reborrow = &count;
    assert_eq!(*reborrow, 0);

    // inc holds an inmmutable reference of count
    let v = inc();
    assert_eq!(v, 1);

    let v = inc();
    assert_eq!(v, 1);

    let reborrow = &count;
    assert_eq!(*reborrow, 0);
}

#[test]
fn closure_with_mutable_borrowing() {
    let mut count = 0;
    let mut inc = || {
        count += 1;
        count
    };
    // these lines do not compile, because count is borrowed as mutable and immutable at the same time breaking the rules of borrowing
    // let reborrow = &count;
    // assert_eq!(count, 1);

    let v = inc();
    assert_eq!(v, 1);
    let v = inc();
    assert_eq!(v, 2);
    // this works because count is not borrowed at this point
    assert_eq!(count, 2);
}

#[test]
fn closure_with_ownership_and_inmmutable() {
    let color = String::from("green");
    let print_with_move = move || format!("Color is {}", color);
    let printted = print_with_move();
    assert_eq!(printted, "Color is green");

    // TODO why this works? I guess it has to break since the closure takes the ownership of color
    let printted = print_with_move();
    assert_eq!(printted, "Color is green");
    // it doe not compile, error borrow of moved value
    //  let reborrow = &color;

    // using int instead of String
    let color = 4;
    let print_with_move = move || format!("Color is {}", color);
    let printted = print_with_move();
    assert_eq!(printted, "Color is 4");
    let printted = print_with_move();
    assert_eq!(printted, "Color is 4");
    // TODO why this works? I guess it has to break since is the same case as above but using int
    let reborrow = &color;
    assert_eq!(*reborrow, 4);
}

#[test]
fn closure_with_ownership_and_mutable() {
    let mut count = 0;
    // move keyword is used to take the ownership of count
    let mut inc = move || {
        count += 1;
        count
    };
    // it works since count is borrowed as inmmutable, and the closure takes the ownership of count
    let reborrow = &count;
    assert_eq!(*reborrow, 0);
    assert_eq!(count, 0);

    let v = inc();
    assert_eq!(v, 1);
    assert_eq!(*reborrow, 0);
    // TODO why in thi case count is not modified? how can modify the value of count using move?
    assert_eq!(count, 0);

    // TODO why this works? I guess it has to break since the closure takes the ownership of color 2 times
    // TODO why closure returns 2? does it takes it own copy of count?
    let v = inc();
    assert_eq!(v, 2);
    assert_eq!(*reborrow, 0);
    assert_eq!(count, 0);

    // the closure no longer needs to borrow '&mut count'. Therfore, it is possible to reborrow without error
    let count_reborrowed = &mut count;
    assert_eq!(*count_reborrowed, 0);
    assert_eq!(count, 0);
}

#[test]
fn closure_take_ownership_vs_borrowing() {
    fn take_ownership<T>(_: T) {}
    let movable = Box::new(4);
    let consume = || {
        assert_eq!(*movable, 4);
        take_ownership(movable);
    };
    consume();
    // does not compile, error use of moved value
    // closure can be called only once because it takes the ownership of movable
    // consume();

    fn take_borrowing<T>(_: &T) {}
    let movable = Box::new(4);
    let consume = || {
        assert_eq!(*movable, 4);
        take_borrowing(&movable);
    };
    // any number of calls work since the closure does not take the ownership of movable
    consume();
    consume();
}

#[test]
fn closure_take_inmmutable_reference() {
    let movable = Box::new(4);
    // this does not take the ownership of movable and does not need a mutable reference
    // TODO why move is needed here?
    let consume = move || {
        assert_eq!(*movable, 4);
    };
    consume();
    consume();

    let movable = Box::new(4);
    // this does not take the ownership of movable and does not need a mutable reference
    let consume = || {
        assert_eq!(*movable, 4);
    };
    consume();
    consume();
}

#[test]
fn trait_bound_fn_trait() {
    // the closure uses the captured value by reference (&t)
    let captured_value = vec![1, 2, 3];
    // TODO can the type of function be defined?
    let my_fn = |x| x == captured_value.len();
    // the closures can be called any number of times since it my_fn is a fn trait and it captures the value by reference
    assert_eq!(my_fn(3), true);
    assert_eq!(my_fn(4), false);

    // this is just a function that calls the closure
    fn execute_fn<F>(func: F)
    where
        F: Fn(usize) -> bool,
    {
        assert_eq!(func(3), true);
        assert_eq!(func(4), false);
    }
    execute_fn(my_fn);
    // can bu used any number of times
    execute_fn(my_fn);
}

#[test]
fn trait_bound_fnmut_trait() {
    // TODO improve the case
    let mut s = String::from("hello");
    let update_string = |str| s.push_str(str);
    // TODO why does not compile?
    //update_string();

    // TODO why I need the lifetime annotation?
    fn execute_fnmut<'a, F>(mut func: F)
    where
        F: FnMut(&'a str),
    {
        func("omar");
        func("barra");
    }
    execute_fnmut(update_string);

    // does not compile, error use of moved value
    // TODO why?
    // execute_fnmut(update_string);

    let mut s = String::from("hello");
    let update_string = |str| s.push_str(str);
    fn execute_fnmut_otherway<'a, F: FnMut(&'a str)>(mut func: F) {
        func("omar");
        func("barra");
    }
    execute_fnmut_otherway(update_string);
}

#[test]
fn trait_bound_fn_traitss() {
    let mut s = String::from("hello");
    let update_string = || s.push_str(" world");
    exec(update_string);
    println!("{}", s);
}

fn exec<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

#[test]
fn trait_bound_trying_to_use_the_least_restrive_manner_possible() {
    fn execute_function<F>(func: F)
    where
        // F cannot be Fn or FnMut since it is not possible to call the closure more than once
        F: FnOnce(),
    {
        func();
    }

    let greeting = "hello";
    let mut farewall = "java".to_owned();
    // diary closure capures by FnOnce since the least restrictive manner possible cannot be applied, so it takes the more generic way: FnOnce
    // the traits implemented by the closure type are determined by what the closure does with the captured values, not how it captures them.
    let diary = || {
        // greeting is captured by inmmutable reference: requires Fn
        assert_eq!(greeting, "hello");

        // mutation foreces firwall to be captured by mutable reference: requires FnMut
        farewall.push_str("omar");
        assert_eq!(farewall, "javaomar");

        // Manually calling drop forces farewall to be captured by value: requires FnOnce
        mem::drop(farewall);
    };

    execute_function(diary);
}

#[test]
fn move_vs_fntraits() {
    fn execute_fnonce<F>(func: F)
    where
        F: FnOnce(),
    {
        func();
    }

    fn execute_fn<F>(func: F)
    where
        F: Fn(),
    {
        func();
    }
    // move closures may still implement Fn or FnMut, even though they capture variables by move

    // difference between move and fn traits
    // fn traits defines what the closuere does with the captured values
    // move defines how it captures the values, the move keyword only especifies the latter

    let s = String::from("omar");
    let assert_string = move || {
        assert_eq!(s, "omar");
    };
    // assert_string can be executed by FnOnce and Fn because moves does not define the traits
    execute_fnonce(assert_string);

    let s = String::from("omar");
    let assert_string = move || {
        assert_eq!(s, "omar");
    };
    // assert_string can be executed by FnOnce and Fn  because moves does not define the traits
    execute_fn(assert_string);
    // TODO why it does not compile?
    // execute_fn(assert_string);
}

#[test]
fn trait_bound_fnonce() {
    let mut s = String::from("java");
    let update_string = |str| -> String {
        s.push_str(str);
        // since it returns s, it has to be captured by value and cannot be FnMut
        s
    };

    fn execute_fnonce<'a, F: FnOnce(&'a str) -> String>(func: F) {
        let s = func("omar");
        assert_eq!(s, "javaomar");
    }

    execute_fnonce(update_string);
}

#[test]
fn input_function() {
    fn execute<F: Fn(&str)>(func: F) {
        func("omar");
    }

    // simple functions and closures can be passed as arguments
    fn function(str: &str) {
        assert_eq!(str, "omar");
    }
    execute(function);

    let closue = |str: &str| {
        assert_eq!(str, "omar");
    };
    execute(closue);
}

#[test]
fn closure_as_return_using_static_dispatch() {
    // static dispatch
    fn create_fn() -> impl Fn(i32) -> i32 {
        let num = 4;
        // num get moved into the closure, the closure will be come the owner of the variable
        move |x| x + num
    }
    let closure = create_fn();
    assert_eq!(closure(2), 6);
}

#[test]
fn closure_as_return_using_dynamic_dispatch() {
    // dynamic dispatch
    fn create_fn() -> Box<dyn Fn(i32) -> i32> {
        let num = 4;
        // num get moved into the closure, the closure will be come the owner of the variable
        Box::new(move |x| x + num)
    }
    let closure = create_fn();
    assert_eq!(closure(2), 6);
}

#[test]
fn closure_as_return_using_dynamic_dispatch_mandatory() {
    // dynamic dispatch in this case is mandatory since the closures have different memory locations even if they have the same code
    fn create_fn() -> Box<dyn Fn(i32) -> i32> {
        let num = 4;
        if num > 0 {
            Box::new(move |x| x + num)
        } else {
            Box::new(move |x| x + num)
        }
    }
    let closure = create_fn();
    assert_eq!(closure(2), 6);
}
