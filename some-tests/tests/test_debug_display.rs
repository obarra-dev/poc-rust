use std::fmt::Debug;

// this structure cannot be printed, since it does not implement  either with fmt::Debug or fmt::Display
struct UnPrintable(i32);

// to make the structure printable with debug, we can derive the automatic implementation provided by Rust
#[derive(Debug)]
struct DebugPrintable(i32);

#[test]
fn debug() {
    // does not compile, because UnPrintable does not implement Debug
    // let s = format!("{:?}", UnPrintable(4));

    // does not compile, because UnPrintable does not implement Debug
    // println!("{:?}", UnPrintable(4));

    // :? is for Debug
    let s = format!("{:?}", DebugPrintable(4));
    assert_eq!(s, "DebugPrintable(4)");
    println!("{:?}", DebugPrintable(4));
}

#[test]
fn debeug_elegant() {
    let s = format!("{:#?}", DebugPrintable(4));
    assert_eq!(s, "DebugPrintable(\n    4,\n)");

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: "Omar".to_string(),
        age: 30,
    };
    let s = format!("{:#?}", person);
    assert_eq!(s, "Person {\n    name: \"Omar\",\n    age: 30,\n}");

    let s = format!("{:?}", person);
    assert_eq!(s, "Person { name: \"Omar\", age: 30 }");
}

#[test]
fn debeug_manual_implementation() {
    struct Structure(i32);
    // cannot derive Debug, we have to use either #[derive(Debug)] or manual implementation
    struct Deep(Structure);

    impl Debug for Deep {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "the value is: {}", self.0 .0)
        }
    }

    let s = format!("{:?}", Deep(Structure(4)));
    assert_eq!(s, "the value is: 4");
}

#[test]
fn display() {
    // there is no way to derive the implementation of Display trait
    // we have to implement it manually
    // the placeholder for Display is {} not {:?}

    // does not compile, because DebugPrintable does not implement Display
    // println!("{}", DebugPrintable(4));

    struct Person {
        name: String,
        age: u8,
    }

    impl std::fmt::Display for Person {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Person {} is {} years old, using Display",
                self.name, self.age
            )
        }
    }

    let person = Person {
        name: "Omar".to_string(),
        age: 30,
    };

    // does not compile, because Person does not implement Debug, :? is for Debug
    //let s = format!("{:?}", person);

    // {} is for Display
    let s = format!("{}", person);
    assert_eq!(s, "Person Omar is 30 years old, using Display");
}

#[test]
fn displayss() {
    struct List(Vec<i32>);

    impl std::fmt::Display for List {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // using ? operator helps to handle errors
            write!(f, "[")?;
            let vec = &self.0;

            for (i, v) in vec.iter().enumerate() {
                if i == 0 {
                    write!(f, "{}: {}", i, v)?;
                } else {
                    write!(f, ", {}: {}", i, v)?;
                }
            }

            write!(f, "]")
        }
    }

    let list = List(vec![1, 2, 3]);
    let s = format!("{}", list);
    assert_eq!(s, "[0: 1, 1: 2, 2: 3]");
}
