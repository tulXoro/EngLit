mod lexer;
mod parser;

use crate::lexer::Lexer;

fn main() {
    // Take args
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Invalid number of args!");
    }

    // Open file
    let contents: String = std::fs::read_to_string(&args[1]).expect("Could not find file!");

    // Make sure our file is written in ascii because the language is pure english anyways
    if !contents.is_ascii() {
        panic!("File is not in ascii!");
    }

    let lexer = Lexer {};

    let tokens = lexer.tokenize(contents);

    // Export assembly to file
    std::fs::write("../out_program.asm", lexer.tokens_to_asm(tokens)).expect("Could not write to file!");
    // Run assembly
    std::process::Command::new("nasm")
        .arg("-felf64")
        .arg("../out_program.asm")
        .output()
        .expect("Failed to execute nasm!");

    std::process::Command::new("ld")
        .arg("-o")
        .arg("../out_program")
        .arg("../out_program.o")
        .output()
        .expect("Failed to execute ld!");

    std::process::Command::new("../out_program")
        .output()
        .expect("Failed to execute program!");
}
