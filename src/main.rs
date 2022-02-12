#![warn(clippy::all)]

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

use std::io::stdout;

use crossterm::{
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    ExecutableCommand, Result,
};

mod bookkeeper;
mod error;
mod scanner;

use crate::scanner::Source;

fn main() -> Result<()> {
    // Collect the command-line arguments
    let args: Vec<String> = env::args().collect();
    // Check for invalid use and terminate if required
    if args.len() != 2 {
        stdout()
            .execute(SetForegroundColor(Color::Red))?
            .execute(SetAttribute(Attribute::Bold))?
            .execute(Print("Usage: "))?
            .execute(SetAttribute(Attribute::NoBold))?
            .execute(SetAttribute(Attribute::Italic))?
            .execute(Print("./scanner <filename>\n"))?
            .execute(ResetColor)?;
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
        Ok(_) => println!("{}", s),
    };

    let mut src: Source = Source::new(s);

    while !src.is_done() {
        let tkn = src.scan();
        println!("{:?}", tkn);
    }

    // Return an Ok result
    Ok(())
}
