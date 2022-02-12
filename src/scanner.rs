#![warn(clippy::all)]
use lazy_static::lazy_static;
use regex::Regex;

use crate::bookkeeper::{SymbolType, Token};
use crate::error::{Error, ErrorType};

// A struct to represent the scanner, keeping track of where the character is consumed, among other things.
#[derive(Clone, Debug)]
pub struct Source {
    source: String,
    index: usize,
    line_number: usize,
    pub(crate) token: Option<Token>,
    pub(crate) extra_token: Option<Token>,
    pub(crate) error: Option<Error>,
}

impl Source {
    // Create a new source object.
    pub fn new(src: String) -> Self {
        Source {
            source: src,
            index: 0,
            line_number: 1,
            token: None,
            extra_token: None,
            error: None,
        }
    }

    fn read_character(&mut self) -> char {
        let ret: char = self.source.chars().nth(self.index).unwrap();
        if ret == '\n' {
            self.line_number += 1;
        }
        self.index += 1;
        println!("read character {} from the source", ret);

        ret
    }

    // Determine whether we have consumed all characters in the source.
    pub fn is_done(&mut self) -> bool {
        self.index == self.source.len()
    }

    // Start moving along the DFA.
    pub fn scan(&mut self) -> Option<&Token> {
        if self.is_done() {
            return None;
        }

        if self.extra_token.is_some() {
            println!("The flag is marked.");
            // Return the extra token in here
            return self.extra_token.as_ref();
        }

        self.error = None;
        self.token = None;
        self.extra_token = None;

        self.initial_state();

        self.token.as_ref()
    }

