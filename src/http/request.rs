use std::fmt::{Debug, Display, Formatter};
use std::error::Error as CustomError;
use std::convert::TryFrom;
use std::str::Utf8Error;
use crate::http::method::MethodParsingError;
use crate::http::QueryString;
use super::method::Method;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = HttpParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = std::str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(HttpParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(HttpParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(HttpParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(HttpParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[(i + 1)..]));
        }
    }
    None
}

pub enum HttpParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl HttpParseError {
    fn message(&self) -> &str {
        match self {
            HttpParseError::InvalidRequest => "InvalidRequest",
            HttpParseError::InvalidEncoding => "Invalid Encoding",
            HttpParseError::InvalidProtocol => "Invalid Protocol",
            HttpParseError::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodParsingError> for HttpParseError {
    fn from(_: MethodParsingError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for HttpParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for HttpParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Debug for HttpParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl CustomError for HttpParseError {}