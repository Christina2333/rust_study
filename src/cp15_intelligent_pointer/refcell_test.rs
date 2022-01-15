use std::cell::RefCell;

#[cfg(test)]
mod test {
    use super::{LimitTracker, MockMessengerRefCell};

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
    }
}