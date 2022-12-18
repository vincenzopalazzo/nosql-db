//! Custom error message for the database
use std::{fmt::Display, string::FromUtf8Error};

#[derive(Debug)]
pub struct Error {
    #[allow(dead_code)]
    value: String,
}

impl Error {
    pub fn new(msg: &str) -> Self {
        Error {
            value: msg.to_string(),
        }
    }

    pub fn from<T: Display>(err: T) -> Self {
        let err = err.to_string();
        Error::new(err.as_str())
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Error::new(value.to_string().as_str())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
