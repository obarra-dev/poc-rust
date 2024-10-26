// concrete type A
struct A;
// concrete type S, take as argument a instance of A
struct S(A);
// Generic type SGen
struct SGen<T>(T);

#[test]
fn generic() {
    fn reg_fn(_s: S) {}
    reg_fn(S(A));

    fn gen_spec_t(_s: SGen<A>) {}
    gen_spec_t(SGen(A));

    fn gen_spec_i32(_s: SGen<i32>) {}
    gen_spec_i32(SGen(4));

    fn generic<T>(_s: SGen<T>) {}
    // explicitly specified type parameter
    generic::<i32>(SGen(4));
    // implicitly specified type parameter
    generic(SGen('a'));

    fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }

    assert_eq!(sum(1, 2), 3);
    assert_eq!(sum(1.2, 2.5), 3.7);
}

#[test]
fn generic_struct() {
    struct Point<T> {
        x: T,
        y: T,
    }

    let p = Point { x: 5, y: 4 };
    assert_eq!(p.y, 4);
    let p = Point { x: 5.0, y: 4.4 };
    assert_eq!(p.y, 4.4);

    struct MyError<T, U> {
        code: T,
        desc: U,
    }

    let e = MyError {
        code: 404,
        desc: "Not Found".to_string(),
    };
    assert_eq!(e.desc, "Not Found");

    struct Val<T> {
        val: T,
    }

    impl<T> Val<T> {
        fn value(&self) -> &T {
            &self.val
        }
    }

    let v = Val { val: 3 };
    assert_eq!(*v.value(), 3);

    let v = Val {
        val: "omar".to_string(),
    };
    assert_eq!(v.value(), "omar");

    impl<T, U> MyError<T, U> {
        fn mixup<V, W>(self, other: MyError<V, W>) -> MyError<T, W> {
            MyError {
                code: self.code,
                desc: other.desc,
            }
        }
    }

    let e = MyError {
        code: 404,
        desc: "Not Found".to_string(),
    };
    let e2 = MyError {
        code: "500",
        desc: 1234328,
    };
    let mixed = e.mixup(e2);
    assert_eq!(mixed.code, 404);
    assert_eq!(mixed.desc, 1234328);

    impl Point<f64> {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point { x: 3.0, y: 4.0 };
    assert_eq!(p.distance_from_origin(), 5.0);

    let p = Point { x: 3, y: 4 };
    // does not compile, the method is only implemented for Point<f64>
    //  p.distance_from_origin()
}

#[test]
fn const_generic() {
    struct Array<T, const N: usize> {
        data: [T; N],
    }

    let a: Array<i32, 3> = Array { data: [1, 2, 3] };
    assert_eq!(a.data.len(), 3);

    let array_of_array: Array<Array<i32, 3>, 2> = Array {
        data: [Array { data: [1, 2, 3] }, Array { data: [4, 5, 6] }],
    };
    assert_eq!(array_of_array.data.len(), 2);

    let arrays: [Array<f64, 2>; 2] = [Array { data: [1.0, 2.0] }, Array { data: [4.0, 5.0] }];
    assert_eq!(arrays.len(), 2);
}
