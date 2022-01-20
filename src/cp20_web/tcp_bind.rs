use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::process::id;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;

type Job = Box<dyn FnOnce() + Send + 'static>;

#[cfg(test)]
mod test {

}

// 此时通过浏览器访问http://localhost:7878就可以看到打印效果
pub fn tcp_bind() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            connection_handle(stream);
        });
    }
}

// 处理连接
fn connection_handle(mut stream: TcpStream) {
    // 则个buffer的大小很重要！！！从文中的512改成1024
    let mut buffer = [0; 1024];

    // 前缀"b"表示为二进制的字符串，因为buffer是二进制数组
    let get = b"GET / HTTP/1.1\r\n";
    let slow = b"GET /sleep HTTP/1.1\r\n";
    stream.read(&mut buffer);
    // println!("Request {}", String::from_utf8_lossy(&buffer));

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(slow) {
        // 模拟一个慢请求
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let response = format!("{}{}", status_line, fs::read_to_string(filename).unwrap());
    stream.write(response.as_bytes()).unwrap();
    // 会等待并阻止程序继续执行知道所有字节被写入连接
    stream.flush().unwrap();

}

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