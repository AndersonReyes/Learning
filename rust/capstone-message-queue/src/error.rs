use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    OffsetOutOfRange(u64),
    CorruptRecord(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "io error: {e}"),
            Error::OffsetOutOfRange(o) => write!(f, "offset {o} out of range"),
            Error::CorruptRecord(msg) => write!(f, "corrupt record: {msg}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Error::Io(e) = self { Some(e) } else { None }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
