use super::method::Method;
use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    str,
    str::Utf8Error,
};

#[derive(Debug)]
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(value)?;

        Ok(Request {
            path: request.to_string(),
            query_string: None,
            method: Method::GET,
        })
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid request",
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidProtocol => "Invalid protocol",
            Self::InvalidMethod => "Invalid method",
        }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}
