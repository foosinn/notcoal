use std::convert::From;
use std::{error, fmt, io, result};

pub type Result<T> = result::Result<T, Error>;

// XXX The following ought to be handled by a macro

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    JSONError(serde_json::Error),
    RegexError(regex::Error),
    NotmuchError(notmuch::Error),
    MailParseError(mailparse::MailParseError),
    UnsupportedQuery(String),
    UnsupportedValue(String),
    RegexUncompiled(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnsupportedQuery(ref e) => e,
            Error::UnsupportedValue(ref e) => e,
            Error::RegexUncompiled(ref e) => e,
            _ => self.description(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            _ => Some(self),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(s: serde_json::Error) -> Error {
        Error::JSONError(s)
    }
}

impl From<io::Error> for Error {
    fn from(s: std::io::Error) -> Error {
        Error::IoError(s)
    }
}

impl From<regex::Error> for Error {
    fn from(s: regex::Error) -> Error {
        Error::RegexError(s)
    }
}

impl From<notmuch::Error> for Error {
    fn from(s: notmuch::Error) -> Error {
        Error::NotmuchError(s)
    }
}

impl From<mailparse::MailParseError> for Error {
    fn from(s: mailparse::MailParseError) -> Error {
        Error::MailParseError(s)
    }
}
