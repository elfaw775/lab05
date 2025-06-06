
## 个人信息
- 班级：2314班
- 姓名：胡皓冬
- 学号：20232241405

## 1. 实验目的
实现并对比 Rust 单线程、多线程、Tokio 异步I/O 三种Web服务器的基本性能。

## 2. 相关知识概述
### Web服务器工作原理简述

Web服务器的主要功能是**监听客户端（如浏览器）发送的HTTP请求，处理请求后返回响应数据（如网页、图片等）**。其工作流程一般如下：

1. **启动服务器**：服务器进程绑定到指定IP和端口，持续监听网络连接。
2. **等待连接**：客户端（如浏览器）发起HTTP请求，服务器接受连接。
3. **解析请求**：服务器接收并解析HTTP请求内容。
4. **生成响应**：服务器根据请求内容（比如请求的资源路径）生成对应的HTTP响应。
5. **发送响应**：服务器将响应内容通过网络返回给客户端。
6. **关闭连接或等待下一个请求**。

常见的Web服务器还有处理静态资源、支持并发、处理错误等功能。


### Rust中的线程与异步基本方法

#### 1. 线程

Rust通过标准库 `std::thread` 支持多线程：

- **创建线程**：使用 `thread::spawn` 创建新线程，线程里可以执行闭包代码。
- **线程同步**：可以用 `join()` 等待子线程完成。
- **数据共享**：多线程共享数据时，需用 `Arc`（原子引用计数）和 `Mutex`（互斥锁）来保证线程安全。
- **线程池**：用线程池（如 `threadpool` crate）可以让固定数量的线程反复处理任务，提高性能并降低系统压力。
#### 2. 异步

Rust 的异步编程依赖于 async/await 语法和异步运行时（如 Tokio）：

- **异步函数**：使用 `async fn` 定义。
- **.await**：等待一个异步操作完成。
- **Tokio等运行时**：提供任务调度、异步IO和高性能网络支持。


## 3. 实现过程

### 3.1 单线程Web服务器
- 代码
```rust
use std::net::TcpListener;

use std::io::{Read, Write};

  

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("单线程Web服务器启动，监听127.0.0.1:7878");

  

    for stream in listener.incoming() {

        let mut stream = stream.unwrap();

        let mut buffer = [0; 512];

        stream.read(&mut buffer).unwrap();

  

        let contents = "<!DOCTYPE html>

<html>

<head><title>Rust Web Server</title></head>

<body>

    <h1>连接成功！</h1>

    <p>你已经成功访问了Rust单线程Web服务器。</p>

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

}
```

![[Pasted image 20250606194329.png]]
![[Pasted image 20250606194316.png]]


### 3.2 多线程Web服务器
- 代码
```rust
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
```

![[Pasted image 20250606194152.png]]
![[Pasted image 20250606194143.png]]
### 3.3 基于Tokio的异步Web服务器
- 代码
```rust
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
```

- ![[Pasted image 20250606193702.png]]
- ![[Pasted image 20250606193652.png]]

## 4. 性能对比
- 对三种方式分别进行简单并发测试（如ab、wrk等工具），记录响应时间、QPS等。

| 方案        | 响应时间 | QPS   | 备注   |
| ----------- | ------- | ----- | ------ |
| 单线程      |         |       |        |
| 多线程      |         |       |        |
| Tokio异步   |         |       |        |

## 5. 实验总结
- 三种实现的优缺点
- 性能结果分析
- 学习体会



