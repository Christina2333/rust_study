use std::thread;
use std::time::Duration;

// 模拟算法接口延时2s
pub fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly");
    thread::sleep(Duration::from_secs(2));
    intensity
}

pub fn generate_workout(intensity: u32, random_num: u32) {
    // 存储了一个结构体，结构体包含一个闭包和闭包的结果
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_num == 3 {
            println!("Take a break today");
        } else {
            println!("Today, run for {} minutes", expensive_closure.value(intensity));
        }
    }
}

// 闭包会实现Fn/FnMut/FnOnce中的一个，下面是一个存储了闭包和闭包结果的结构体
pub struct Cacher<T, R>
    where T: Fn(R) -> R,
          R: PartialOrd + Copy,
{
    calculation: T, // 闭包
value: Option<R> // 闭包结果
}

impl<T, R> Cacher<T, R>
    where T: Fn(R) -> R,
          R: PartialOrd + Copy
{
    // Cacher初始化时value为None
    pub fn new(calculation: T) -> Cacher<T, R> {
        Cacher {
            calculation,
            value: None
        }
    }
    // 获取闭包结果时，先从value获取，如果value为None，则调用闭包获取结果，再缓存起来
    pub fn value(&mut self, arg: R) -> R {
        match self.value {
            Some(v) => v,
            None => {
                let value = (self.calculation)(arg);
                self.value = Some(value);
                value
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Cacher;
    use super::*;

    // 当闭包的参数不同时，返回的value值也是一样的
    #[test]
    fn test_cacher() {
        let mut cacher = Cacher::new(|x| {x});
        let c1 = cacher.value(5);
        let c2 = cacher.value(3);
        assert_eq!(c1, c2);
    }

    // 闭包不同于函数的特性
    #[test]
    fn closure_character() {
        // 闭包可以使用和自身同一作用域的变量，例如i
        let i = 9;
        let test_closure = |x| { x == i };
        let res = test_closure(7);
        assert_eq!(res, false);

        // 以下函数无法编译通过，但是函数中不可以使用和函数同一作用域的变量i
        // fn test_fn(t: i32) -> bool {
        //     t == i
        // }
    }

    #[test]
    fn move_closure() {
        let x = vec![1,2,3];
        let equal_to_x = move |z| { z == x };
        // println!("can not use x here: {:?}", x); // 由于使用move关键字，x的所有权移动到了闭包中，因此无法在闭包外使用x
        let y = vec![1,2,3];
        assert!(equal_to_x(y));
    }

    #[test]
    fn fn_closure() {
        let x = vec![1,2,3];
        let equal_to_x = |z| { z == x }; // 此处闭包使用的是x的不可变借用
        println!("can not use x here: {:?}", x); // 因此可以打印x
        let y = vec![1,2,3];
        assert!(equal_to_x(y));
    }

    // 返回闭包
    #[test]
    fn return_closure() {
        fn _returns_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }
    }

    #[test]
    fn generate_work_out() {
        let simulated_user_specified_value = 10;
        let simulated_random_number = 3;
        generate_workout(
            simulated_user_specified_value, simulated_random_number);
    }
}

