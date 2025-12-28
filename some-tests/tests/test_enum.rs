use some_tests::type_of;

#[test]
fn enum_test() {
    enum IpAddrKind {
        V4(String),
        V6(String),
    };

    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    assert_eq!(type_of(&home), "test_enum::enum_test::IpAddrKind");

    let loopback = IpAddrKind::V6(String::from("::1"));
    assert_eq!(type_of(&loopback), "test_enum::enum_test::IpAddrKind");

    // TODO how to get the internal value?
}

#[test]
fn enum_discriminant_test() {
    // If a field-less enum only contains unit variants, the enum is called an unit-only enum
    // by default, the first variant is 0, the second is 1, and so on
    #[derive(PartialEq, Debug)]
    enum Number {
        Zero,
        One,
        Two,
    };

    let number = Number::One;
    assert_eq!(number, Number::One);
    assert_eq!(number as u8, 1);

    // Zero is 10, One is 11, Two is 12
    #[derive(PartialEq, Debug)]
    enum NumberOdd {
        Zero = 10,
        One,
        Two,
    };
    let number = NumberOdd::Two;
    assert_eq!(number, NumberOdd::Two);
    assert_eq!(number as u8, 12);
}

#[test]
fn enum_several_variant_test() {
    // each enum variant can optionally hold its own data
    // enum is called a ‘sum type’: the set of possible values of the enum is the sum of the sets of possible values for each variant.
    #[derive(PartialEq, Debug)]
    enum Message {
        // variant without any data
        Quit,
        // variant with named data (like struct)
        Move { x: i32, y: i32 },
        Write(String),
        // variant with unnamed data (like tuple structs)
        ChangeColor(i32, i32, i32),
    }

    let m = Message::Write(String::from("omar"));
    // question: how to extract the value for no primitive types?
    // assert_eq!(m as String, "omar");
    assert_eq!(m, Message::Write(String::from("omar")));
    let s = format!("{:?}", m);
    assert_eq!(s, "Write(\"omar\")");

    let m = Message::Quit;
    assert_eq!(m, Message::Quit);
    let s = format!("{:?}", m);
    assert_eq!(s, "Quit");

    let m = Message::Move { x: 1, y: 2 };
    let s = format!("{:?}", m);
    assert_eq!(s, "Move { x: 1, y: 2 }");

    let m = Message::ChangeColor(1, 2, 3);
    assert_eq!(m, Message::ChangeColor(1, 2, 3));
    let s = format!("{:?}", m);
    assert_eq!(s, "ChangeColor(1, 2, 3)");

    let m = Message::Move { x: 1, y: 2 };
    // pattern matching and destructuring
    if let Message::Move { x, y } = m {
        assert_eq!(x, 1);
        assert_eq!(y, 2);
    } else {
        panic!("NEVER LET THIS RUN");
    }

    let messages = [
        Message::Quit,
        Message::Move { x: 1, y: 2 },
        Message::Write(String::from("omar")),
    ];
    for m in messages {
        match m {
            Message::Quit => assert_eq!(format!("{:?}", m), "Quit"),
            Message::Move { x, y } => {
                assert_eq!(format!("{:?}", m), "Move { x: 1, y: 2 }");
                assert_eq!(x, 1);
                assert_eq!(y, 2);
            }
            Message::Write(s) => {
                // question: why partionally moved?
                // assert_eq!(format!("{:?}", m), "Write(\"omar\")");
                assert_eq!(s, "omar");
            }
            _ => panic!("NEVER LET THIS RUN"),
        }
    }
}

#[test]
fn enum_constructor_like_function_test() {
    #[derive(PartialEq, Debug)]
    enum Message {
        Write(String),
    }

    let v = vec!["Hello".to_string(), "World".to_string()];

    // enum constructor can also be used like a function
    let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();
    assert_eq!(
        v1,
        vec![
            Message::Write("Hello".to_string()),
            Message::Write("World".to_string())
        ]
    );

    fn foo(x: String) -> Message {
        Message::Write(x)
    }

    let x = foo("Hello, world".to_string());
    assert_eq!(x, Message::Write("Hello, world".to_string()));
}
