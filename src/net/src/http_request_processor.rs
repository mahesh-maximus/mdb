use std::fs;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{prelude::*, BufReader};
use crate::login_processor::LoginProcessor;

use std::fs::File;
use std::io::LineWriter;

pub struct HttpRequestProcessor {
    address: String,
}

struct Request {
    headers: Vec<String>,
    body: String,
    test: String,
}

impl HttpRequestProcessor {
    pub fn new(address: String) -> Self {
        Self {
            address,
        }
    }

    pub fn print_address(&mut self) {
        println!("Tcp address for Http requests: {}", self.address); 
    } 

    pub fn process_http_requests(&mut self) {
        println!("HTTP Request Processor <<>>");
        
        let tcp_address = self.address.to_string();

        thread::spawn(|| {
            let listener = TcpListener::bind(tcp_address).unwrap();

            for stream in listener.incoming() {
                thread::spawn(||{
                    let stream = stream.unwrap();
                 Self::handle_connection(stream);
                });
            }
        });
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream); 

        let headers: Vec<_>  = buf_reader.lines()
            .map(|r| r.unwrap())
            .take_while(|x| !x.is_empty())
            .collect();
        

        let x_body_index = headers.iter().position(|r| r.starts_with("X-Body: ")).unwrap_or(0);
        let mut x_body = String::new();

        if x_body_index > 0 {
            x_body = headers[x_body_index].to_string();    
        }
        
        let request = Request {
            headers,
            test: "test".to_string(),
            body: x_body,
        };

        println!("request_headers Vec len: {}", request.headers.len());

        for request_item in &request.headers {
            println!("Request item: {request_item}");
        }      

        println!("First request item HTTP method: {}", request.headers[0]);

        println!("Request: {:#?}", request.headers);
      
        println!("Request Body : {}", request.body);

        let mut login_processor  = LoginProcessor::new("mahesh".to_string(), "123".to_string());
        login_processor.validate_username_password();
        
        let cookie_index =  request.headers.iter().position(|r| r.starts_with("Cookie: ")).unwrap_or(0);
        
        println!("Cockie index: {}", cookie_index.to_string());

        let should_load_login_page = true;
        let mut response = String::new();

        if cookie_index > 0 {
            println!("Cookies aviable: {}", request.headers[cookie_index]);
        }

        if should_load_login_page {
            response = Self::create_response("HTTP/1.1 200 OK".to_string(), "login.html".to_string());    
        } else if request.headers[0] == "GET / HTTP/1.1" {
            response = Self::create_response("HTTP/1.1 200 OK".to_string(), "index.html".to_string());  
        } else {
            response = Self::create_response("HTTP/1.1 200 OK".to_string(), "404.html".to_string());  
        }

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn create_response(status_line: String, filename: String) -> String { 
        let file_name = format!("{}{}", "/mdb/frontend/", filename);
        println!("Response filename : {}", file_name);

        let contents = fs::read_to_string(file_name).unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        response 
    }

    #[allow(dead_code)]
    fn extract_request(mut stream: TcpStream) -> Request {
        let buf_reader = BufReader::new(&mut stream); 

        let headers: Vec<_>  = buf_reader.lines()
            .map(|r| r.unwrap())
            .take_while(|x| !x.is_empty())
            .collect();
        

        let x_body_index = headers.iter().position(|r| r.starts_with("X-Body: ")).unwrap_or(0);
        let mut x_body = String::new();

        if x_body_index > 0 {
            x_body = headers[x_body_index].to_string();    
        }
        
        Request {
            headers,
            test: "test".to_string(),
            body: x_body,
        }
    }
}
