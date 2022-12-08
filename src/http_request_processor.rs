use std::thread::spawn;
use std::{
    fs,
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
    str, thread,
    time::Duration,
};

use std::io::{prelude::*, BufReader};

pub fn process_requests() {
    println!("HTTP Request Processor <<>>");

    thread::spawn(|| {
        let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            handle_connection(stream);
        }
    });
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    for request_item in &http_request {
        println!("Request item: {request_item}");
    }

    println!("First request item HTTP method: {}", http_request[0]);

    println!("Request: {:#?}", http_request);

    let (status_line, filename) = if http_request[0] == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else if http_request[0] == "GET /sleep HTTP/1.1" {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let baseFrontendPath = "/mdb/frontend/";
    let a = format!("{}{}", baseFrontendPath, filename);
    println!("Response filename : {}", a);

    let contents = fs::read_to_string(a).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
