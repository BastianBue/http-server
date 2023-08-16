use std::io::{Read};
use std::convert::TryFrom;
use crate::http::{Request, StatusCode};
use std::net::{TcpListener};
use crate::http::request::HttpParseError;
use crate::http::response::Response;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &HttpParseError) -> Response;
}

pub struct Server {
    addr: String,
}

impl Server {
    // main constructor should always be new()
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    // self is the equivalent of this in java
    // self also indicates that the function requires instanciation
    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on port {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("OK");
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(&request);
                                    handler.handle_request(&request)
                                }
                                Err(e) => {
                                    println!("Failed to parse a request: {}", &e);
                                    handler.handle_bad_request(&e)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send Response: {}", e)
                            }
                            dbg!(response);
                        }
                        Err(e) => {
                            println!("Failed to read from connection: {}", e)
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e)
                }
            }
        }
    }
}
