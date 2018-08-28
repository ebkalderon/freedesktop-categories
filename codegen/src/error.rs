//! Common error type used throughout the crate.

use std::env::VarError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::error::Error as StdError;
use std::io::Error as IoError;

use curl::Error as CurlError;

/// Types of errors and their causes.
#[derive(Debug)]
pub enum Error {
    /// Failed to download and/or cache the specification XML file.
    Curl(CurlError),
    /// Failed to read or write from a file descriptor.
    Io(IoError),
    /// Failed to load an environment variable.
    Var(VarError),
    /// Failed to parse the Freedesktop.org Desktop Menu spec.
    Xml(Box<StdError>),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Curl(_) => "Failed to download XML file from Freedesktop.org",
            Error::Io(_) => "Failed to read or write from a file",
            Error::Var(_) => "Failed to load an environment variable",
            Error::Xml(_) => "Could not parse Freedesktop.org Desktop Menu spec",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Curl(ref e) => Some(e),
            Error::Io(ref e) => Some(e),
            Error::Var(ref e) => Some(e),
            Error::Xml(ref e) => Some(e.as_ref()),
        }
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            Error::Curl(ref e) => write!(fmt, "{}: {}", self.description(), e),
            Error::Io(ref e) => write!(fmt, "{}: {}", self.description(), e), 
            Error::Var(ref e) => write!(fmt, "{}: {}", self.description(), e), 
            Error::Xml(ref e) => write!(fmt, "{}: {}", self.description(), e), 
        }
    }
}

impl From<CurlError> for Error {
    fn from(e: CurlError) -> Self {
        Error::Curl(e)
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::Io(e)
    }
}

impl From<VarError> for Error {
    fn from(e: VarError) -> Self {
        Error::Var(e)
    }
}

impl From<Box<StdError>> for Error {
    fn from(e: Box<StdError>) -> Self {
        Error::Xml(e)
    }
}
