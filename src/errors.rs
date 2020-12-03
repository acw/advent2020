use nom;
use std::fmt;
use std::io;
use std::num::ParseIntError;

macro_rules! convert_error {
    ($type: ty, $super_type: ident, $pattern: ident) => {
        impl From<$type> for $super_type {
            fn from(x: $type) -> $super_type {
                $super_type::$pattern(x)
            }
        }
    };
}

pub enum TopLevelError {
    IOError(io::Error),
    NoInputFound,
    NoSolutionFound,
    UnknownError,
}

impl fmt::Display for TopLevelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TopLevelError::IOError(e) => write!(f, "IO error: {}", e),
            TopLevelError::NoInputFound => write!(f, "No valid inputs found"),
            TopLevelError::NoSolutionFound => write!(f, "No solution found."),
            TopLevelError::UnknownError => {
                write!(f, "Unknown error occurred; this shouldn't be possible.")
            }
        }
    }
}

convert_error!(io::Error, TopLevelError, IOError);

pub enum PasswordParseError {
    StringToIntError(ParseIntError),
    NomError(nom::Err<()>),
}

impl fmt::Display for PasswordParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PasswordParseError::NomError(e) => write!(f, "Parse error: {}", e),
            PasswordParseError::StringToIntError(e) => {
                write!(f, "Error converting string to integer: {}", e)
            }
        }
    }
}

convert_error!(ParseIntError, PasswordParseError, StringToIntError);
convert_error!(nom::Err<()>, PasswordParseError, NomError);

pub enum MapParseError {
    UnexpectedCharacter(char),
    UnevenLines(usize),
}

impl fmt::Display for MapParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MapParseError::UnevenLines(l) => write!(f, "Map has uneven width at line {}", l),
            MapParseError::UnexpectedCharacter(c) => {
                write!(f, "Unexpected character parsing map: {}", c)
            }
        }
    }
}
