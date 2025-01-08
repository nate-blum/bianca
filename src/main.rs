mod lexer;
mod parser;
mod tables;
mod helper;

use std::fs;

fn main() {
    let file_path = "test.bky"; // replace with args
    let contents = fs::read_to_string(file_path).expect("Could not read file path");

    let tokens = lexer::tokenize(&contents);
    let prog = parser::parse(tokens);
    println!("{}", prog);

    // for token in tokens {
    //     println!("{}", token);
    // }
}
