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

// instead of trait, we use enum to represent different types
enum Thing {
    Rectangle(Rectangle),
    Triangle(Triangle),
}

struct Holder {
    things: Vec<Thing>,
}

impl Holder {
    fn new() -> Self {
        Holder { things: Vec::new() }
    }

    fn add(&mut self, thing: Thing) {
        self.things.push(thing);
    }

    fn total_area(&self) -> i32 {
        let mut total = 0;
        for thing in &self.things {
            total += match thing {
                Thing::Rectangle(r) => r.area(),
                Thing::Triangle(t) => t.area(),
            };
        }
        total
    }
}

#[test]
fn trait_test() {
    let rectangle = Rectangle {
        width: 7,
        height: 6,
    };

    let triangle = Triangle { base: 4, height: 3 };

    let mut holder = Holder::new();
    holder.add(Thing::Rectangle(rectangle));
    holder.add(Thing::Triangle(triangle));
    let total_area = holder.total_area();
    assert_eq!(total_area, 48);
}
