#![warn(clippy::all)]
// Types of symbols in the Simple Scala programming language.
pub enum SymbolType {
    Keyword,
    Constant,
    Identifier,
    SpecialSymbol,
}

pub struct Token {
    token: String,
    symbol_type: SymbolType,
}
