#![warn(clippy::all)]
// Keeping track of a few types of errors
enum Errors {
    InvalidCharacter,
    IdentifierHasTooManyPeriods,
    AllOthers,
    // TODO: add more as you come up with them
}
