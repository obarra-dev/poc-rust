use std::fmt::{format, Display};

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

    let area = rectangle.area();
    assert_eq!(area, 42);

    let r = rectangle.can_hold(&Rectangle {
        width: 4,
        height: 3,
    });
    assert!(r);

    let triangle = Triangle { base: 4, height: 3 };
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
    fn get_area_formated(s: impl Shape) -> String {
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
