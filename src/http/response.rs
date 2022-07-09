use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    io::{Result as IoResult, Write},
    net::TcpStream,
};

use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub fn send(&self, f: &mut TcpStream) -> IoResult<()> {
        write!(
            f,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            self.body.as_ref().map(|s| s.as_str()).unwrap_or("")
        )
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            self.body.as_ref().map(|s| s.as_str()).unwrap_or("")
        )
    }
}
