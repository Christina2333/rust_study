#[cfg(test)]
mod test {
    use std::net::TcpListener;

    #[test]
    fn tcp_bind() {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection established");
        }
    }
}