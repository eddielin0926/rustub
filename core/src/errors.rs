use std::fmt;
use std::io;

#[derive(Debug)]
pub enum DiskError {
    InvalidPageId,
    PageNotFound,
    Io(io::Error),
}

impl From<io::Error> for DiskError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl fmt::Display for DiskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPageId => write!(f, "Invalid page ID"),
            Self::PageNotFound => write!(f, "Page not found"),
            Self::Io(err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl std::error::Error for DiskError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}
