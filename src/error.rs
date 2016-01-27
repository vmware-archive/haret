/// This is the standard error type used throughout this crate
/// Implementation based on suggestions in http://blog.burntsushi.net/rust-error-handling

use std::error::Error;
use std::fmt;
use std::io;
use super::Timeout;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum VrError {
    Io(String),
    AlreadyExists,
    BadEncoding(&'static str),
    Eof,
    Timeout(Timeout)
}

impl Error for VrError {
    fn description(&self) -> &str {
        match *self {
            VrError::Io(ref string) => string,
            VrError::AlreadyExists => "resource already exists",
            VrError::BadEncoding(_)  => "could not read encoded data",
            VrError::Eof => "end of file",
            VrError::Timeout(_) => "timeout"
        }
    }
}

impl fmt::Display for VrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VrError::Io(ref string) => write!(f, "IO error: {}", string),
            VrError::AlreadyExists => write!(f, "Error: resource already exists"),
            VrError::BadEncoding(encoding_type) => write!(f, "Could not read data encoded with {}",
                                                          encoding_type),
            VrError::Eof => write!(f, "Error: End Of File"),
            VrError::Timeout(ref timeout) => write!(f, "{}", timeout)
        }
    }
}

/// Need to implement from so we can use try!
impl From<io::Error> for VrError {
    fn from(err: io::Error) -> VrError {
        let s = format!("{}", err);
        VrError::Io(s)
    }
}
