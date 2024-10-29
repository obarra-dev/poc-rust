use std::fmt::{format, Debug, Display};

use some_tests::type_of;

trait Shape {
    fn my_default_method(&self) -> String {
        let own_type = type_of(&self);
        format!("default method for {}", own_type)
    }

    fn area(&self) -> i32;
}

struct Rectangle {
    width: i32,
    height: i32,
}

impl Shape for Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

// the struct can have other methods different from the trait
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Triangle {
    base: i32,
    height: i32,
}

impl Shape for Triangle {
    fn area(&self) -> i32 {
        self.base * self.height / 2
    }
}

#[test]
fn trait_test() {
    // question: why cannot I put the Shape as a type here?
    let rectangle = Rectangle {
        width: 7,
        height: 6,
    };

    // the static dispatch is used here, it generates the method area for rectangle
    let area = rectangle.area();
    assert_eq!(area, 42);

    let r = rectangle.can_hold(&Rectangle {
        width: 4,
        height: 3,
    });
    assert!(r);

    let triangle = Triangle { base: 4, height: 3 };
    // the static dispatch is used here, it generates the method area for triangle
    let area = triangle.area();
    assert_eq!(area, 6);
}

#[test]
fn trait_default_method() {
    let rectangle = Rectangle {
        width: 7,
        height: 6,
    };

    // default method does not need to be implemented
    let default_method = rectangle.my_default_method();
    assert_eq!(default_method, "default method for &test_traits::Rectangle");
}

#[test]
fn trait_as_parameter() {
    fn get_area_formated(s: &impl Shape) -> String {
        format!("The area is {}", s.area())
    }

    let rectangle = Rectangle {
        width: 7,
        height: 6,
    };
    let area = get_area_formated(&rectangle);
    assert_eq!(area, "The area is 42");

    let triangle = Triangle { base: 4, height: 3 };
    let area = get_area_formated(&triangle);
    assert_eq!(area, "The area is 6");
}

#[test]
fn trait_as_return() {
    fn return_rectangle() -> impl Shape {
        Rectangle {
            width: 7,
            height: 6,
        }
    }
    let rectangle = return_rectangle();
    assert_eq!(rectangle.area(), 42);

    fn return_triangle() -> impl Shape {
        Triangle { base: 4, height: 3 }
    }
    let triangle = return_triangle();
    assert_eq!(triangle.area(), 6);

    // this does not compile because the compiler does not know the size of the return type
    // you have to use trait objects
    /*
    fn return_shape(isRectangle: bool) -> impl Shape {
        if isRectangle {
            Rectangle {
                width: 7,
                height: 6,
            }
        } else {
            Triangle { base: 4, height: 3 }
        }
    }
    */
}

#[test]
fn trait_objects_dynamic() {
    trait Animal {}
    struct Dog;
    struct Cat;

    impl Animal for Dog {}
    impl Animal for Cat {}

    // as the trait object is behind a pointer, the size is known at compile time, which use usize
    // the exact return type does not have to be known at compile time as long as the size fixed
    fn return_animal(s: &str) -> &dyn Animal {
        match s {
            "dog" => &Dog,
            "cat" => &Cat,
            _ => panic!("unknown animal"),
        }
    }

    let dog = return_animal("dog");
    assert_eq!(
        type_of(&dog),
        "&dyn test_traits::trait_objects_dynamic::Animal"
    );

    let cat = return_animal("cat");
    assert_eq!(
        type_of(&cat),
        "&dyn test_traits::trait_objects_dynamic::Animal"
    );

    // TODO question: when do I have to use Box<dyn Trait> and when &dyn ?
    fn return_shape(is_rectangle: bool) -> Box<dyn Shape> {
        if is_rectangle {
            Box::new(Rectangle {
                width: 7,
                height: 6,
            })
        } else {
            Box::new(Triangle { base: 4, height: 3 })
        }
    }

    // the dynamic dispatch is used here
    let rectangle = return_shape(true);
    assert_eq!(rectangle.area(), 42);
}

