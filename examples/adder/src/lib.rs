#[derive(Debug)]
pub struct Rectangle {
    x: i32,
    y: i32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.x > other.x && self.y > other.y
    }
}

pub fn add_two(target: i32) -> i32 {
    target + 2
}

// -----------------------------------------------
#[derive(Debug)]
struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Didnt expect a value under 1 or over 100! Actual value: {}",
                   value)
        }

        Guess { value: value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { x: 10, y: 10 };
        let smaller = Rectangle { x: 5, y: 5 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let smaller = Rectangle { x: 10, y: 10 };
        let larger = Rectangle { x: 5, y: 5 };

        assert!(!larger.can_hold(&smaller));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[should_panic(expected = "Didnt expect a value under 1 or over 100! Actual value")]
    fn guess_greater_than_100() {
        Guess::new(101);
    }
}
