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
    PassportParseError(PassportParseError),
    SeatParseError(SeatParseError),
}

impl fmt::Display for TopLevelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TopLevelError::IOError(e) => write!(f, "IO error: {}", e),
            TopLevelError::NoInputFound => write!(f, "No valid inputs found"),
            TopLevelError::NoSolutionFound => write!(f, "No solution found."),
            TopLevelError::PassportParseError(p) => write!(f, "Error parsing passport: {}", p),
            TopLevelError::SeatParseError(s) => write!(f, "Error parsing seat: {}", s),
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

pub enum PassportParseError {
    InvalidChunk(String),
    InvalidField(String),
}

impl fmt::Display for PassportParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PassportParseError::InvalidChunk(s) => {
                write!(f, "Invalid chunk in passport line: {}", s)
            }
            PassportParseError::InvalidField(s) => write!(f, "Invalid field in passport: {}", s),
        }
    }
}

convert_error!(PassportParseError, TopLevelError, PassportParseError);

#[derive(Debug, PartialEq)]
pub enum SeatParseError {
    InvalidSeatIdentifier(String),
    BadSeatRowSectionSize(usize),
    BadSeatColumnSectionSize(usize),
    UnexpectedRowCharacter(char),
    UnexpectedColumnCharacter(char),
    DidNotResolveColumn(String),
    DidNotResolveRow(String),
}

impl fmt::Display for SeatParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SeatParseError::InvalidSeatIdentifier(s) => {
                write!(f, "Invalid seat identifier: {:?}", s)
            }
            SeatParseError::BadSeatRowSectionSize(x) => write!(
                f,
                "Bad identifiers for rows; expected {} characters, got {}",
                7, x
            ),
            SeatParseError::BadSeatColumnSectionSize(x) => write!(
                f,
                "Bad identifiers for columns; expected {} characters, got {}",
                3, x
            ),
            SeatParseError::UnexpectedRowCharacter(c) => {
                write!(f, "Unexpected character when parsing rows: {:?}", c)
            }
            SeatParseError::UnexpectedColumnCharacter(c) => {
                write!(f, "Unexpected character when parsing columns: {:?}", c)
            }
            SeatParseError::DidNotResolveRow(s) => {
                write!(f, "Could not resolve row with {:?}", s)
            }
            SeatParseError::DidNotResolveColumn(s) => {
                write!(f, "Could not resolve row with {:?}", s)
            }
        }
    }
}

convert_error!(SeatParseError, TopLevelError, SeatParseError);
