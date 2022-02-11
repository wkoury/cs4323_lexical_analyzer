#![warn(clippy::all)]

// Types of symbols in the Simple Scala programming language.
#[derive(Clone)]
pub enum SymbolType {
    Keyword,
    Constant,
    Identifier,
    SpecialSymbol,
}

#[derive(Clone)]
pub struct Token {
    pub(crate) token: String,
    pub(crate) symbol_type: SymbolType,
    pub(crate) line_number: usize,
}
