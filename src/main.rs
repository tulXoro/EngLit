mod lexer;
mod parser;

use crate::lexer::Lexer;

/*********************************************
 * This is the main file which controls the
 * entire program.
 *********************************************/

fn main() {
    // Take args
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Invalid number of args!");
    }

    // Open file as a stream
    let contents= std::fs::read_to_string(&args[1]).expect("Could not read file!");

    // Make sure file is ASCII because our language is in English anyways
    if !contents.is_ascii() {
        panic!("File is not ASCII!");
    }

    // make contents into bytes so we can iterate through it
    // easier
    let contents = contents.as_bytes();

    let tokens = Lexer::tokenize(contents);

    // Export assembly to file
    std::fs::write("../out_program.asm", Lexer::tokens_to_asm(tokens)).expect("Could not write to file!");
    // Run assembly
    std::process::Command::new("nasm")
        .arg("-felf64")
        .arg("../out_program.asm")
        .output()
        .expect("Failed to execute nasm!");

    // Remove assembly
    /* Commented out for debugging
    std::process::Command::new("rm")
        .arg("../out_program.asm")
        .output()
        .expect("Failed to execute rm!");
    */

    // Link object file
    std::process::Command::new("ld")
        .arg("-o")
        .arg("../out_program")
        .arg("../out_program.o")
        .output()
        .expect("Failed to execute ld!");
    
    // Remove object file
    /* Commented out because our program should
       not delete anything currently
    std::process::Command::new("rm")
        .arg("../out_program.o")
        .output()
        .expect("Failed to execute rm!");
    */

    // Execute program
    std::process::Command::new("../out_program")
        .output()
        .expect("Failed to execute program!");

}
