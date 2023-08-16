use std::fmt::{Display, Formatter};
use std::io::{Write, Result as IOResult};
use std::net::TcpStream;
use super::status_codes::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self {
            status_code,
            body,
        }
    }

    pub fn send(&self, stream: &mut impl Write) -> IOResult<()> {
        let body = match &self.body {
            None => "",
            Some(b) => b
        };

        write!(stream,
               "HTTP/1.1 {} {}\r\n\r\n{}",
               self.status_code,
               self.status_code.reason_phrase(),
               body
        )
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let body = match &self.body {
            None => { "" }
            Some(b) => { b }
        };

        write!(
            f,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}