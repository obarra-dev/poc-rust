#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: u8,
    active: bool,
}

#[test]
fn struct_test() {
    let user = User {
        name: String::from("omar"),
        email: String::from("obarr@test.com"),
        active: true,
        age: 4,
    };
    assert_eq!(user.name, "omar");
    // does not compile, as the struct is not mutable
    //  user.age = 5;

    // mut is to make the struct mutable
    // Rust does not allow to mark only one field as mutable
    let mut user = User {
        name: String::from("omar"),
        email: String::from("obarr@test.com"),
        active: true,
        age: 4,
    };
    user.email = String::from("omar@test.com");
    assert_eq!(user.email, "omar@test.com");

    // function can instantiate and return a struct
    fn build_user(name: String, email: String) -> User {
        // using the field init shorthand syntax to reduce repetition
        User {
            name,
            email,
            active: true,
            age: 4,
        }
    }
    let user = build_user(String::from("omar"), String::from("obarra@test.com"));
    assert_eq!(user.email, "obarra@test.com");

    // struct update syntax
    let other_user = User {
        email: String::from("test@test.com"),
        ..user
    };
    assert_eq!(other_user.email, "test@test.com");

    //println!("the person struct is: {:?}", p);

    let user = build_user(String::from("omar"), String::from("obarra@test.com"));
    // to print or format a struct, it has to implement the Debug trait
    let s = format!("{:?}", user);
    assert_eq!(
        s,
        "User { name: \"omar\", email: \"obarra@test.com\", age: 4, active: true }"
    );

    // Print debug information to stderr
    dbg!(user);

    // partial move
    let user = build_user(String::from("omar"), String::from("obarra@test.com"));
    // name is moved out of user, so you can not use user.name and user, but you can use user.email
    let name = user.name;
    assert_eq!(name, "omar");
    assert_eq!(user.email, "obarra@test.com");
    // does not compile, as user is partial moved
    // assert_eq!(user.name, "omar");
    // print!("{:?}", user);
}

#[test]
fn tuple_struct() {
    struct Color(u8, u8, u8);
    let color = Color(255, 0, 0);
    assert_eq!(color.0, 255);
    assert_eq!(color.1, 0);
    assert_eq!(color.2, 0);
}

#[test]
fn unit_like_structs() {
    struct UnitLikeStruct;
    let _unit_like_struct = UnitLikeStruct;
    // TODO
}
