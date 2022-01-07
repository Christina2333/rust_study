// 结构体中的范型
pub struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn new(x: T, y: T) -> Point<T> {
        Point {
            x,
            y
        }
    }
}

// 仅为Point<T>中某一类型的函数
impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    // self的范型不一定与入参或者返回值的范型一致，该方法中的范型V,W则和impl声明的范型无关，只和该方法有关
    pub fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y
        }
    }

    pub fn new(x: T, y: U) -> Point2<T, U> {
        Point2 {
            x,
            y
        }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &U {
        &self.y
    }
}