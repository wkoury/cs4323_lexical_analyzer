#![warn(clippy::all)]
// Keeping track of a few types of errors
#[derive(Debug, Clone)]
pub enum ErrorType {
    InvalidSymbol,
    IdentifierHasTooManyPeriods,
    AllOthers,
    // TODO: add more as you come up with them
}

#[derive(Clone, Debug)]
pub struct Error {
    pub(crate) error_type: ErrorType,
}
