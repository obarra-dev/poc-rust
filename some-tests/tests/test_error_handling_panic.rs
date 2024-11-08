#[test]
#[should_panic(expected = "assertion `left == right` failed\n  left: \"omar\"\n right: \"barra\"")]
fn assert_cause_panic() {
    assert_eq!("omar", "barra");
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 3 but the index is 99")]
fn index_out_of_bounds_cause_panic() {
    let v = vec![1, 2, 3];
    v[99];
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn unwrap_cause_panic() {
    let v: Option<i32> = None;
    v.unwrap();
}

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn devide_0_cause_panic() {
    fn devide(x: i32, y: i32) -> i32 {
        x / y
    }
    devide(4, 0);
}

#[test]
#[should_panic(expected = "attempt to multiply with overflow")]
fn d_cause_panic() {
    fn dosome(x: u8) -> f64 {
        let y: u8 = 221;
        (x * y) as f64 // 2 * 221 = 442
    }
    dosome(2);
}
