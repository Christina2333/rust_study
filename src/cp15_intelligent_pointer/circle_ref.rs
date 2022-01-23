// 循环引用

use std::cell::RefCell;
use std::rc::Rc;
use super::circle_ref::ListV4::{Cons, Nil};

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use super::ListV4::{Cons, Nil};
    use std::cell::RefCell;

    #[test]
    fn circle_ref() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
        println!("a initial rc count {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            // 通过borrow_mut，把a.tail指向了b
            *(link.borrow_mut()) = Rc::clone(&b);
        }

        // if let的替换
        // let link;
        // match a.tail() {
        //     Some(item) => {
        //         link = item;
        //         // 通过borrow_mut，把a.tail指向了b
        //         *(link.borrow_mut()) = Rc::clone(&b);
        //     },
        //     None => {},
        // }
        println!("b rc count after changing b = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // 此时打印a.tail会发生栈溢出情形
        // println!("a next item = {:?}", a.tail());

    }
}

#[derive(Debug)]
enum ListV4 {
    Cons(i32, RefCell<Rc<ListV4>>), // Rc表示可以保持多个引用，RefCell表示支持内部可变性，可以修改tail节点
    Nil,
}
impl ListV4 {
    // 获取链表的tail节点
    fn tail(&self) -> Option<&RefCell<Rc<ListV4>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}