mod lexer;
mod parser;
mod tables;
mod helper;
mod codegen;

use std::fs;
use std::process::Command;

fn main() {
    let file_path = "test.bky"; // replace with args
    let contents = fs::read_to_string(file_path).expect("Could not read file path");

    let tokens = lexer::tokenize(&contents);
    let prog = parser::parse(tokens);
    let code = codegen::codegen(prog);

    let out_file = "out.asm";
    fs::write(out_file, code).expect("Could not write file");
    // check with nasm and stuff

    Command::new("nasm")
        .args(["-felf64", "out.asm"])
        .output()
        .expect("Failed to execute nasm");

    Command::new("ld")
        .args(["-o", "out", "out.o"])
        .output()
        .expect("Failed to execute ld");
}
