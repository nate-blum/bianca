mod lexer;
mod parser;
mod tables;
mod helper;
mod codegen;

use std::fs;

fn main() {
    let file_path = "test.bky"; // replace with args
    let contents = fs::read_to_string(file_path).expect("Could not read file path");

    let tokens = lexer::tokenize(&contents);
    let prog = parser::parse(tokens);
    let code = codegen::codegen(prog);

    let out_file = "test.asm";
    fs::write(out_file, code).expect("Could not write file");
}
