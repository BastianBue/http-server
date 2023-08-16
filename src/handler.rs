use std::fs;
use crate::http::{Request, StatusCode};
use crate::http::method::Method;
use crate::http::request::HttpParseError;
use crate::http::response::Response;
use super::server::Handler;

pub struct WebHandler {
    public_path: String,
}

impl WebHandler {
    pub fn new(public_path: String) -> Self {
        Self {
            public_path
        }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        if !fs::canonicalize(&path).ok()?.starts_with(&self.public_path) {
            println!("Directory traversal Attack attempted");
            return None;
        }

        match fs::read_to_string(&path) {
            Ok(string) => Some(string),
            Err(e) => {
                println!("Failed to read file at {}: {}", path, e);
                None
            }
        }
    }
}

impl Handler for WebHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        return match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, Some("Hello".to_string())),
                path => match self.read_file(path) {
                    None => Response::new(StatusCode::NotFound, None),
                    Some(content) => Response::new(StatusCode::Ok, Some(content))
                }
            },
            _ => Response::new(StatusCode::NotFound, None)
        };
    }

    fn handle_bad_request(&mut self, _: &HttpParseError) -> Response {
        Response::new(StatusCode::BadRequest, None)
    }
}