pub fn add(left: usize, right: usize) -> usize {
    left + right
}
#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
