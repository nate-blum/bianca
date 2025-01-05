mod lexer;

use std::fs;

fn main() {
    let file_path = "test.bky"; // replace with args
    let contents = fs::read_to_string(file_path).expect("Could not read file path");

}
