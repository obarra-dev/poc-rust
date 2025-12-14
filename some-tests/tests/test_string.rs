use std::vec;

use some_tests::type_of;

#[test]
fn string_test() {
    // String type: heap allocated, growable and utf-8 encoded great for Safety!!
    // called an owned type because it owns the underlying data and is responsible for cleaning  it up
    // This type contents pointer to heap, length and capacity
    // Use case: create or modify string data dynamically at runtime
    // String is just a Vec<u8> with some methods
    let s = String::from("omar");
    assert_eq!(s, "omar");
    assert_eq!(type_of(&s), "alloc::string::String");
}

#[test]
fn string_slice_test() {
    // string slice type (&str) is a view into a String or string literal
    // called a borrow type because it does not own the underlying  data
    // This type contents pointer to heap and only length
    // Can point to data anywhere (stack, heap, or static memory).
    // Use case: great for read only operations over pre-existing string data
    let s = "omar";
    assert_eq!(s, "omar");
    assert_eq!(type_of(&s), "&str");
}

#[test]
fn string_literal_test() {
    // string literary, type &str
    // string literary is a hardcode in the binary itself
    // so the size is known at compile time
    // string literary is also considering a string slice because it is Immutable (a read-only view) and it a reference to a static memory
    let s = "omar";
    assert_eq!(s, "omar");
    assert_eq!(type_of(&s), "&str");
}

#[test]
fn string_move_ownership() {
    let s = String::from("omar");
    // new_s is the mutable owner of the underlying data
    let mut new_s = s;
    new_s.push_str(" barra");
    assert_eq!(new_s, "omar barra");

    // does not compile, as the ownership of s was moved
    // assert_eq!(s, "omar");
}

#[test]
fn string_clone_to_not_move_ownership() {
    fn move_ownership(s: String) -> String {
        // s is the owner of the underlying data
        let mut new_s = String::from("barra");
        new_s.push_str(&s);
        // the ownership of the content of new_s will be moved to the caller
        new_s
    }

    let s = String::from("omar");
    // we can use clone to avoid moving the ownership
    let cloned_s = s.clone();
    // after calling move_ownership, cloned_s cannot be used since its ownership was moved
    let x = move_ownership(cloned_s);
    assert_eq!(x, "barraomar");
    assert_eq!(s, "omar");

    // does not compile, as the ownership of s was moved
    // assert_eq!(cloned_s, "omar");
}

#[test]
fn string_to_string_slice() {
    // String type
    let s = String::from("omar");
    assert_eq!(type_of(&s), "alloc::string::String");

    // first way to convert String to string slice (&str), using &s[..]
    // most common way
    let x = &s[..];
    assert_eq!(x, "omar");
    assert_eq!(type_of(&x), "&str");

    // second way to convert String to string slice (&str), using as_str()
    let x = s.as_str();
    assert_eq!(x, "omar");
    assert_eq!(type_of(&x), "&str");

    // third way to convert String to string slice (&str), using &s
    // question: I have to define the type explicitly, why?
    // answer by default, &s where s is a String gives you a reference to the whole String object, which is of type &String, not &str.
    // Rust does not automatically COERCE &String to &str unless the type is explicitly required
    let x: &str = &s;
    assert_eq!(x, "omar");
    assert_eq!(type_of(&x), "&str");

    // &String can be used where &str is expected because of Deref coercion
    // Deref coercion: mechanism that allows the compiler to automatically convert a reference to a type that implements the Deref trait into a reference to its target type. Because the standard library implements Deref<Target = str> for String, any &String can be used where a &str is expected
    let x = &s;
    assert_eq!(type_of(&x), "&alloc::string::String");
}

#[test]
fn string_to_string_slice_with_range() {
    let s = String::from("omar");
    // slice from 0 to 2
    let x = &s[..2];
    assert_eq!(x, "om");

    // slice from 2 to the end
    let x = &s[2..];
    assert_eq!(x, "ar");

    // slice the whole string
    let x = &s[..];
    assert_eq!(x, "omar");
}

#[test]
fn string_slice_to_string() {
    // string slice type (&str)
    let s = "omar";
    assert_eq!(type_of(&s), "&str");

    // convert &str to String
    // to_string() allocates memory on the heap, it is more readable
    let x = s.to_string();
    assert_eq!(type_of(&x), "alloc::string::String");

    let y = String::from(s);
    assert_eq!(type_of(&x), type_of(&y));

    // not only for string, Often used to convert any borrowed type (&T) into its owned counterpart (T)
    let z = s.to_owned();
    assert_eq!(type_of(&x), type_of(&z));
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
fn length_is_number_of_bytes() {
    let s = String::from("omar");
    assert_eq!(s.len(), 4);
    let s = String::from("读写汉字 - 学中文");
    // length is not the number of characters, it is the number of bytes
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

#[test]
fn scape_characters() {
    // \ is for scape characters
    let s = "this is Ru\x73\x74";
    assert_eq!(s, "this is Rust");
}

#[test]
fn string_utf8() {
    // TODO is a native funtio?
    // let s = "hello, 中文 ";
    // let ds = utf8_slice::slice(s, 7, 8);

    let v = vec![111, 109, 97, 114];
    let s = String::from_utf8(v).unwrap();
    assert_eq!(s, "omar");

    let v = vec![111, 109, 97, 114];
    // Converts a slice of bytes to a string, including invalid characters
    // TODO why Cow?
    let s = String::from_utf8_lossy(&v);
    assert_eq!(s, "omar");
}

#[test]
fn concat() {
    let s = String::from("omar");
    let x = String::from("barra");
    // we can concat a String with &str, we cannot concat a String with a String
    // & convert String to string slice (&str)
    let y = s + &x;
    assert_eq!(y, "omarbarra");
    // does not compile, as the ownership of s was moved
    // assert_eq!(s, "omar");

    // using push_str to concat
    let mut owned_string = String::from("omar");
    let borrowed_string = "barra";
    owned_string.push_str(borrowed_string);
    assert_eq!(owned_string, "omarbarra");
    // using push to add a single character
    owned_string.push('!');
    assert_eq!(owned_string, "omarbarra!");

    // this way is valid
    owned_string += "!";
    assert_eq!(owned_string, "omarbarra!!");

    // using format! macro to concat
    let owned_string = String::from("omar");
    let owned_string2 = String::from("barra");
    let s3 = format!("{}{}", owned_string, owned_string2);
    assert_eq!(s3, "omarbarra");
}

#[test]
fn string_replace() {
    let s = String::from("omar java");
    // it allocate new memory and store the modified string there, it does not modify the original one
    let x = s.replace("java", "barra");
    assert_eq!(s, "omar java");
    assert_eq!(x, "omar barra");
}

#[test]
fn string_parse() {
    let r = "4".parse::<i32>();
    match r {
        Ok(n) => assert_eq!(n, 4),
        Err(_) => panic!("parse error"),
    }

    let r = "omar".parse::<i32>();
    match r {
        Ok(n) => assert_eq!(n, 4),
        Err(e) => assert_eq!(e.to_string(), "invalid digit found in string"),
    }
}

#[test]
fn box_test() {
    // we can use str only by boxed it, it has to be explicitly typed
    // omar is converted into a Box<str>, heap allocated
    let s: Box<str> = "omar".into();
    // & can be used to convert Box<str> to &str, it has to be explicitly typed
    let x: &str = &s;
    assert_eq!(x, "omar");
}
