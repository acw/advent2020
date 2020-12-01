use std::fmt;
use std::io;

pub enum TopLevelError {
    IOError(io::Error),
    NoSolutionFound,
    UnknownError,
}

impl fmt::Display for TopLevelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TopLevelError::IOError(e) => write!(f, "IO error: {}", e),
            TopLevelError::NoSolutionFound => write!(f, "No solution found."),
            TopLevelError::UnknownError => {
                write!(f, "Unknown error occurred; this shouldn't be possible.")
            }
        }
    }
}

macro_rules! convert_error {
    ($type: ty, $pattern: ident) => {
        impl From<$type> for TopLevelError {
            fn from(x: $type) -> TopLevelError {
                TopLevelError::$pattern(x)
            }
        }
    };
}

convert_error!(io::Error, IOError);
