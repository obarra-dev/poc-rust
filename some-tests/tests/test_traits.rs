trait Shape {
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


    let triangle = Triangle {
        base: 4,
        height: 3,
    };
    let area = triangle.area();
    assert_eq!(area, 6);
}
