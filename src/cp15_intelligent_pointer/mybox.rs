use std::ops::Deref;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quote() {
        let x = 5;
        let boxy = MyBox::new(5);
        assert_eq!(x, 5);
        assert_eq!(*boxy, 5); // 实现了Deref之后，才不会报错
        assert_eq!(*(boxy.deref()), 5); // 实现了Deref之后，上述语句会被执行为这个语句
    }

    #[test]
    fn hello_mybox() {
        let s = String::from("zwy");
        hello(&s);

        let mybox = MyBox::new(s);
        hello(&mybox);
        hello(mybox.deref()); // &mybox通过mybox.deref()实现
        hello(&((*mybox)[..])); //如果不存在隐式转换，上述过程会是这样
    }

}

// 自定义Box，拥有T类型元素的元组结构体，MyBox可以理解为指向T的指针
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    // 貌似是获取&Mybox，此处即为存储T的地址
    fn deref(&self) -> &Self::Target {
        &(self.0)
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}