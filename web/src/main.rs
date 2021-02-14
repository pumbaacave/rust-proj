extern crate threads_pool;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use threads_pool::ThreadPool;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listner.incoming() {
        let stm = stream.unwrap();
        pool.execute(|| {
            handle(stm);
        })
        .unwrap();
    }
}

fn handle(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    let content = "PLAIN TEXT";
    let resp = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );

    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
    //println!("Request: {}", String::from_utf8_lossy(&buf));
}
