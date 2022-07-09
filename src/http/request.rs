use super::method::{Method, MethodError};
use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Debug, Display, Result as FmtResult},
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
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol.trim() != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(path[i + 1..].to_string());
            path = &path[..i];
        }

        Ok(Self {
            method,
            path: path.to_string(),
            query_string,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (idx, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..idx], &request[idx + 1..]));
        }
    }
    None
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

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        ParseError::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
