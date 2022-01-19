use std::ops::Add;
use std::fmt;
use std::fmt::Formatter;
use std::io;

#[cfg(test)]
mod test {
    use super::*;

    // 为范型指定默认类型
    #[test]
    fn default_type_for_pattern() {
        assert_eq!(Point {x: 1, y: 0} + Point {x : 2, y: 3}, Point {x: 3, y: 3});
    }

    // 把范型默认参数修改为自定义参数
    #[test]
    fn custom_type_for_pattern() {
        let millimeters = Millimeters(100).add(Meters(1));
        println!("millimeters = {}", millimeters.0);
    }

    // 使用全限定性名消除歧义
    #[test]
    fn same_method() {
        let person = Human {};
        // 默认调用Human中直接实现的fly()
        person.fly();
        // 显示调用trait上实现的fly
        Pilot::fly(&person);
        Wizard::fly(&person);
    }

    // trait中的同名方法调用，如果trait中不含self参数
    #[test]
    fn same_method_v2() {
        // 调用Dog上的baby_name
        println!("A baby dog is called a {}", Dog::baby_name());
        // 调用Dog实现的baby_name
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    }

    #[test]
    fn super_trait() {
        let p = Point {x: 1, y: 2};
        p.outline_print();
    }

    // 绕开孤儿原则，为Vec<String>实现Display，newtype模式
    #[test]
    fn orphan() {
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
}
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {

    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

// trait中的重名方法调用
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Up");
    }
}
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// trait中的重名方法调用(2)
trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// supertrait
// OutlinePrint依赖于Display
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
// 为了让Point可以实现OutlinePrint，需要让Point先实现Display
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl OutlinePrint for Point {}

// 绕开孤儿原则，为Vec<String>实现Display
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}