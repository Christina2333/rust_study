use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::thread;
use std::time::Duration;

use super::common;

#[cfg(test)]
mod test {

}

// 此时通过浏览器访问http://localhost:7878就可以看到打印效果
pub fn tcp_bind() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = common::ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            connection_handle(stream);
        });
    }
    println!("shutting down");
}

// 处理连接
pub fn connection_handle(mut stream: TcpStream) {
    // 则个buffer的大小很重要！！！从文中的512改成1024
    let mut buffer = [0; 1024];

    // 前缀"b"表示为二进制的字符串，因为buffer是二进制数组
    let get = b"GET / HTTP/1.1\r\n";
    let slow = b"GET /sleep HTTP/1.1\r\n";
    stream.read(&mut buffer).unwrap();
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