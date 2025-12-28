#[test]
fn array() {
    // infers the type is [i32; 3]
    let arr = [1, 2, 3];
    // allocated on the stack so the memory they occupy is fixed for their entire lifetime.
    assert_eq!(arr.len(), 3);
    //size_of_val returns the bytes which an array occupies in memory
    assert_eq!(std::mem::size_of_val(&arr), 12);

    // all elements in an array can be initialized with the same value at once
    let arr = [4; 4];
    assert_eq!(arr.len(), 4);
    assert_eq!(arr[0], 4);
    assert_eq!(arr[3], 4);

    // accessing elements using indexing and references
    let arr = [1, 2, 3];
    let x = &arr[0];
    // TODO question: &1 what is it?
    assert_eq!(x, &1);

    let arr = ["omar", "barra"];
    // panic if the index is out of bounds
    // assert_eq!(arr[7], "omar");

    // we can use index to access the elements but is not safe
    assert_eq!(arr[0], "omar");

    // option is a safe way to access the elements
    let optional = arr.get(7);
    assert_eq!(optional, None);

    let optional = arr.get(0);
    // TODO why &"omar"?
    assert_eq!(optional, Some(&"omar"));
}

#[test]
fn array_mutable() {
    let arr = [1, 2, 3];
    // does not compile, as the array is not mutable
    // arr[0] = 4;

    // arr is mutable, so we can change the values
    let mut arr = [1, 2, 3];
    arr[0] = 4;
    assert_eq!(arr[0], 4);

    // other way using mutable reference
    let mut arr = [1, 2, 3];
    let x = &mut arr[0];
    *x = 4;
    assert_eq!(arr[0], 4);

    let mut arr = [1, 2, 3];
    let x = &arr[0];
    assert_eq!(x, &1);
    let y = &mut arr[0];
    *y = 4;
    assert_eq!(arr[0], 4);
}

#[test]
fn slice() {
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(arr.len(), 5);

    // does not compile, error the size for values of type `[{integer}]` cannot be known at compilation time
    // let slice = arr[1..3];

    // type is &[i32] slice of i32
    // we cannot use slice directly, we have to use the reference of the slice
    let slice = &arr[1..3];
    assert_eq!(slice.len(), 2);
    assert_eq!(slice, [2, 3]);
    assert_eq!(slice, &[2, 3]);
    // TODO question: why both work?
}

#[test]
fn slice_size_of_val() {
    let arr = ['学', '中'];
    assert_eq!(arr.len(), 2);
    // 8 = 4 bytes for each character * 2 characters
    assert_eq!(std::mem::size_of_val(&arr), 8);

    let slice = &arr[..2];
    // 16 = (4 bytes for pointer + 4 bytes of length ) * 2 characters
    // slice is a fat pointer as it contains a pointer to the data and the length of the slice
    assert_eq!(std::mem::size_of_val(&slice), 16);
    let s = format!("{:?}", slice);
    assert_eq!(s, "['学', '中']");
}

#[test]
fn slice_string() {
    // string slice is also a slice
    let s = String::from("学中");
    let slice = &s[0..3];
    assert_eq!(slice, "学");

    let s = "学中";
    // it takes 3 bytes
    let slice = &s[0..3];
    assert_eq!(slice, "学");

    // TODO example with string slice
    // https://doc.rust-lang.org/std/string/struct.String.html#method.clear
}