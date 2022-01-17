#[cfg(test)]
mod test {
    use std::sync::{Mutex, Arc};
    use std::thread;
    use std::rc::Rc;

    #[test]
    fn single_thread_mutex() {
        let m = Mutex::new(5);

        {
            // 使用unwrap()，获取不到锁时触发panic
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}", m);
    }

    #[test]
    // fn multi_thread_mutex_error() {
    //     let counter = Mutex::new(0);
    //     let mut handles = vec![];
    //     for _ in 0..10 {
    //         // 循环中创建10个线程，把counter传入闭包，因为传入后，主线程就不存在counter的所有权，因此无法通过编译。
    //         let handle = thread::spawn(move || {
    //            let mut num = counter.lock().unwrap();
    //             *num += 1;
    //         });
    //         handles.push(handle);
    //     }
    //     for handle in handles {
    //         handle.join().unwrap();
    //     }
    //     println!("Result : {}", counter.lock().unwrap());
    // }

    #[test]
    fn multi_thread_mutex() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
               let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result={}", counter.lock().unwrap());

    }
}