    // Start another iteration of the DFA.
    fn initial_state(&mut self) {
        let mut c = self.read_character();

        while c.is_whitespace() {
            if !self.is_done() {
                c = self.read_character();
            } else {
                return;
            }
        }

        match c {
            'p' => self.state_1(),
            'i' => self.state_23(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_1(&mut self) {
        let c = self.read_character();

        match c {
            'a' => self.state_2(),
            'r' => self.state_8(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_2(&mut self) {
        let c = self.read_character();

        match c {
            'c' => self.state_3(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_3(&mut self) {
        let c = self.read_character();

        match c {
            'k' => self.state_4(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_4(&mut self) {
        let c = self.read_character();

        match c {
            'a' => self.state_5(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_5(&mut self) {
        let c = self.read_character();

        match c {
            'g' => self.state_6(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_6(&mut self) {
        let c = self.read_character();

        match c {
            'e' => self.state_7(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_7(&mut self) {
        let c = self.read_character();

        if c.is_whitespace() {
            self.token = Some(Token {
                token: "package".to_string(),
                symbol_type: SymbolType::Keyword,
                line_number: self.line_number,
            });
        } else {
            self.error = Some(Error {
                error_type: ErrorType::InvalidSymbol,
            });
        }
    }

    fn state_8(&mut self) {
        let c = self.read_character();

        match c {
            'i' => self.state_9(),
            'o' => self.state_16(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_9(&mut self) {
        let c = self.read_character();

        match c {
            'v' => self.state_10(),
            'n' => self.state_14(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_10(&mut self) {
        let c = self.read_character();

        match c {
            'a' => self.state_11(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_11(&mut self) {
        let c = self.read_character();

        match c {
            't' => self.state_12(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_12(&mut self) {
        let c = self.read_character();

        match c {
            'e' => self.state_13(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_13(&mut self) {
        let c = self.read_character();

        if c.is_whitespace() {
            self.token = Some(Token {
                token: "private".to_string(),
                symbol_type: SymbolType::Keyword,
                line_number: self.line_number,
            });
        } else {
            self.error = Some(Error {
                error_type: ErrorType::InvalidSymbol,
            });
        }
    }

    fn state_14(&mut self) {
        let c = self.read_character();

        match c {
            't' => self.state_15(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_15(&mut self) {
        let c = self.read_character();

        if c.is_whitespace() {
            self.token = Some(Token {
                token: "print".to_string(),
                symbol_type: SymbolType::Keyword,
                line_number: self.line_number,
            });
        } else {
            self.error = Some(Error {
                error_type: ErrorType::InvalidSymbol,
            });
        }
    }

    fn state_16(&mut self) {
        let c = self.read_character();

        match c {
            't' => self.state_17(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_17(&mut self) {
        let c = self.read_character();

        match c {
            'e' => self.state_18(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_18(&mut self) {
        let c = self.read_character();

        match c {
            'c' => self.state_19(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_19(&mut self) {
        let c = self.read_character();

        match c {
            't' => self.state_20(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_20(&mut self) {
        let c = self.read_character();

        match c {
            'e' => self.state_21(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_21(&mut self) {
        let c = self.read_character();

        match c {
            'd' => self.state_22(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_22(&mut self) {
        let c = self.read_character();

        if c.is_whitespace() {
            self.token = Some(Token {
                token: "protected".to_string(),
                symbol_type: SymbolType::Keyword,
                line_number: self.line_number,
            });
        } else {
            self.error = Some(Error {
                error_type: ErrorType::InvalidSymbol,
            });
        }
    }

    fn state_23(&mut self) {
        let c = self.read_character();

        match c {
            'm' => self.state_24(),
            'f' => self.state_29(),
            'n' => self.state_30(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_24(&mut self) {
        let c = self.read_character();

        match c {
            'p' => self.state_25(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_25(&mut self) {
        let c = self.read_character();

        match c {
            'o' => self.state_26(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_26(&mut self) {
        let c = self.read_character();

        match c {
            'r' => self.state_27(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_27(&mut self) {
        let c = self.read_character();

        match c {
            't' => self.state_28(),
            _ => {
                self.error = Some(Error {
                    error_type: ErrorType::InvalidSymbol,
                })
            }
        }
    }

    fn state_28(&mut self) {
        let c = self.read_character();

        if c.is_whitespace() {
            self.token = Some(Token {
                token: "import".to_string(),
                symbol_type: SymbolType::Keyword,
                line_number: self.line_number,
            });
        } else {
            self.error = Some(Error {
                error_type: ErrorType::InvalidSymbol,
            });
        }
    }

    fn state_29(&mut self) {
        let c = self.read_character();

        if c.is_whitespace() {
            self.token = Some(Token {
                token: "if".to_string(),
                symbol_type: SymbolType::Keyword,
                line_number: self.line_number,
            });
        } else {
            self.error = Some(Error {
                error_type: ErrorType::InvalidSymbol,
            });
        }
    }

    fn state_30(&mut self) {
        let c = self.read_character();

        if c.is_whitespace() {
            self.token = Some(Token {
                token: "in".to_string(),
                symbol_type: SymbolType::Keyword,
                line_number: self.line_number,
            });
        } else {
            match c {
                't' => self.state_31(),
                _ => {
                    self.error = Some(Error {
                        error_type: ErrorType::InvalidSymbol,
                    })
                }
            }
        }
    }

    fn state_31(&mut self) {
        let c = self.read_character();

        if c.is_whitespace() {
            self.token = Some(Token {
                token: "int".to_string(),
                symbol_type: SymbolType::Keyword,
                line_number: self.line_number,
            });
        } else {
            self.error = Some(Error {
                error_type: ErrorType::InvalidSymbol,
            });
        }
    }

    fn state_32(&mut self) {
        let c = self.read_character();
    }

    fn state_33(&mut self) {
        let c = self.read_character();
    }

    fn state_34(&mut self) {
        let c = self.read_character();
    }

    fn state_35(&mut self) {
        let c = self.read_character();
    }

    fn state_36(&mut self) {
        let c = self.read_character();
    }

    fn state_37(&mut self) {
        let c = self.read_character();
    }

    fn state_38(&mut self) {
        let c = self.read_character();
    }

    fn state_39(&mut self) {
        let c = self.read_character();
    }

    fn state_40(&mut self) {
        let c = self.read_character();
    }

    fn state_41(&mut self) {
        let c = self.read_character();
    }

    fn state_42(&mut self) {
        let c = self.read_character();
    }

    fn state_43(&mut self) {
        let c = self.read_character();
    }

    fn state_44(&mut self) {
        let c = self.read_character();
    }

    fn state_45(&mut self) {
        let c = self.read_character();
    }

    fn state_46(&mut self) {
        let c = self.read_character();
    }

    fn state_47(&mut self) {
        let c = self.read_character();
    }

    fn state_48(&mut self) {
        let c = self.read_character();
    }

    fn state_49(&mut self) {
        let c = self.read_character();
    }

    fn state_50(&mut self) {
        let c = self.read_character();
    }

    fn state_51(&mut self) {
        let c = self.read_character();
    }

    fn state_52(&mut self) {
        let c = self.read_character();
    }

    fn state_53(&mut self) {
        let c = self.read_character();
    }

    fn state_54(&mut self) {
        let c = self.read_character();
    }

    fn state_55(&mut self) {
        let c = self.read_character();
    }

    fn state_56(&mut self) {
        let c = self.read_character();
    }

    fn state_57(&mut self) {
        let c = self.read_character();
    }

    fn state_58(&mut self) {
        let c = self.read_character();
    }

    fn state_59(&mut self) {
        let c = self.read_character();
    }

    fn state_60(&mut self) {
        let c = self.read_character();
    }

    fn state_61(&mut self) {
        let c = self.read_character();
    }

    fn state_62(&mut self) {
        let c = self.read_character();
    }

    fn state_63(&mut self) {
        let c = self.read_character();
    }

    fn state_64(&mut self) {
        let c = self.read_character();
    }

    fn state_65(&mut self) {
        let c = self.read_character();
    }

    fn state_66(&mut self) {
        let c = self.read_character();
    }

    fn state_67(&mut self) {
        let c = self.read_character();
    }

    fn state_68(&mut self) {
        let c = self.read_character();
    }

    fn state_69(&mut self) {
        let c = self.read_character();
    }

    fn state_70(&mut self) {
        let c = self.read_character();
    }

    fn state_71(&mut self) {
        let c = self.read_character();
    }

    fn state_72(&mut self) {
        let c = self.read_character();
    }

    fn state_73(&mut self) {
        let c = self.read_character();
    }

    fn state_74(&mut self) {
        let c = self.read_character();
    }

    fn state_75(&mut self) {
        let c = self.read_character();
    }

    fn state_76(&mut self) {
        let c = self.read_character();
    }

    fn state_77(&mut self) {
        let c = self.read_character();
    }

    fn state_78(&mut self) {
        let c = self.read_character();
    }

    fn state_79(&mut self) {
        let c = self.read_character();
    }

    fn state_80(&mut self) {
        let c = self.read_character();
    }

    fn state_81(&mut self) {
        let c = self.read_character();
    }

    fn state_82(&mut self) {
        let c = self.read_character();
    }

    fn state_83(&mut self) {
        let c = self.read_character();
    }

    fn state_84(&mut self) {
        let c = self.read_character();
    }

    fn state_85(&mut self) {
        let c = self.read_character();
    }

    fn state_86(&mut self) {
        let c = self.read_character();
    }

    fn state_87(&mut self) {
        let c = self.read_character();
    }

    fn state_88(&mut self) {
        let c = self.read_character();
    }

    fn state_89(&mut self) {
        let c = self.read_character();
    }

    fn state_90(&mut self) {
        let c = self.read_character();
    }

    fn state_91(&mut self) {
        let c = self.read_character();
    }

    fn state_92(&mut self) {
        let c = self.read_character();
    }

    fn state_93(&mut self) {
        let c = self.read_character();
    }

    fn state_94(&mut self) {
        let c = self.read_character();
    }

    fn state_95(&mut self) {
        let c = self.read_character();
    }

    fn state_96(&mut self) {
        let c = self.read_character();
    }

    fn state_97(&mut self) {
        let c = self.read_character();
    }

    fn state_98(&mut self) {
        let c = self.read_character();
    }

    fn state_99(&mut self) {
        let c = self.read_character();
    }

    fn state_100(&mut self) {
        let c = self.read_character();
    }

    fn state_101(&mut self) {
        let c = self.read_character();
    }
}

// Given a character, determine whether it is alphabetical.
pub fn is_alphabetical(c: char) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[A-z]").unwrap();
    }
    RE.is_match(&c.to_string())
}

#[cfg(test)]
mod is_alphabetical_tests {
    use crate::scanner::is_alphabetical;

    #[test]
    fn test_lowercase_a() {
        assert!(is_alphabetical('a'));
    }

    #[test]
    fn test_uppercase_a() {
        assert!(is_alphabetical('A'));
    }

    #[test]
    fn test_uppercase_z() {
        assert!(is_alphabetical('Z'));
    }

    #[test]
    fn test_lowercase_z() {
        assert!(is_alphabetical('z'));
    }

    #[test]
    fn test_uppercase_m() {
        assert!(is_alphabetical('M'));
    }

    #[test]
    fn test_lowercase_m() {
        assert!(is_alphabetical('m'));
    }

    #[test]
    fn test_digit_zero() {
        assert!(!is_alphabetical('0'));
    }

    #[test]
    fn test_digit_nine() {
        assert!(!is_alphabetical('9'));
    }
}

// Given a character, determine if it is a digit.
fn is_digit(c: char) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[0-9]").unwrap();
    }

    RE.is_match(&c.to_string())
}

#[cfg(test)]
mod is_digit_tests {
    use crate::scanner::is_digit;

    #[test]
    fn test_lowercase_a() {
        assert!(!is_digit('a'));
    }

    #[test]
    fn test_uppercase_a() {
        assert!(!is_digit('A'));
    }

    #[test]
    fn test_uppercase_z() {
        assert!(!is_digit('Z'));
    }

    #[test]
    fn test_lowercase_z() {
        assert!(!is_digit('z'));
    }

    #[test]
    fn test_uppercase_m() {
        assert!(!is_digit('M'));
    }

    #[test]
    fn test_lowercase_m() {
        assert!(!is_digit('m'));
    }

    #[test]
    fn test_digit_zero() {
        assert!(is_digit('0'));
    }

    #[test]
    fn test_digit_nine() {
        assert!(is_digit('9'));
    }
}

// Keeping track of all of the special symbols in our language.
const SPECIAL_SYMBOLS: [char; 12] = ['#', ';', '{', '}', '(', ')', ':', ',', '=', '+', '*', '@'];

// Given a character, determines if the symbol is a special symbol.
pub fn is_special_symbol(c: char) -> bool {
    SPECIAL_SYMBOLS.contains(&c)
}

#[cfg(test)]
mod test_is_special_symbol {
    use crate::scanner::is_special_symbol;

    #[test]
    fn test_pound() {
        assert!(is_special_symbol('#'));
    }

    #[test]
    fn test_at() {
        assert!(is_special_symbol('@'));
    }

    #[test]
    fn test_comma() {
        assert!(is_special_symbol(','));
    }

    #[test]
    fn test_a() {
        assert!(!is_special_symbol('a'));
    }

    #[test]
    fn test_z() {
        assert!(!is_special_symbol('z'));
    }

    #[test]
    fn test_five() {
        assert!(!is_special_symbol('5'));
    }
}

// Given a character, determine if it is whitespace
pub fn is_whitespace(c: char) -> bool {
    if c == '\0' || c == ' ' || c == '\n' || c == '\t' {
        true
    } else {
        c.to_string().contains(char::is_whitespace)
    }
}

#[cfg(test)]
mod test_is_whitespace {
    use crate::scanner::is_whitespace;

    #[test]
    fn test_newline() {
        assert!(is_whitespace('\n'));
    }

    #[test]
    fn test_space() {
        assert!(is_whitespace(' '));
    }

    #[test]
    fn test_tab() {
        assert!(is_whitespace('\t'));
    }

    #[test]
    fn test_null() {
        assert!(is_whitespace('\0'));
    }

    #[test]
    fn test_a() {
        assert!(!is_whitespace('a'));
    }

    #[test]
    fn test_one() {
        assert!(!is_whitespace('1'));
    }

    #[test]
    fn test_period() {
        assert!(!is_whitespace('.'));
    }
}

#[cfg(test)]
mod scanner_tests {
    use crate::scanner::*;

    // Verify that the scanner can recognize the keyword package.
    #[test]
    fn test_package() {
        let src_str = "package a;".to_string();
        let mut src = Source::new(src_str);

        let tkn = src.scan().unwrap();

        let expected = &Some(Token {
            token: "package".to_string(),
            symbol_type: SymbolType::Keyword,
            line_number: 1,
        })
        .unwrap();

        assert_eq!(tkn, expected);
    }

    // Verify that the scanner can recognize the protected keyword.
    #[test]
    fn test_protected() {
        let src_str = "protected package a;".to_string();
        let mut src = Source::new(src_str);

        let tkn = src.scan().unwrap();

        let expected = &Some(Token {
            token: "protected".to_string(),
            symbol_type: SymbolType::Keyword,
            line_number: 1,
        })
        .unwrap();

        assert_eq!(tkn, expected);
    }

    // Verify that the scanner can recognize the keyword "int."
    #[test]
    fn test_int() {
        let src_str = "int a;".to_string();
        let mut src = Source::new(src_str);

        let tkn = src.scan().unwrap();

        let expected = &Some(Token {
            token: "int".to_string(),
            symbol_type: SymbolType::Keyword,
            line_number: 1,
        })
        .unwrap();

        assert_eq!(tkn, expected);
    }

    // Verify that the scanner can recognize the keyword "if."
    #[test]
    fn test_if() {
        let src_str = "if a;".to_string();
        let mut src = Source::new(src_str);

        let tkn = src.scan().unwrap();

        let expected = &Some(Token {
            token: "if".to_string(),
            symbol_type: SymbolType::Keyword,
            line_number: 1,
        })
        .unwrap();

        assert_eq!(tkn, expected);
    }

    // Verify that the scanner can recognize the keyword "in."
    #[test]
    fn test_in() {
        let src_str = "in a;".to_string();
        let mut src = Source::new(src_str);

        let tkn = src.scan().unwrap();

        let expected = &Some(Token {
            token: "in".to_string(),
            symbol_type: SymbolType::Keyword,
            line_number: 1,
        })
        .unwrap();

        assert_eq!(tkn, expected);
    }

    // Verify that the scanner can recognize the keyword "import."
    #[test]
    fn test_import() {
        let src_str = "import package a;".to_string();
        let mut src = Source::new(src_str);

        let tkn = src.scan().unwrap();

        let expected = &Some(Token {
            token: "import".to_string(),
            symbol_type: SymbolType::Keyword,
            line_number: 1,
        })
        .unwrap();

        assert_eq!(tkn, expected);
    }

    #[test]
    fn test_whitespace() {
        let src_str = " \t\n".to_string();
        let mut src = Source::new(src_str);

        let tkn: Option<&Token> = src.scan();
        let expected: Option<&Token> = None;

        // Basically what this test is doing is checking if tkn == None.
        debug_assert_eq!(tkn, expected);
    }
}
