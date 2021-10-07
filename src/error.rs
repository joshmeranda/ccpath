use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
// todo: consider a "no change" option
pub enum PathConvertError {
    InvalidUtf8Path,
    InvalidPath,
}

impl Display for PathConvertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PathConvertError::InvalidUtf8Path => {
                write!(f, "path contains invalid utf-8 characters")
            }
            PathConvertError::InvalidPath => {
                write!(f, "paths must container either a stem or a path or both")
            }
        }
    }
}

impl Error for PathConvertError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            _ => None,
        }
    }
}
