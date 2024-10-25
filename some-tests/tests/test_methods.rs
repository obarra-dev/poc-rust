struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(width: i32, height: i32) -> Rectangle {
        Rectangle { width, height }
    }

    fn new_q(value: i32) -> Self {
        Rectangle {
            width: value,
            height: value,
        }
    }

    fn new_q_1(value: i32) -> Self {
        Self {
            width: value,
            height: value,
        }
    }

    // it uses sintatic sugar
    // self is a reference to the instance of the struct (borrowing), it can be mutable
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn area_without_sintaticsugar(self: &Self) -> i32 {
        self.width * self.height
    }

    // self will take the ownership of current struct instance,
    // however, &self will only borrow a reference from instance
    fn set_width(&mut self, width: i32) {
        self.width = width;
    }
}

// the structs are allowed to have multiple impl blocks
// it is for organization purposes
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[test]
fn method_struct() {
    let rectangle = Rectangle {
        width: 7,
        height: 6,
    };
    let area = rectangle.area();
    assert_eq!(area, 42);
    let area = rectangle.area_without_sintaticsugar();
    assert_eq!(area, 42);

    // rectangle.set_width(4);

    let other = Rectangle {
        width: 4,
        height: 3,
    };
    let r = rectangle.can_hold(&other);
    assert!(r);
}

#[test]
fn method_struct_mutable() {
    let mut rectangle = Rectangle {
        width: 7,
        height: 6,
    };

    rectangle.set_width(4);

    let area = rectangle.area();
    assert_eq!(area, 24);
}

#[test]
fn assoicated_function() {
    // using the assoicated function
    let rectangle = Rectangle::new(7, 6);
    let area = rectangle.area();
    assert_eq!(area, 42);

    let rectangle = Rectangle::new_q(6);
    let area = rectangle.area();
    assert_eq!(area, 36);

    let rectangle = Rectangle::new_q_1(6);
    let area = rectangle.area();
    assert_eq!(area, 36);
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn color(&self) -> &str {
        match self {
            TrafficLight::Red => "red",
            TrafficLight::Yellow => "yellow",
            TrafficLight::Green => "green",
        }
    }
}

#[test]
fn method_enum() {
    let red = TrafficLight::Red;
    let color = red.color();
    assert_eq!(color, "red");
}
