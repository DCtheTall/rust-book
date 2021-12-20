use std::fmt::Debug;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(i: i32) -> i32 { i + 2 }

pub fn add_greeting(name: &str) -> String { format!("Hello, {}", name) }

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be less than or equal to 100");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // Run by calling `cargo test -- --ignored`, will fail
    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

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
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn test_add_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_greeting_contains_name() {
        let result = add_greeting(&String::from("Carol"));
        // Assert can have custom error messages
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, was {}", result);
    }

    // Test that code should crash
    #[test]
    #[should_panic]
    fn greater_than_180() {
        Guess::new(200);
    }

    // Test it should panic with a specific error
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_180_msg() {
        Guess::new(200);
    }

    // Test functions can also return Result
    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
