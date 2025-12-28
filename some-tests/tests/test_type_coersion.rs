use some_tests::type_of;

#[test]
fn type_coersion() {
    let a = 1000;
    let v = a as u8; // 1000 - 256 - 256 -256 = 232
    assert_eq!(v, 232);

    let a = -1_i8;
    let v = a as u8; // -1 + 256 = 255
    assert_eq!(v, 255);

    let a = 97.123_f32;
    let v = a as u8; // 97
    assert_eq!(v, 97);

    let c = a as u8 as char;
    assert_eq!(c, 'a');

    let c = v as char;
    assert_eq!(c, 'a');
}
