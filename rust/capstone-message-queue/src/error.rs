use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    OffsetOutOfRange(u64),
    CorruptRecord(String),
    TopicAlreadyExists(String),
    TopicNotFound(String),
    PartitionOutOfRange(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "io error: {e}"),
            Error::OffsetOutOfRange(o) => write!(f, "offset {o} out of range"),
            Error::CorruptRecord(msg) => write!(f, "corrupt record: {msg}"),
            Error::TopicAlreadyExists(name) => write!(f, "topic already exists: {name}"),
            Error::TopicNotFound(name) => write!(f, "topic not found: {name}"),
            Error::PartitionOutOfRange(id) => write!(f, "partition {id} out of range"),
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
