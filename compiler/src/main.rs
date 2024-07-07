mod lexer;
mod parser;

use crate::parser::{ParseError, Parser};
use std::fs;

fn main() {
    let content = fs::read_to_string("examples/basic.ez").expect("failed to read file");
    println!("source:");
    println!("{}", content);
    println!();

    let mut parser = Parser::new(&content);
    loop {
        let result = parser.parse();
        match result {
            Ok(expr) => {
                println!("EXPR: {:?}", expr);
            }

            Err(ParseError::NoMoreTokens) => break,

            Err(e) => {
                println!("ERROR: {:?}", e);
                break;
            }
        }
    }
}
