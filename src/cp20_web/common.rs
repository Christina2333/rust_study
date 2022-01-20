use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    // 此处sender视为一个队列，用于接收要执行的任务
    sender: mpsc::Sender<Message>,
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
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 第一个循环用于向worker中发送停止的通知
        println!("Sending terminate message to all workers");
        for _ in &mut self.workers {
            // 发送停止通知，可以终止worker中的死循环
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers");
        // 第二个循环用于停掉每个worker线程
        for worker in &mut self.workers {
            if let Some(t) = worker.thread.take() {
                // join方法无法通过&调用，需要消耗掉所有权
                println!("Shutting down worker {}", worker.id);
                t.join().unwrap();
            };
        }
    }
}

struct Worker {
    id: usize,
    // 在rust中，如果一个字段可以为空，一定需要Option
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        // move会使闭包获取变量所有权
        let thread = thread::spawn(move || {
            // 死循环，每个Worker一直循环等待任务，为了在停机时可以退出循环，需要传递Message
            loop {
                // lock获取互斥锁，recv获取放入send的待处理任务
                let msg = receiver.lock().unwrap().recv().unwrap();
                match msg {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job executing.", id);
                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate", id);
                        break
                    }
                }

            }
        });
        Worker {
            id,
            thread: Some(thread)
        }
    }
}
enum Message {
    NewJob(Job),
    Terminate
}