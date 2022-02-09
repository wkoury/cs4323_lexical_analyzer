#![warn(clippy::all)]
use lazy_static::lazy_static;
use regex::Regex;

// A struct to represent the scanner, keeping track of where the character is consumed, among other things.
#[derive(Clone)]
pub struct Source {
    source: String,
    index: usize,
    state: u32,
    flag: bool, // FIXME: I would like to rename this, I don't think the name is very informative right now.
}

impl Source {
    // Create a new source object.
    pub fn new(src: String) -> Self {
        Source {
            source: src,
            index: 0,
            state: 0,
            flag: false,
        }
    }

    // Step to the next character in the source and return it.
    pub fn step(&mut self) -> char {
        // TODO: skip whitespace
        let ret: char = self.source.chars().nth(self.index).unwrap();
        self.index += 1;

        ret
    }

    // Determine whether we have consumed all characters in the source.
    pub fn is_done(&mut self) -> bool {
        self.index == self.source.len()
    }

    // Start moving along the DFA.
    pub fn scan(&mut self) {
        if self.is_done() {
            return;
        }

        self.initial_state();
    }

    fn initial_state(&mut self) {
        println!("hi");
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
    if c == '\0' {
        // FIXME: do we actually need this? is this useful?
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
