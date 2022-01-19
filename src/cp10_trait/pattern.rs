#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn struct_pattern() {
        let integer = Point::new(1, 3);
        let float = Point::new(1.9, 3.4);
        println!("float.x={}", float.x());
        println!("float.distance={}", float.distance_from_origin());

        let p1 = Point2::new(1, "c");
        let p2 = Point2::new("a", 3.9);
        let p3 = p1.mixup(p2);
        println!("p3.x={}, p3.y={}", p3.x(), p3.y());
    }

    #[test]
    fn pattern_find_largest() {
        let list = [3, 6, 1, 10];
        // 使用带范型的函数比较大小
        let max = find_largest(&list);
        println!("max:{}", max);
        let list = vec![34, 50, 25, 100, 65];
        println!("max:{}", find_largest(&list));
        println!("max:{}", find_largest_simple(&list));
    }
}

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

// 函数中加入范型，
//(1)符号"<"用于比较大小，这一运算符被定义为std::cmp::PartialOrd中，因此范型T需要属于该类型
//(2)如果使用list[0]方式取出数据，需要实现Copy
pub fn find_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &i in list.iter() {
        if max < i {
            max = i;
        }
    }
    return max;
}
// 如果返回值为&T，入参就可以不用实现Copy
pub fn find_largest_simple<T>(list: &[T]) -> &T
where T: PartialOrd {
    let mut max = &(list[0]);
    for i in list.iter() {
        if *max < *i {
            max = &i;
        }
    }
    return max;
}