use std::fmt::Display;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let pair = Pair::new(10, 45);
        pair.cmp_display();
    }
}

pub struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x={}", self.x);
        } else {
            println!("The largest number is y={}", self.y);
        }
    }
}