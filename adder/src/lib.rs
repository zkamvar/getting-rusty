pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Greetings, {}!", name)
}

#[derive(Debug)]
pub struct Rectangle {
    width: i32,
    height: i32,
}

pub fn get_rekt(width: i32, height: i32) -> Rectangle {
    if width <= 0 || height <= 0 {
        panic!("Negative or zero widths: start panicking");
    }
    Rectangle {
        width: width,
        height: height,
    }
}
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

#[cfg(test)]
mod tests {
    // allows for testing of private functions
    use super::*;
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Morpheus");
        // custom assertion messages
        assert!(
            result.contains("Morpheus"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
    #[test]
    // custom panics
    #[should_panic(expected = "start panicking")]
    fn lets_fail_rekt() {
        get_rekt(-6, 6);
    }
    #[test]
    fn lets_get_rekt() {
        let rect = get_rekt(6, 6);
        assert!(rect.is_square());
    }
    #[test]
    fn big_box_hold_little_box() {
        let larger = Rectangle {
            width: 6,
            height: 9,
        };
        let smol = Rectangle {
            width: 3,
            height: 3,
        };
        assert!(larger.can_hold(&smol));
        assert!(!smol.can_hold(&larger));
        assert!(!smol.can_hold(&smol));
        assert!(!larger.can_hold(&larger));
    }
    #[test]
    fn lets_be_l7() {
        let rekt = Rectangle {
            width: 6,
            height: 9,
        };
        let l7 = Rectangle {
            width: 3,
            height: 3,
        };
        assert!(l7.is_square());
        assert!(!rekt.is_square());
    }

    #[test]
    fn addition_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
