mod lexer;

use std::fs;

use crate::lexer::Lexer;

fn main() {
    let content = fs::read_to_string("examples/basic.ez").expect("failed to read file");
    println!("source:");
    println!("{}", content);
    println!();

    let mut lexer = Lexer::new(&content);
    while let Some(token) = lexer.tokenize() {
        println!("token: {:?}", token);
    }
}
