use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::path::{Path, PathBuf};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum PathConvertError {
    InvalidUtf8Path(PathBuf),
    InvalidPath(PathBuf),
}

impl Display for PathConvertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PathConvertError::InvalidUtf8Path(path) => {
                write!(f, "path contains invalid utf-8 characters: {}", path.to_string_lossy())
            }
            PathConvertError::InvalidPath(path) => {
                write!(f, "paths must container either a stem or a path or both: '{}'", path.to_string_lossy())
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
