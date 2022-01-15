use std::rc::Rc;

#[cfg(test)]
mod test {
    use super::*;
    use super::ListV2::{Cons, Nil};
    use crate::cp15_intelligent_pointer::box_test::List;

    // 两个列表共享第三个列表的部分数据
    #[test]
    fn rc_test() {
        let a = List::Cons(5, Box::new(
            List::Cons(10, Box::new(
                List::Nil
            ))
        ));
        // 此时列表b持有了列表a的所有权
        let b = List::Cons(3, Box::new(a));
        // let c = List::Cons(4, Box::new(a));
    }

    #[test]
    fn rc_local() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        // 此处的clone只会增加引用计数，而不会进行数据拷贝，虽然此处和a.clone的效果相同
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
}

#[derive(Debug)]
pub enum ListV2<> {
    Cons(i32, Rc<ListV2>),
    Nil
}