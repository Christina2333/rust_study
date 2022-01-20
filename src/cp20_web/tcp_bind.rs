use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;

#[cfg(test)]
mod test {

}

// 此时通过浏览器访问http://localhost:7878就可以看到打印效果
pub fn tcp_bind() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        connection_handle(stream);
    }
}

// 处理连接
fn connection_handle(mut stream: TcpStream) {
    // 则个buffer的大小很重要！！！从文中的512改成1024
    let mut buffer = [0; 1024];
    stream.read(&mut buffer);
    println!("Request {}", String::from_utf8_lossy(&buffer));

    let content = fs::read_to_string("hello.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);
    println!("Response: {}", &response);

    stream.write(response.as_bytes()).unwrap();
    // 会等待并阻止程序继续执行知道所有字节被写入连接
    stream.flush().unwrap();
}