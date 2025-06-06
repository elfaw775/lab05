use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    println!("Tokio异步Web服务器启动，监听127.0.0.1:7878");

    let contents = "<!DOCTYPE html>
<html>
<head><title>Rust Web Server</title></head>
<body>
    <h1>连接成功！</h1>
    <p>你已经成功访问了RustWeb异步服务器。</p>
</body>
</html>";


    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buffer = [0u8; 512];
            socket.read(&mut buffer).await.unwrap();
            let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        
        socket.write_all(response.as_bytes()).await.unwrap();
            socket.flush().await.unwrap();
        });
    }
}
