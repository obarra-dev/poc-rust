use std::collections::HashMap;

use some_tests::type_of;

#[test]
fn hash_map() {
    let mut hash_map = HashMap::new();
    hash_map.insert("omar", 4);
    hash_map.insert("barra", 9);
    assert_eq!(hash_map.len(), 2);

    // the safe way to get a value from a hash map
    let v = hash_map.get("omar");
    assert_eq!(v, Some(&4));

    let v = hash_map["barra"];
    assert_eq!(v, 9);

    if hash_map.contains_key("omar") {
        let v = hash_map.remove("omar");
        assert_eq!(v, Some(4));
    }
    assert_eq!(hash_map.len(), 1);

    for (k, v) in hash_map {
        let aux = format!("{:?} {:?}", k, v);
        assert_eq!(aux, "\"barra\" 9");
    }

    // tuple to hash map, using from
    let names = [("omar", 4), ("barra", 9)];
    let hash_map = HashMap::from(names);
    assert_eq!(hash_map.len(), 2);

    // tuple to hash map, using collect
    let hash_map: HashMap<&str, i32> = names.into_iter().collect();
    assert_eq!(hash_map.len(), 2);
}

#[test]
fn or_insert() {
    let mut hash_map = HashMap::new();
    hash_map.insert("omar", 4);

    fn generate_value() -> i32 {
        9
    }
    hash_map.entry("barra").or_insert_with(generate_value);

    hash_map.entry("alberto").or_insert(10);
    assert_eq!(hash_map.len(), 3);

    // return a mutable reference to the value in the entry
    let value = hash_map.entry("alberto").or_insert(100);
    assert_eq!(*value, 10);
    *value -= 6;
    assert_eq!(*value, 4);
}

#[test]
fn or_insertdd() {
    #[derive(Debug, Eq, PartialEq, Hash)]
    struct Foo(String);

    let hash_map = HashMap::from([(Foo("omar".to_string()), 4), (Foo("barra".to_string()), 9)]);

    for (k, v) in &hash_map {
        assert_eq!(type_of(v), "i32");
        assert_eq!(type_of(k), "test_hash_map::or_insertdd::Foo");
        assert!(v == &4 || v == &9);
        assert!(k == &Foo("omar".to_string()) || k == &Foo("barra".to_string()));
    }
}

#[test]
fn capacity_and_len() {
    let mut s: HashMap<String, i32> = HashMap::new();
    assert_eq!(s.capacity(), 0);
    assert_eq!(s.len(), 0);

    // by default the capacity has its own formula
    for i in 0..5 {
        s.insert(i.to_string() + "omar", 4);
        assert_eq!(s.len(), i + 1);
        assert!(s.capacity() == 3 || s.capacity() == 7);
    }

    let mut s: HashMap<String, i32> = HashMap::with_capacity(16);
    // the real capacity could be bigger than the initial capacity
    assert_eq!(s.capacity(), 28);
    assert_eq!(s.len(), 0);

    for i in 0..5 {
        s.insert(i.to_string() + "omar", 4);
        assert_eq!(s.len(), i + 1);
        println!("capacity: {}", s.capacity());
        assert_eq!(s.capacity(), 28);
    }

    s.shrink_to(8);
    assert!(s.capacity() >= 8);

    s.shrink_to_fit();
    assert!(s.capacity() >= 5);
}

#[test]
fn capacity_and_lenss() {
    let v = 4;
    let mut h = HashMap::new();
    h.insert(v, v);
    assert_eq!(v, 4);

    let v = "omar".to_string();
    let mut h = HashMap::new();
    // ownership of v is moved to the hash map
    h.insert(v, 4);
    // v cannot be used anymore, as it was moved
    //assert_eq!(v, "omar".to_string());

    let v = "omar".to_string();
    let mut h: HashMap<&str, i32> = HashMap::new();
    // this is a reference to v, so the ownership is not moved
    h.insert(&v, 4);
    assert_eq!(v, "omar".to_string());
}
