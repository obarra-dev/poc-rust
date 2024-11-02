use std::vec;

use some_tests::type_of;

#[test]
fn vector() {
    let arr = [1, 2, 3];
    let v = Vec::from(arr);
    assert_eq!(v, vec![1, 2, 3]);
    assert_eq!(type_of(&v), "alloc::vec::Vec<i32>");

    let v: Vec<i32> = arr.into();
    assert_eq!(v, vec![1, 2, 3]);

    let v = vec![1, 2, 3];
    assert_eq!(v, vec![1, 2, 3]);
    assert_eq!(type_of(&v), "alloc::vec::Vec<i32>");

    // TODO  why there are two ways to create a vector?
    let v = vec![1, 2, 3];
    assert_eq!(v, vec![1, 2, 3]);
    assert_eq!(type_of(&v), "alloc::vec::Vec<i32>");

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    assert_eq!(v, vec![1, 2, 3]);
    assert_eq!(type_of(&v), "alloc::vec::Vec<i32>");

    // this is different from the previous examples
    // vector of arrays, each element is an array of 3 elements of type i32
    let mut v = vec![arr];
    assert_eq!(type_of(&v), "alloc::vec::Vec<[i32; 3]>");
    assert_eq!(v, vec![[1, 2, 3]]);
    v.push([1, 2, 3]);
    assert_eq!(v.len(), 2);
}

#[test]
fn vector_methods() {
    // getting individual elements
    let v = vec![1, 2, 3];
    let mut count = 0;
    for i in &v {
        count = count + *i;
    }
    assert_eq!(count, 6);

    let v = Vec::from([1, 2, 3]);
    let mut v1: Vec<i32> = Vec::new();
    v1.extend(&v);
    assert_eq!(v1, v);

    // CRUD
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    assert_eq!(v, vec![1, 2]);
    let d = v.pop();
    assert_eq!(d, Some(2));
    assert_eq!(v, vec![1]);

    let v = Vec::from([1]);
    let mut s = String::new();
    for i in 0..2 {
        // this will panic if the index is out of bounds
        //let m = v[i];

        // get(i) is the safe way to get an element
        let aux = format!("{:?}", v.get(i));
        s.push_str(&aux);
    }
    assert_eq!(s, "Some(1)None");

    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        match v.get(i) {
            Some(e) => {
                v[i] = e + 1;
            }
            None => {
                v.push(i + 2);
            }
        }
    }
    assert_eq!(v, vec![2, 3, 4, 5, 6]);
}

#[test]
fn string_to_vec() {
    let s = String::from("omar");
    let v: Vec<u8> = s.into();
    assert_eq!(v, vec![111, 109, 97, 114]);

    // seems that into is the same as into_bytes
    let s = String::from("omar");
    let v = s.into_bytes();
    assert_eq!(v, vec![111, 109, 97, 114]);

    let s = "omar";
    let v: Vec<u8> = s.into();
    assert_eq!(v, vec![111, 109, 97, 114]);

    let s = "omar";
    let v = Vec::from(s);
    assert_eq!(v, vec![111, 109, 97, 114]);
}

#[test]
fn collect_to_create_vec() {
    // collect is a java collector jeje
    let v: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v, vec![0; 10]);
    assert_eq!(v, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn vec_to_slice() {
    let v = vec![1, 2, 3];
    let s = &v[..];
    assert_eq!(s, &[1, 2, 3]);

    let v = vec![1, 2, 3];
    let s = &v[0..v.len()];
    assert_eq!(s, &[1, 2, 3]);

    let v = vec![1, 2, 3];
    let s = v.as_slice();
    assert_eq!(s, &[1, 2, 3]);

    // MUTABLE
    let mut v = vec![1, 2, 3];
    // this is not a simple mutable slice
    let vec_ref = &mut v;
    vec_ref[1] = 4;
    (*vec_ref)[0] = 4;
    vec_ref.push(5);
    assert_eq!(vec_ref, &[4, 4, 3, 5]);
    assert_eq!(v, vec![4, 4, 3, 5]);

    let mut v = vec![1, 2, 3];
    // TODO what is the difference between &mut v[..] and &mut v[0..3]?
    // theoretically we cannot use slices to modify the vector, so why?
    let vec_ref = &mut v[0..3];
    vec_ref[0] = 4;
    assert_eq!(vec_ref, &[4, 2, 3]);

    let mut v = vec![1, 2, 3];
    let vec_ref = v.as_mut_slice();
    vec_ref[0] = 4;
    assert_eq!(vec_ref, &[4, 2, 3]);
}

#[test]
fn capacity_and_len() {
    let mut s = Vec::new();
    assert_eq!(s.capacity(), 0);
    assert_eq!(s.len(), 0);

    // by default the capacity has its own formula
    for i in 0..5 {
        s.push("omar");
        assert_eq!(s.len(), i + 1);
        assert!(s.capacity() == 4 || s.capacity() == 8);
    }

    let mut s = Vec::with_capacity(16);
    assert_eq!(s.capacity(), 16);
    assert_eq!(s.len(), 0);
    for i in 0..5 {
        s.push("omar");
        assert_eq!(s.len(), i + 1);
        assert_eq!(s.capacity(), 16);
    }
}

#[test]
fn store_distint_types() {
    // we can use enums or trait object to store different types

    // using enum
    #[derive(Debug, PartialEq)]
    enum IpAddrEnum {
        V4(String),
        V6(String),
    }

    let v = vec![
        IpAddrEnum::V4(String::from("127.0.0.1")),
        IpAddrEnum::V6(String::from("::1")),
    ];

    assert_eq!(v[0], IpAddrEnum::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddrEnum::V6("::1".to_string()));

    // using trait objects
    trait IpAddrTrait {
        fn get_value(&self) -> String;
    }

    struct V4(String);
    impl IpAddrTrait for V4 {
        fn get_value(&self) -> String {
            self.0.clone()
        }
    }

    struct V6(String);
    impl IpAddrTrait for V6 {
        fn get_value(&self) -> String {
            self.0.clone()
        }
    }

    let v: Vec<Box<dyn IpAddrTrait>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    // dynamic dispatch is working
    assert_eq!(v[0].get_value(), "127.0.0.1");
    assert_eq!(v[1].get_value(), "::1");
}
