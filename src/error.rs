#![warn(clippy::all)]
// Keeping track of a few types of errors
#[derive(Debug, Clone)]
enum ErrorTypes {
    InvalidCharacter,
    IdentifierHasTooManyPeriods,
    AllOthers,
    // TODO: add more as you come up with them
}

#[derive(Clone, Debug)]
pub struct Error {
    msg: String,
    error_type: ErrorTypes,
}
