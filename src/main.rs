#![warn(clippy::all)]

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

use colored::*;

mod bookkeeper;
mod error;
mod scanner;

use crate::bookkeeper::{convert_token_to_symbol_table_token, Bookkeeper, SymbolType, Token};
use crate::error::Error;
use crate::scanner::Source;

fn main() {
    // Collect the command-line arguments
    let args: Vec<String> = env::args().collect();
    // Check for invalid use and terminate if required
    if args.len() != 2 {
        print!("{}", "Usage: ".bold().red());
        println!("{}", "./scanner <filename>".red());
        process::exit(1);
    }

    // Attempt to open the file
    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => println!("{}\n{}", "Source program:".blue().bold(), s),
    };

    // Initialize the source
    let mut src: Source = Source::new(s);

    //Initialize the symbol table
    let mut symtab: Bookkeeper = Bookkeeper::new();

    while !src.is_done() {
        let scan_result = src.scan();
        let tkn: Option<&Token> = scan_result.0;
        let err: Option<&Error> = scan_result.1;

        if let Some(..) = err {
            print!("{}", "An error occurred: ".red().bold());
            println!("{:?}", err);
        }

        if let Some(..) = tkn {
            // If we have a token of some kind, print it.
            let tkn_ref = tkn.unwrap();
            println!("{}", tkn_ref);

            // If the token belongs in the symbol table, add it.
            if tkn.unwrap().symbol_type == SymbolType::Constant
                || tkn.unwrap().symbol_type == SymbolType::Identifier
            {
                symtab.insert(convert_token_to_symbol_table_token(tkn.unwrap().clone()));
            }
        }
    }

    println!("{}", "Symbol table contents:".blue().bold());
    for symbol in symtab.symbols {
        println!("{}", symbol);
    }
}
