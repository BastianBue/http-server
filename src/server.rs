use std::io::{Read, Write};
use std::convert::TryFrom;
use crate::http::{Request, StatusCode};
use std::net::{TcpListener };
use crate::http::response::Response;

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
    pub fn run(self) {
        println!("Listening on port {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("OK");
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    let response = Response::new(StatusCode::Ok, Some("<h1>It works!</h1>".to_string()));
                                    write!(&stream,"{}", response).expect("TODO: panic message");
                                    dbg!(response);
                                }
                                Err(e) => {
                                    println!("Failed to parse a request: {}", e)
                                }
                            }
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
