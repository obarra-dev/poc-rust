// THIS IN EFFICIENT IS TERRIBLE, using Box to store in the HEAP
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

struct Triangle {
    base: i32,
    height: i32,
}

impl Shape for Triangle {
    fn area(&self) -> i32 {
        self.base * self.height / 2
    }
}

// attemp to create a more complex structure by composing other shapes
struct Holder<T: Shape> {
    things: Vec<T>,
}

impl<T: Shape> Holder<T> {
    fn new() -> Self {
        Holder { things: Vec::new() }
    }

    fn add(&mut self, thing: T) {
        self.things.push(thing);
    }

    fn total_area(&self) -> i32 {
        let mut total = 0;
        for thing in &self.things {
            total += thing.area();
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

    // TODO how can I use this approach?
    // let  holder = Holder::new();
}
