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

#[test]
fn rev() {
    let s = String::from("omar");
    let chars = s.chars();
    let rev = chars.rev();
    let s_reverted:String = rev.collect();
    assert_eq!(s_reverted, "ramo");

    let s_reverted:String = s.chars().rev().collect();
    assert_eq!(s_reverted, "ramo");

    let s_reverted:Vec<char> = s.chars().rev().collect();
    assert_eq!(s_reverted, vec!['r', 'a', 'm', 'o']);
}