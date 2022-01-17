use std::thread;

#[cfg(test)]
mod test {

    use std::thread;
    use std::time::Duration;
    // multiple producer, single consumer
    use std::sync::mpsc;

    #[test]
    fn test_spawn() {
        // 主线程结束，这部分代码就会终止打印
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn test_handle() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        // handle.join().unwrap(); // 放在这里会等spawned线程结束后再向下执行

        for i in 1..5 {
            println!("hi number {} from the main thread", i);
            thread::sleep(Duration::from_millis(1));
        }
        // 通过thread::spawn的返回值JoinHandle的join方法阻塞main线程，直到handle结束
        handle.join().unwrap();
    }

    // 测试move闭包
    #[test]
    fn move_closure() {
        let v = vec![1, 2, 3];
        // 闭包增加move才可以编译通过，表示闭包会获取v的所有权
        let handle = thread::spawn(move || {
            println!("Here's a vector {:?}", v);
        });
        handle.join().unwrap();
    }

    // 基于消息实现线程间通信
    #[test]
    fn test_msg() {
        let (tx, rx) = mpsc::channel();
        // 发送端线程，使用move获取tx的所有权
        thread::spawn(move || {
            let val = String::from("hi");
            // send方法返回Result<T,E>
            tx.send(val).unwrap();
            // println!("val is {}", val);  // 此处打印会报错，因为执行了send操作后，val的使用权就被转移到接收者
        });
        // 阻塞主线程。直到有值被传入通道，可以使用try_recv改为不会阻塞线程
        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    // 发送多条消息
    #[test]
    fn test_multi_msg() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
           let vals = vec![
               String::from("hi"),
               String::from("from"),
               String::from("the"),
               String::from("thread")
           ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        });

        // 将rx视作迭代器，不再显示调用recv函数，查看Receiver中Iterator的实现，next方法即是调用recv()
        for received in rx {
            println!("Got: {}", received);
        }
    }

    // 多个sender，一个receiver
    #[test]
    fn multi_sender() {
        let (tx, rx) = mpsc::channel();

        let tx1 = mpsc::Sender::clone(&tx);

        thread::spawn(move || {
           let vals = vec![
               String::from("hi"),
               String::from("from"),
               String::from("the"),
               String::from("thread")
           ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("message"),
                String::from("for"),
                String::from("you")
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        });

        for receive in rx {
            println!("Got: {}", receive);
        }
    }

}