#[test]
fn inmmutable_borrowed() {
    let count = 0;
    // count can be borrowed as inmmutable many times
    let reborrow = &count;
    let rereborrow = &count;

    assert_eq!(*reborrow, 0);
    assert_eq!(*rereborrow, 0);
    assert_eq!(count, 0);
}

#[test]
fn mutable_borrowed() {
    let mut count = 0;
    let reborrow = &mut count;
    // it does not compile since count can be borrowed as mutable only once
    //let rereborrow = &mut count;

    // it does not compile, count can not be borrowed as mutable and then as inmmutable at the same time
    //let rereborrow = &count;
    assert_eq!(*reborrow, 0);
    assert_eq!(count, 0);
}
