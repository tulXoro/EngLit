mod lexer;
mod parser;

use crate::lexer::Lexer;

fn main() {
    // Take args
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Invalid number of args!");
    }

    let contents: String = std::fs::read_to_string(&args[1]).expect("Could not find file!");

    let lexer = Lexer {};

    lexer.tokenize(contents);
}
