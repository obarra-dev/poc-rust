use std::fmt::format;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value == 0 {
            panic!("Guess value must not be 0, got {}", value)
        } else if value < 1 {
            panic!("Guess value must be greater than 1")
        } else if value > 100 {
            panic!("Guess value must be less than 100")
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn greeting_test() {
        let actual = greeting("Omar");
        assert_eq!("Hello Omar!", actual);
    }

    #[test]
    fn greeting_contains_name() {
        let actual = greeting("Barra");
        assert!(
            actual.contains("Barra"),
            "It does not contain name, value was {}",
            actual
        )
    }

    #[test]
    #[should_panic]
    fn it_is_0() {
        Guess::new(0);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than 100")]
    fn greater_than_100() {
        Guess::new(400);
    }

    // test return a result type
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 3 == 4 {
            Ok(())
        } else {
            Err(String::from("Some silly error"))
        }
    }
}
