use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
    Ok = 200,
    Accepted = 202,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::Accepted => "Accepted",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}
