#![warn(clippy::all)]

use std::collections::HashSet;

// Types of symbols in the Simple Scala programming language.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SymbolType {
    Keyword,
    Constant,
    Identifier,
    SpecialSymbol,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Token {
    pub(crate) token: String,
    pub(crate) symbol_type: SymbolType,
    pub(crate) line_number: usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Token: {}\t Type: {:?}\t Line: {}",
            self.token, self.symbol_type, self.line_number
        )
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SymbolTableToken {
    pub(crate) token: String,
    pub(crate) symbol_type: SymbolType,
}

// Given a Token, return a SymbolTableToken.
pub fn convert_token_to_symbol_table_token(tkn: Token) -> SymbolTableToken {
    SymbolTableToken {
        token: tkn.token.clone(),
        symbol_type: tkn.symbol_type,
    }
}

pub struct Bookkeeper {
    pub(crate) symbols: HashSet<SymbolTableToken>,
}

impl Bookkeeper {
    pub fn new() -> Self {
        Bookkeeper {
            symbols: HashSet::new(),
        }
    }

    pub fn insert(&mut self, t: SymbolTableToken) {
        self.symbols.insert(t);
    }
}

#[cfg(test)]
mod symbol_table_tests {
    use crate::bookkeeper::*;

    #[test]
    fn test_one_entry() {
        let mut symtab = Bookkeeper::new();
        let tkn = SymbolTableToken {
            token: "test".to_string(),
            symbol_type: SymbolType::Identifier,
        };

        symtab.insert(tkn);

        let expected = 1;
        let actual = symtab.symbols.len();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_duplicate_entries() {
        let mut symtab = Bookkeeper::new();
        let tkn = SymbolTableToken {
            token: "test".to_string(),
            symbol_type: SymbolType::Identifier,
        };

        let dup_tkn = tkn.clone();

        symtab.insert(tkn);
        symtab.insert(dup_tkn);

        let expected = 1;
        let actual = symtab.symbols.len();

        assert_eq!(expected, actual);
    }
}
