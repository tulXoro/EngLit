use crate::lexer::Lexer;

mod lexer;
mod parser;

fn main() {
    // Take args
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Invalid number of args!");
    }

    let contents: String = std::fs::read_to_string(&args[2])
        .expect("Could not find file!");

    let lexer = Lexer{};

    lexer.tokenize(contents);

}
