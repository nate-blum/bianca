use crate::lexer::lexer_types::SYMBOLS;
use std::iter;
use std::str::FromStr;

pub mod lexer_types;

fn is_terminator(c: char) -> bool {
    c.is_whitespace() || SYMBOLS.contains_key(&c)
}

pub fn tokenize(input: &str) -> Vec<lexer_types::Token> {
    let (mut tokens, mut buff) = (Vec::new(), String::new());

    for c in input.chars().chain(iter::once(' ')) {
        if is_terminator(c) {
            if buff.len() > 0 {
                tokens.push(lexer_types::Token::from_str(&buff).unwrap());
            }

            if SYMBOLS.contains_key(&c) {
                tokens.push(lexer_types::Token::SymbolLit(SYMBOLS.get(&c).cloned().unwrap()));
            }

            buff.clear();
        } else {
            buff.push(c);
        }
    }

    tokens
}
