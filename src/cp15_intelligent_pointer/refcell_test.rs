use std::cell::RefCell;
use std::rc::Rc;

// 链表，其中每个列表的所有权可以有多个，且可以进行更改
#[derive(Debug)]
pub enum ListV3 {
    Cons(Rc<RefCell<i32>>, Rc<ListV3>),
    Nil,
}

#[cfg(test)]
mod test {
    use super::{LimitTracker, MockMessengerRefCell};
    use std::rc::Rc;
    use std::cell::RefCell;
    use super::ListV3::{Cons, Nil};

    // 这部分代码无法编译通过，因为set_value表示tracker是一个mut，但是send方法需要入参是一个不可变引用
    #[test]
    // fn it_sends_an_over_75_percent_warning_message_no_works() {
    //     let messenger = MockMessenger::new();
    //     let mut tracker = LimitTracker::new(&messenger, 100);
    //     tracker.set_value(80);
    //     assert_eq!(messenger.send_messages.len(), 1);
    // }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let messenger = MockMessengerRefCell::new();
        let mut tracker = LimitTracker::new(&messenger, 100);
        tracker.set_value(80);
        assert_eq!(messenger.send_messages.borrow().len(), 1);
    }

    #[test]
    fn list_v3_rc_refcell() {
        let value = Rc::new(RefCell::new(5));
        // 此处需要传入Rc::clone(&value)，如果直接传入value，则value无法在后续进行使用
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10; // value.borrow_mut()返回的是RefMut，需要通过*进行解引用，此处
        println!("a={:?}", a);
        println!("b={:?}", b);
        println!("c={:?}", c);
    }

}

pub trait Messenger {
    fn send(&self, message: &str);
}

struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}
impl<'a, T> LimitTracker<'a, T>
where T: Messenger
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let ratio = value as f64 / self.max as f64;
        if ratio >= 1.0 {
             self.messenger.send("Error: You are over your quota!");
        } else if ratio > 0.9 {
            self.messenger.send("Error: You are used up over 90% of your quota!");
        } else if ratio > 0.75 {
            self.messenger.send("Error: You are used up over 75% of your quota!");
        }
    }
}

// 用于测试的Messenger
// struct MockMessenger {
//     send_messages: Vec<String>,
// }
// impl MockMessenger {
//     pub fn new() -> MockMessenger {
//         MockMessenger {
//             send_messages: vec![]
//         }
//     }
// }
// impl Messenger for MockMessenger {
//     fn send(&mut self, message: &str) {
//         self.send_messages.push(String::from(message));
//     }
// }

struct MockMessengerRefCell {
    send_messages: RefCell<Vec<String>>,
}
impl MockMessengerRefCell {
    pub fn new() -> MockMessengerRefCell {
        MockMessengerRefCell {
            send_messages: RefCell::new(vec![]),
        }
    }
}
impl Messenger for MockMessengerRefCell {
    fn send(&self, message: &str) {
        self.send_messages.borrow_mut().push(String::from(message))

        // 测试拥有多个可变引用时，触发panic
        // let mut one_borrow =  self.send_messages.borrow_mut();
        // let mut two_borrow = self.send_messages.borrow_mut();
        // one_borrow.push(String::from(message));
        // two_borrow.push(String::from(message));
    }
}