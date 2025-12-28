#[test]
fn size_of_val() {
    let c = 'o';
    let s = std::mem::size_of_val(&c);
    assert_eq!(s, 4);

    let c = '🙀';
    let s = std::mem::size_of_val(&c);
    assert_eq!(s, 4);

    let c = '中';
    let s = std::mem::size_of_val(&c);
    assert_eq!(s, 4);
}
