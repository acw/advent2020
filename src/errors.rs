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
    #[error("Numeric conversion error: {0}")]
    NumConversionError(#[from] ParseIntError),
    #[error("Error parsing map: {0}")]
    MapParseError(#[from] MapParseError),
    #[error("Error accessing map: {0}")]
    MapOperationError(#[from] MapOperationError),
    #[error("Illegal ferry command: {0}")]
    IllegalFerryCommand(#[from] IllegalFerryCommand),
    #[error("Mask parsing error: {0}")]
    MaskParseError(#[from] MaskParseError),
    #[error("Bitmask command parsing error: {0}")]
    BitmaskCommandParseError(#[from] BitmaskCommandParseError),
    #[error("Ticket parsing error: {0}")]
    TicketParseError(#[from] TicketParseError),
    #[error("Bad rule parse: {0}")]
    GrammarParseError(#[from] GrammarParseError),
    #[error("Bad tile parse: {0}")]
    TileParseError(#[from] TileParseError),
    #[error("Error parsing directions: {0}")]
    DirectionParseError(#[from] DirectionParseError),
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

#[derive(Error, Debug)]
pub enum InstructionParseError {
    #[error("Unknown opcode {0}")]
    UnknownOpcode(String),
    #[error("Couldn't convert number: {source}")]
    NumConversionError {
        #[from]
        source: ParseIntError,
    },
    #[error("Encountered an empty instruction (?)")]
    EmptyInstruction,
    #[error("Instruction '{0}' missing an operand")]
    MissingOperand(String),
}

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Tried to execute non-existent instruction at {0}")]
    NonExistentLocation(isize),
}

#[derive(Error, Debug)]
pub enum MapOperationError {
    #[error("Out of bounds indexing map with ({0},{1})")]
    OutOfBounds(usize, usize),
    #[error("Fell off the edge of the map")]
    FellOffEdge,
}

#[derive(Error, Debug)]
pub enum IllegalFerryCommand {
    #[error("Ran into an empty string for a command (?)")]
    EmptyCommand,
    #[error("Unknown command {0:?}")]
    UnknownCommand(char),
    #[error("Problem converting number: {0}")]
    BadNumber(#[from] ParseIntError),
}

#[derive(Error, Debug)]
pub enum MaskParseError {
    #[error("Mask value is the wrong size; expected 36 characters, got {0}")]
    WrongLength(usize),
    #[error("Unexpected character '{0}'")]
    UnexpectedCharacter(char),
}

#[derive(Error, Debug)]
pub enum BitmaskCommandParseError {
    #[error("Got an empty command?")]
    EmptyCommand,
    #[error("Got a partial {0} command")]
    PartialCommand(String),
    #[error("Unknown command: {0}")]
    UnknownCommand(String),
    #[error("Error parsing mask: {0}")]
    MaskParseError(#[from] MaskParseError),
    #[error("Bad integer value encountered: {0}")]
    BadNumber(#[from] ParseIntError),
}

#[derive(Error, Debug)]
pub enum TicketParseError {
    #[error("Unterminated field section in file")]
    UnterminatedFieldDefs,
    #[error("Failure to parse number: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("Problem parsing your ticket")]
    YourTicketParseError,
    #[error("Problem parsing nearby tickets")]
    NearbyTicketParseError,
    #[error("Bad field definition: {0}")]
    BadFieldDefinition(String),
}

#[derive(Error, Debug, PartialEq)]
pub enum GrammarParseError {
    #[error("Bad rule reference: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("Reference to unknown rule: {0}")]
    UnknownRule(usize),
    #[error("Bad rule definition: {0}")]
    BadRule(String),
    #[error("Duplicate rule definition for {0}")]
    DuplicateRule(usize),
}

#[derive(Error, Debug, PartialEq)]
pub enum TileParseError {
    #[error("No tile identifier found")]
    NoTileIdentifier,
    #[error("Illegal character: '{0}'")]
    IllegalCharacter(char),
    #[error("Illegal tile identifier: {0}")]
    IllegalTileIdentifier(#[from] ParseIntError),
    #[error("Illegal tile dimensions for tile {0}")]
    IllegalDimensions(usize),
    #[error("Weird start to tile: {0}")]
    BadTileStart(String),
}

#[derive(Error, Debug, PartialEq)]
pub enum DirectionParseError {
    #[error("Invalid base for direction: {0}")]
    InvalidBaseDirection(char),
    #[error("Invalid suffix for north/south direction: {0}")]
    InvalidNorthSouthSuffix(char),
    #[error("Incomplete north/south direction")]
    IncompleteNorthSouthDirection,
}
