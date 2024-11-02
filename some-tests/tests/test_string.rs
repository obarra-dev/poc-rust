use std::vec;

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
    // allocate memory on the heap
    // String is just a Vec<u8> with some methods
    let mut s = String::from("omar");
    // push_str() takes a string slice
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

    // \ is for scape characters
    let s = "this is Ru\x73\x74";
    assert_eq!(s, "this is Rust");
}

#[test]
fn string_move_owneship() {
    let s = String::from("omar");
    let mut new_s = s;
    new_s.push_str(" barra");
    assert_eq!(new_s, "omar barra");

    // does not compile, as the ownership of s was moved
    // assert_eq!(s, "omar");
}

#[test]
fn string_clone_to_not_move_owneship() {
    fn move_owneship(s: String) -> String {
        let mut new_s = String::from("barra");
        new_s.push_str(&s);
        new_s
    }

    let s = String::from("omar");
    // we can use clone to avoid moving the ownership
    let x = move_owneship(s.clone());
    assert_eq!(x, "barraomar");
    assert_eq!(s, "omar");
}

#[test]
fn string_to_string_slice() {
    let s = String::from("omar");
    assert_eq!(type_of(&s), "alloc::string::String");

    // first way to convert String to string slice (&str), using &s[..]
    let x = &s[..];
    assert_eq!(x, "omar");
    assert_eq!(type_of(&x), "&str");

    // second way to convert String to string slice (&str), using as_str()
    let x = s.as_str();
    assert_eq!(x, "omar");
    assert_eq!(type_of(&x), "&str");

    // third way to convert String to string slice (&str), using &s
    // TODO question: I have to define the type explicitly, why?
    let y: &str = &s;
    assert_eq!(type_of(&x), type_of(&y));
    // TODO question: if it is implicitly defined it is &String why?
    let z = &s;
    assert_eq!(type_of(&z), "&alloc::string::String");

    let x = &s[..2];
    assert_eq!(x, "om");
}

#[test]
fn slice_string_to_string() {
    let s = "omar";
    assert_eq!(type_of(&s), "&str");

    // convert &str to String
    // to_string() allocates memory on the heap
    let x = s.to_string();
    assert_eq!(type_of(&x), "alloc::string::String");

    let y = String::from(s);
    assert_eq!(type_of(&x), type_of(&y));

    let z = s.to_owned();
    assert_eq!(type_of(&x), type_of(&z));
}

#[test]
fn concat() {
    // TODO why it works?
    // concat
    let s = String::from("omar");
    let x = String::from("barra");
    // concat a String with &str, we cannot concat a String with a String
    // & convert String to string slice (&str)
    let y = s + &x;
    assert_eq!(y, "omarbarra");
    // does not compile, as the ownership of s was moved
    // assert_eq!(s, "omar");
}

#[test]
fn string_utf8() {
    // TODO is a native funtio?
    // let s = "hello, 中文 ";
    // let ds = utf8_slice::slice(s, 7, 8);

    let v = vec![111, 109, 97, 114];
    let s = String::from_utf8(v).unwrap();
    assert_eq!(s, "omar");
}

#[test]
fn capacity_and_len() {
    let mut s = String::new();
    assert_eq!(s.capacity(), 0);
    assert_eq!(s.len(), 0);

    // by default the capacity has its own formula
    for i in 0..2 {
        s.push_str("omar");
        assert_eq!(s.len(), 4 * (i + 1));
        assert!(s.capacity() == 8 || s.capacity() == 16);
    }

    let mut s = String::with_capacity(16);
    assert_eq!(s.capacity(), 16);
    assert_eq!(s.len(), 0);
    for i in 0..2 {
        s.push_str("omar");
        assert_eq!(s.len(), 4 * (i + 1));
        assert_eq!(s.capacity(), 16);
    }
}

#[test]
fn lenght_is_number_of_bytes() {
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

    let s = "hello, 学中文";
    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '学');
        }
    }
}