#[test]
fn trait_bounds() {
    // similar to the previous example, but using trait bounds and more verbose
    fn get_area_formated<T: Shape>(s: T) -> String {
        format!("The area is {}", s.area())
    }

    let rectangle = Rectangle {
        width: 7,
        height: 6,
    };
    let area = get_area_formated(rectangle);
    assert_eq!(area, "The area is 42");

    let triangle = Triangle { base: 4, height: 3 };
    let area = get_area_formated(triangle);
    assert_eq!(area, "The area is 6");

    // USE CASE: use trait bounds if you have lots of parameters to avoid my_fn(s: &impl Shape, other: &impl Shape, ...)
    fn get_areas_formated<T: Shape>(s: &T, other: &T) -> String {
        format!("The areas are {} {}", s.area(), other.area())
    }
    let triangle = Triangle { base: 4, height: 3 };
    let result = get_areas_formated(&triangle, &triangle);
    assert_eq!(result, "The areas are 6 6");
}

#[test]
fn trait_bounds_where_clause() {
    // heavy uses of trait bounds
    fn some_function<T: Display + Clone>(t: T) -> String {
        let s = t.to_string();
        let c = t.clone();

        format!("string: {}, clone: {}", s, c)
    }
    let s = some_function("omar");
    assert_eq!(s, "string: omar, clone: omar");

    // using where clauses to make the function signature more readable
    fn some_function_with_where_clauses<T>(t: T) -> String
    where
        T: Display + Clone,
    {
        let s = t.to_string();
        let c = t.clone();

        format!("string: {}, clone: {}", s, c)
    }

    let s = some_function_with_where_clauses("omar");
    assert_eq!(s, "string: omar, clone: omar");
}

#[test]
fn trait_automatic_impl_derive() {
    // the compiler is capable of providing basic implementations for some traits via the #[derive] attribute
    #[derive(Debug, PartialEq, PartialOrd)]
    struct Seconds(i32);

    let second = Seconds(4);
    // trait Debug allows to print the struct using :?
    let s = format!("{:?}", second);
    assert_eq!(s, "Seconds(4)");

    // trait PartialEq allows to compare the struct using ==
    assert!(second == Seconds(4));

    // trait PartialOrd allows to compare the struct using <, >, <=, >=
    assert!(second < Seconds(5));

    // compiler automatically implements these traits for Centimeters
    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);

    #[derive(Debug)]
    struct Inches(i32);

    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            // destructuring self and get the value not the reference
            let &Inches(inches) = self;
            Centimeters(inches as f64 * 2.54)
        }
    }

    let foot = Inches(12);
    let meter = Centimeters(100.0);
    let cmp = if foot.to_centimeters() < meter {
        "foot is smaller than meter"
    } else {
        "foot is bigger than meter"
    };
    assert_eq!(cmp, "foot is smaller than meter");
}

#[test]
fn trait_operator() {
    // in rust many operators can be overloaded via traits

    // T has to implement the Mul trait
    // it uses associated types, which are types that are associated with a trait
    fn multiply<T: std::ops::Mul<Output = T>>(a: T, b: T) -> T {
        a * b // a.mul(b) also works, since a * b is a syntactic sugar for a.mul(b)
    }

    let r = multiply(2, 3);
    assert_eq!(r, 6);

    let r = multiply(2.0, 3.0);
    assert_eq!(r, 6.0);

    struct Foo;
    struct Bar;

    #[derive(PartialEq, Debug)]
    struct BarFoo;
    // overload the - operator
    impl std::ops::Sub<Bar> for Foo {
        type Output = BarFoo;

        fn sub(self, _rhs: Bar) -> BarFoo {
            BarFoo
        }
    }
    assert_eq!(Foo - Bar, BarFoo);

    #[derive(PartialEq, Debug)]
    struct FooBar;
    // overload the + operator
    impl std::ops::Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, _rhs: Bar) -> FooBar {
            FooBar
        }
    }
    assert_eq!(Foo + Bar, FooBar);
}

#[test]
fn trait_example_generic_implemeting_traits_from_lib() {
    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: std::fmt::Debug + PartialOrd + PartialEq> Pair<T> {
        fn get_message(&self) -> String {
            if self.x > self.y {
                format!("{:?} is bigger than {:?}", self.x, self.y)
            } else if self.x < self.y {
                format!("{:?} is smaller than {:?}", self.x, self.y)
            } else {
                format!("{:?} is equal to {:?}", self.x, self.y)
            }
        }
    }

    #[derive(Debug, PartialEq, PartialOrd)]
    struct Unit(i32);

    let pair = Pair::new(Unit(4), Unit(3));
    let message = pair.get_message();
    assert_eq!(message, "Unit(4) is bigger than Unit(3)");
}
