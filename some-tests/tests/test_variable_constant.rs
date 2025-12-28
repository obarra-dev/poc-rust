#[test]
fn variables() {
    // it is immutalbe, so cannot assign twice to immutable variable
    let x = 4;
    assert_eq!(x, 4);

    // with mut, it is mutable
    let mut y = 4;
    assert_eq!(y, 4);
    y = 10;
    assert_eq!(y, 10);

    // rust allows to overrides variables, it is named shadowing
    // the first variable is shadowed by the second one
    let z = 4;
    assert_eq!(z, 4);
    let z = 20;
    assert_eq!(z, 20);

    // rust allows to overrides variables changing the type
    let a = 4;
    assert_eq!(a, 4);
    let a = "omar";
    assert_eq!(a, "omar");

    // underscore is to prevent warnings, #[allow(unused_variables)] also can be used
    let _v = 4;
    assert_eq!(_v, 4);
}

#[test]
fn virables_shadowing() {
    let x = 4;
    assert_eq!(x, 4);

    {
        // interior SCOPE, this x valuse only lives here
        let x = x - 2;
        assert_eq!(x, 2);
    }

    let x = x + 1;
    assert_eq!(x, 5);
}

#[test]
fn constant() {
    // compiler forces to be UPERCASE and define a type
    // a const cannot be override or redifine
    const X: u8 = 4;
    assert_eq!(X, 4);
}
