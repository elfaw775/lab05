use std::net::TcpListener;
use std::io::{Read, Write};
use std::thread;

fn handle_connection(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let contents = "<!DOCTYPE html>
<html>
<head><title>Rust Web Server</title></head>
<body>
    <h1>连接成功！</h1>
    <p>你已经成功访问了Rust多线程Web服务器。</p>
</body>
</html>";

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("多线程Web服务器启动，监听127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
