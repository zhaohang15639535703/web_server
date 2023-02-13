#![allow(unused)]
use std::{
    fmt::format,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server::ThreadPool;
fn main() {
    //在本地监听本地7878端口的tcp连接
    let listener = TcpListener::bind("localhost:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // println!("Connection established")
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let mut lines = buf_reader.lines();
    let first_line_result = lines.next().unwrap();

    //print request
    let http_request: Vec<_> = lines
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    // println!("Request:{:#?}", http_request);

    let request_line = first_line_result.unwrap();

    let (status_line, filname) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./html/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "./html/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "./html/404.html"),
    };
    let contents = fs::read_to_string(filname).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
