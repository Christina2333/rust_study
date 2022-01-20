use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    // 此处sender视为一个队列，用于接收要执行的任务
    sender: mpsc::Sender<Job>,
}
impl ThreadPool {
    /// 创建线程池
    /// #panics
    /// 当入参为0触发panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        // 因为Receiver是多生产者单消费者，多个线程想通过Receiver接收任务，需要考虑数据竞争，因此需要使用Mutex
        // 同时多个线程都需要持有Receiver的所有权，因此需要是用Arc
        let receiver= Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));

        }
        ThreadPool {
            workers,
            sender
        }
    }
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        // move会使闭包获取变量所有权
        let thread = thread::spawn(move || {
            // 死循环，每个Worker一直
            loop {
                // lock获取互斥锁，recv获取放入send的待处理任务
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing.", id);
                job();
            }
        });
        Worker {
            id,
            thread
        }
    }
}