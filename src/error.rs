#![warn(clippy::all)]
// Keeping track of a few types of errors
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    InvalidSymbol,
    ConstantHasTooManyPeriods,
    // TODO: add more as you come up with them
}

#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    pub(crate) error_type: ErrorType,
}
