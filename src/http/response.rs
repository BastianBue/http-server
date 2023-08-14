use super::status_codes::StatusCode;

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(statusCode: StatusCode, body: Option<String>) -> Self{
       !unimplemented!()
    }
}