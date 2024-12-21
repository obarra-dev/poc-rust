#[test]
fn  danglin_reference() {
    // lifetime of r is valid until the end of the test
    let r;
    {
        // life time of x is valid until the end of this scope
        let x = 1;
        r = &x;
    }
    
    // this does not compile, error `x` does not live long enough
    // since r points to something that is not valid anymore, dangling pointer/references
    // println!("r: {}", r);
}

#[test]
fn values_lives_on_its_scope() {
    // v has the longest lifetime because its scope enterily encloses both borrow and borrow2
    let v = 4;
    {
        let borrow_r = &v;
        // TODO why it does not need to dereference
        println!("borrow_r: {}", borrow_r);
        // TODO a better way to do it?
        assert_eq!(*borrow_r, 4);
   }
    
   {
    let borrow_r2 = &v;
    assert_eq!(*borrow_r2, 4);
}


}