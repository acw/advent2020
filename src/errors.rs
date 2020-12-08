use std::io;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TopLevelError {
    #[error("IO error encountered ({source})")]
    IOError {
        #[from]
        source: io::Error,
    },
    #[error("No input found")]
    NoInputFound,
    #[error("No solution found")]
    NoSolutionFound,
    #[error("Unknown error occurred")]
    UnknownError,
    #[error("Failed to parse passport: {source}")]
    PassportParseErrorPassport {
        #[from]
        source: PassportParseError,
    },
    #[error("Failed to parse seat: {source}")]
    SeatParseError {
        #[from]
        source: SeatParseError,
    },
    #[error("Failed to parse baggage rule: {source}")]
    BaggageParseError {
        #[from]
        source: BaggageRuleParseError,
    },
    #[error("Failed to parse instruction: {source}")]
    InstructionParseError {
        #[from]
        source: InstructionParseError,
    },
    #[error("Error executing machine: {source}")]
    MachineExecutionError {
        #[from]
        source: ExecutionError,
    },
}

#[derive(Error, Debug)]
pub enum PasswordParseError {
    #[error("Failed to convert string to integer: {source}")]
    StringToIntError {
        #[from]
        source: ParseIntError,
    },
    #[error("Error nomnomnoming: {source}")]
    NomError {
        #[from]
        source: nom::Err<()>,
    },
}

#[derive(Error, Debug)]
pub enum MapParseError {
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("Uneven width at line {0}")]
    UnevenLines(usize),
}

#[derive(Error, Debug)]
pub enum PassportParseError {
    #[error("Invalid chunk in passport line: {0}")]
    InvalidChunk(String),
    #[error("Invalid field in passport: {0}")]
    InvalidField(String),
}

#[derive(Error, Debug, PartialEq)]
pub enum SeatParseError {
    #[error("Invalid seat identifier: {0}")]
    InvalidSeatIdentifier(String),
    #[error("Bad row section size {0}")]
    BadSeatRowSectionSize(usize),
    #[error("Bad column section size {0}")]
    BadSeatColumnSectionSize(usize),
    #[error("Unexpected row character '{0}'")]
    UnexpectedRowCharacter(char),
    #[error("Unexpected column character '{0}'")]
    UnexpectedColumnCharacter(char),
    #[error("Wasn't able to resolve column for '{0}'")]
    DidNotResolveColumn(String),
    #[error("Wasn't able to resolve row for '{0}'")]
    DidNotResolveRow(String),
}

#[derive(Error, Debug)]
pub enum BaggageRuleParseError {
    #[error("Error parsing rule: {0}")]
    NomError(String),
}

impl<'a> From<nom::Err<nom::error::Error<&'a str>>> for BaggageRuleParseError {
    fn from(x: nom::Err<nom::error::Error<&'a str>>) -> BaggageRuleParseError {
        match x {
            nom::Err::Incomplete(x) => {
                BaggageRuleParseError::NomError(format!("Incomplete data stream (need: {:?})", x))
            }
            nom::Err::Error(e) => BaggageRuleParseError::NomError(e.to_string()),
            nom::Err::Failure(e) => BaggageRuleParseError::NomError(e.to_string()),
        }
    }
}

#[derive(Error,Debug)]
pub enum InstructionParseError {
    #[error("Unknown opcode {0}")]
    UnknownOpcode(String),
    #[error("Couldn't convert number: {source}")]
    NumConversionError {
        #[from]
        source: ParseIntError
    },
    #[error("Encountered an empty instruction (?)")]
    EmptyInstruction,
    #[error("Instruction '{0}' missing an operand")]
    MissingOperand(String),
}

#[derive(Error,Debug)]
pub enum ExecutionError {
    #[error("Tried to execute non-existent instruction at {0}")]
    NonExistentLocation(isize),
}