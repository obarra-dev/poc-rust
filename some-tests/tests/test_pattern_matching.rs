#[test]
fn match_test() {
    enum Direction {
        East,
        West,
        North,
        South,
    }

    let d = Direction::South;
    // match is an expression, so it can return a value
    let message = match d {
        Direction::East => "East",
        // | to match several values
        Direction::West | Direction::North | Direction::South => "My direction",
        _ => "Unknown",
    };
    assert!(message == "My direction");

    let c = 'O';
    let message = match c {
        // .. to match range
        // ..= to match range inclusive
        'a'..='z' => "lowercase",
        'A'..='Z' => "uppercase",
        _ => "neither",
    };
    assert_eq!(message, "uppercase");
}

#[test]
fn matches_test() {
    // matches
    let alphabets = ['a', 'b', 'O', 'M', '9'];
    for &c in alphabets.iter() {
        // matches is a macro, and do more than match
        assert!(matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'));
    }

    enum MyEnum {
        Foo,
        Bar,
        Baz,
    }

    let mut count = 0;
    let arr = [MyEnum::Foo, MyEnum::Bar, MyEnum::Baz, MyEnum::Foo];
    for e in arr {
        if matches!(e, MyEnum::Foo) {
            count += 1;
        }
    }
    assert_eq!(count, 2);
}

#[test]
fn if_let_test() {
    // match could be verbose, so if let is a more concise way to handle it
    let some_number = Some(5);
    let v = match some_number {
        Some(i) => i,
        None => panic!("NEVER LET THIS RUN"),
    };
    assert_eq!(v, 5);

    // using if let for this simple cases
    // if let allows to match only one pattern and to unwrap the value
    let some_number = Some(5);
    let v = if let Some(i) = some_number {
        i
    } else {
        panic!("NEVER LET THIS RUN");
    };
    assert_eq!(v, 5);
}

#[test]
fn pattern() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 7, y: 6 };
    match p {
        Point { x: 0, y } => assert_eq!(y, 7),
        Point { x, y: 0 } => assert_eq!(x, 0),
        // does not compile, y is not defined in this scope for some reason
        // Point { x, y: 0 } => assert_eq!(y, 0),

        // @ operator lets us create a variable that holds a value, at the same time as testing that value
        // @ operator destructures the value and matches it with the pattern
        Point {
            x: 2..=5,
            y: y @ 2..=5,
        } => assert!(y >= 2 && y <= 5),
        Point {
            x: x @ 6..=7,
            y: y @ 6..=7,
        } => assert!(x >= 6 && x <= 7 && y >= 6 && y <= 7),

        // otherwise on neither axis
        Point { x, y } => {
            assert_eq!(x, 0);
            assert_eq!(y, 7);
        }
    }

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => assert!(id_variable >= 3 && id_variable <= 7),
        Message::Hello { id: id @ (10 | 11) } => assert!(id == 10 || id == 11),
        Message::Hello { id } => assert_eq!(id, 5),
    }

    // match guard is an additional if condition
    // todo improve the example
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) if x == split => assert!(x == split),
        Some(x) if x > split => assert!(x > split),
        Some(x) => panic!("NEVER LET THIS RUN"),
        None => panic!("NEVER LET THIS RUN"),
        _ => panic!("NEVER LET THIS RUN"),
    }

    let numbers = (2, 4, 6, 8, 10);
    match numbers {
        // destructuring a tuple to get the first and last values
        (first, .., last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 10);
        }
    }

    let mut v = String::from("omar");
    let r = &mut v;
    // use pattern &mut V to match a mutable reference needs you to be very careful, due to v being a value after matching
    match r {
        value => value.push_str(" barra"),
    }

    assert_eq!(v, "omar barra");
}
