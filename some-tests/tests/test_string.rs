use some_tests::type_of;

#[test]
fn string_test() {
    let a = String::from("omar");
    assert_eq!(a, "omar");
}

#[test]
fn string() {
    // string literary, type &str
    // string literary is a hardcode in the binary itself
    // so the size is known at compile time
    // string literay is also considering a string slice because it is mutable and it a reference to a static memory
    let s = "omar";
    assert_eq!(s, "omar");
    let pointer = format!("{:p}", s);
    // it retuns the address of the string
    // question, how to check it?
    // assert_eq!(pointer, "0x7ff64179275");

    // type String
    let s = String::from("omar barra");
    assert_eq!(s, "omar barra");

    // string slice
    // points to a squance of characters stored on the heap
    let x = &s[0..4];
    let y = &s[5..10];
    assert_eq!(x, "omar");
    assert_eq!(y, "barra");

    // we can use str only by boxed it, it has to be explicitly typed
    // omar is converted into a Box<str>, heap allocated
    let s: Box<str> = "omar".into();
    // & can be used to convert Box<str> to &str, it has to be explicitly typed
    let x: &str = &s;
    assert_eq!(x, "omar");
}

#[test]
fn string_methods() {
    let mut s = String::from("omar");
    s.push_str(" barra");
    s.push('!');
    // this way is valid
    s += "!";
    assert_eq!(s, "omar barra!!");

    let s = String::from("omar java");
    // it allocate new memory and store the modified string there, it does not modify the original one
    let x = s.replace("java", "barra");
    assert_eq!(s, "omar java");
    assert_eq!(x, "omar barra");

    let s = String::from("omar");
    let x = String::from("barra");
    // as_str() convert String to string slice (&str)
    let y = s + x.as_str();
    assert_eq!(y, "omarbarra");

    let s = String::from("omar");
    // convert String to &str
    let x = s.as_str();
    assert_eq!(type_of(&x), "&str");
    assert_eq!(type_of(&s), "alloc::string::String");
    // question: I have to define the type explicitly, why?
    let y: &str = &s;
    assert_eq!(type_of(&x), type_of(&y));
    // question: if it is implicitly defined it is &String why?
    let z = &s;
    assert_eq!(type_of(&z), "&alloc::string::String");

    let s = "omar";
    // convert &str to String
    let x = s.to_string();
    assert_eq!(type_of(&x), "alloc::string::String");
    assert_eq!(type_of(&s), "&str");
    let y = String::from(s);
    assert_eq!(type_of(&x), type_of(&y));
    let z = s.to_owned();
    assert_eq!(type_of(&x), type_of(&z));

    // concat
    let s = String::from("omar");
    let x = String::from("barra");
    // concat a String with &str, we cannot concat a String with a String
    // & convert String to string slice (&str)
    let y = s + &x;
    assert_eq!(y, "omarbarra");
    // does not compile, as the ownership of s was moved
    // assert_eq!(s, "omar");

    // \ is for scape characters
    let s = "this is Ru\x73\x74";
    assert_eq!(s, "this is Rust");

    // lenght
    let s = String::from("omar");
    assert_eq!(s.len(), 4);
    let s = String::from("读写汉字 - 学中文");
    // lenght is not the number of characters, it is the number of bytes
    assert_eq!(s.len(), 24);

    // s[0] is not allowed, it is not a valid index
    // you have to use string slice &s[from..to]
    let s = String::from("omar 中文");
    let o = &s[0..1];
    assert_eq!(o, "o");
    // 3 bytes for 中
    let chinise_letter = &s[5..8];
    assert_eq!(chinise_letter, "中");

    // operations on UTF8
    let iter_chars = "学中文".chars();
    let mut s = String::new();
    for c in iter_chars {
        s.push(c);
        s.push(c);
    }
    assert_eq!(s, "学学中中文文");
}
