use std::{thread, vec};

use some_tests::type_of;

#[test]
fn string_test() {
    // String type: heap allocated, growable and utf-8 encoded great for Safety!!
    // called an owned type because it owns the underlying data and is responsible for cleaning  it up
    // This type contents pointer to heap, length and capacity
    // Use case: create or modify string data dynamically at runtime
    // String is just a Vec<u8> with some methods that make it convenient to work with Unicode text
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
    // string literal type &str is a variant of string slice (&str)
    // string literal is a hardcode in the binary itself
    // so the size is known at compile time
    // string literal is also considering a string slice because it is Immutable (a read-only view) and it a reference to a static memory
    let s = "omar";
    assert_eq!(s, "omar");
    assert_eq!(type_of(&s), "&str");

    // &'static str is sintactic sugar for a string slice with a 'static lifetime
    // this guarantees that this point to a string that is valid for the entire duration of the program
    let s: &'static str = "omar";
    assert_eq!(s, "omar");
    assert_eq!(type_of(&s), "&str");

    // &'static is not necessary on above example

    // &'static is necessary for storing string slices in struts or enums
    enum MyError {
        SomeError(&'static str),
        // does not compile, as str does not have 'static lifetime
        //SomeError2(& str),
    }
    let error = MyError::SomeError("an error occurred");
    assert!(matches!(error, MyError::SomeError(_)));

    fn get_static_str() -> &'static str {
        "omar"
    }

    // &'static is necessary function that retunrs slice that does not have other borrowed parameters
    let s = get_static_str();
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
fn raw_string_literal() {
    // allows special characters like backslashes or quotes
    // useful for regex
    let r = r#"
{
    "userId": 1,
    "id": 4,
    "title": "omar rules",
    "complete": false
}"#;

    assert_eq!(r, "\n{\n    \"userId\": 1,\n    \"id\": 4,\n    \"title\": \"omar rules\",\n    \"complete\": false\n}");

    // byte string literal
    // create a slice of bytes, type &[u8; N]
    // useful for network protocols that expect data in bytes
    let http_ok = b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
    assert_eq!(type_of(&http_ok), "&[u8; 51]");

    // combine raw string literal and byte string literal
    let png_signature = br"\x89PNG\r\n\x1a\n";
    assert_eq!(type_of(&png_signature), "&[u8; 17]");
}

#[test]
fn string_utf8() {
    // String is a wrapper over Vec<u8> but Vec<u8> can be useful dealing with binary

    let v = vec![111, 109, 97, 114];
    let s = String::from_utf8(v).unwrap();
    assert_eq!(s, "omar");

    let v = vec![111, 109, 97, 114];
    // Converts a slice of bytes to a string, including invalid characters
    //  why Cow?
    // Answer: If the byte slice &v contains only valid UTF-8, it returns a borrowed string slice (Cow::Borrowed(&str)) without allocating new memory.
    // If the byte slice contains invalid UTF-8, it allocates a new String, replacing invalid bytes with the Unicode replacement character (�), and returns an owned string
    let s = String::from_utf8_lossy(&v);
    assert_eq!(s, "omar");

    fn latin_to_string(slice: &[u8]) -> String {
        slice.iter().map(|&b| b as char).collect()
    }

    let utf8_data = latin_to_string(&v);
    assert_eq!(utf8_data, "omar");
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
    // Box is a smart pointer
    // Box<str> represents a owned, non-growable and heap-allocated string slice
    // the box is into the stack and the underlaying data is in the heap
    // so the underlaying data which is "omar" is in the heap but the box itself is in the stack

    // convert String to Box<str>
    let s = String::from("omar");
    let b = s.into_boxed_str();
    assert_eq!(type_of(&b), "alloc::boxed::Box<str>");
    let f = format!("{}", b);
    assert_eq!(f, "omar");
    // TODO why both ways works?
    // only &  to convert Box<str> to &str but explicitly typing
    let x: &str = &b;
    assert_eq!(x, "omar");
    assert_eq!(type_of(&x), "&str");
    // &* to convert Box<str> to &str without explicitly typing
    let x = &*b;
    assert_eq!(x, "omar");
    assert_eq!(type_of(&x), "&str");

    // does not compile, the size cannot be known at compile time
    // let x = *b;

    // converts string slice to Box<str> explicitly typing
    let b: Box<str> = "omar".into();
    assert_eq!(type_of(&b), "alloc::boxed::Box<str>");

    // use case: to freeze a string to prevent modifications or droping the extra capacity information
    // when you want to return an own string that will not be modified further
    // when you want to aggresive optimize memory usage by removing extra capacity information

    // TODO  I don't have any specific use cases in mind.
}

#[test]
fn reference_counted_test() {
    use std::rc::Rc;
    // Rc is a reference counted smart pointer
    // used for shared ownership of data in single-threaded scenarios
    // so not thread safe
    // useful to share ownership of a immutable string slice accros multiple parts of a program without cloning the data
    let s = String::from("omar");
    let rc1 = Rc::new(s);
    assert_eq!(Rc::strong_count(&rc1), 1);

    let rc2 = Rc::clone(&rc1);
    assert_eq!(Rc::strong_count(&rc1), 2);
    assert_eq!(Rc::strong_count(&rc2), 2);

    {
        let rc3 = Rc::clone(&rc1);
        assert_eq!(Rc::strong_count(&rc1), 3);
        assert_eq!(Rc::strong_count(&rc3), 3);
    } // rc3 goes out of scope here

    assert_eq!(Rc::strong_count(&rc1), 2);
    // TODO find real use case
    // beneficial when dealing with large strings that are expensive to clone
}

#[test]
fn stands_atomically_reference_counted_test() {
    use std::sync::Arc;
    // Arc is a thread-safe reference counted smart pointer
    // used for shared ownership of data across multiple threads
    // useful to share ownership of a immutable string slice accros multiple threads without cloning the data
    let s = String::from("omar");
    let arc1 = Arc::new(s);
    assert_eq!(Arc::strong_count(&arc1), 1);

    let arc2 = Arc::clone(&arc1);
    assert_eq!(Arc::strong_count(&arc1), 2);
    assert_eq!(Arc::strong_count(&arc2), 2);

    {
        let arc3 = Arc::clone(&arc1);
        assert_eq!(Arc::strong_count(&arc1), 3);
        assert_eq!(Arc::strong_count(&arc3), 3);
    } // arc3 goes out of scope here

    assert_eq!(Arc::strong_count(&arc1), 2);

    // TODO find real use case
    // beneficial when dealing with large strings that are expensive to clone

    // other example with multiple threads
    let text_string = String::from("This some text that multiple thread will read");
    let shared_text = Arc::new(text_string);
    let mut handles = vec![];
    for _ in 0..4 {
        let text_ref = Arc::clone(&shared_text);
        let handle = thread::spawn(move || {
            println!("{}", text_ref);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn mutable_reference_test() {
    // mutable reference to String
    // modified slice are generally avoided in Rust due to the complexities and potential pitfalls associated with UTF-8 encoding
    // you will see this low level operation in libraries or in code needs to be aggresively optimized
    let mut s = String::from("omar");
    {
        let s_ref: &mut String = &mut s;
        s_ref.push_str(" barra");
    }
    assert_eq!(s, "omar barra");

    // TODO make a function to ensure_single_at using this
}

#[test]
fn copy_on_write_cow_test() {
    // cow enum cow stands for copy on write is a smart pointer
    // useful for funtion that sometimes modifies a string and other times does not
    // and you avoid new allocation in cases where no modification is needed

    // it retunrs a Cow that either borrows the original data or owns a modified version of it
    fn sanitize_input(input: &str) -> std::borrow::Cow<str> {
        if input.contains("java") {
            let sanitized = input.replace("java", "rust");
            std::borrow::Cow::Owned(sanitized)
        } else {
            std::borrow::Cow::Borrowed(input)
        }
    }

    let c = sanitize_input("omar java");
    assert_eq!(type_of(&c), "alloc::borrow::Cow<'_, str>");
    assert_eq!(c, "omar rust");
}

#[test]
fn string_types_for_interoperability_test() {
    // TODO find great examples
    // OsString and OsStr
    // PathBuf and Path
    // CString and CStr
}
