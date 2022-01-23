#[cfg(test)]
mod test {
    use super::List::{Cons, Nil};

    #[test]
    fn create_box() {
        let b = Box::new(5);
        println!("b {}", b);
        // 离开作用域时，b释放了执行5的指针，同时释放了5所占用的内存
    }

    #[test]
    fn create_list() {
        // let list = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));
        let list = Cons(1, Box::new(
            Cons(2, Box::new(
                Cons(3, Box::new(Nil))
            ))
        ));
        println!("list={:?}", list);
    }
}

// 链表定义
// #[derive(Debug)]
// enum List {
//     Cons(i32, List), // 其中i32为元素，List为i32后面的元素
//     Nil // 表示该元素为空
// }

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil
}
