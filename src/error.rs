use core::fmt;
use std::error;

#[derive(Debug)]
pub enum Error {
    TcpConnectionFailed,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::TcpConnectionFailed => write!(f, "tcp connection failed"),
        }
    }
}

impl Error {
    pub fn as_str(&self) -> &'static str {
        match self {
            Error::TcpConnectionFailed => "tcp connection failed",
        }
    }
}
