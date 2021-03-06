use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn node_test() {
        let leaf = Rc::new(Node {
            _value: 3,
            _children: RefCell::new(vec![]),
            _parent: RefCell::new(Weak::new()),
        });
        println!("leaf = {:?}", leaf);
        println!("leaf parent = {:?}", leaf._parent.borrow().upgrade());
        println!("leaf strong ={}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
        {
            let branch = Rc::new(Node {
                _value: 5,
                _children: RefCell::new(vec![Rc::clone(&leaf)]),
                _parent: RefCell::new(Weak::new()),
            });
            // leaf的parent指向branch，即为指向branch的弱引用
            *(leaf._parent.borrow_mut()) = Rc::downgrade(&branch);
            println!("leaf parent = {:?}", leaf._parent.borrow().upgrade());
            // 此时打印leaf的parent时，不会造成栈溢出，parent会标注为weak
            println!("leaf = {:?}", leaf);

            println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
            println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        }
        println!("作用域结束");
        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }
}

// 带有子节点的Node
#[derive(Debug)]
struct Node {
    _value: i32,
    // RefCell表示子树可变，由于子树可能有多个，所以用Vec表示。而每个节点可能是多个树的子节点，因为Vec<T>的T为Rc<Node>
    _children: RefCell<Vec<Rc<Node>>>,
    // 父节点，Node拥有parent的弱引用，因为parent被销毁时，children需要被一起销毁
    _parent: RefCell<Weak<Node>>,
}