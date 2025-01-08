use crate::lexer::lexer_types::*;
use std::fmt;
use std::mem::discriminant;

pub fn format_vec<T: fmt::Display>(v: &Vec<T>) -> String {
    v.iter()
        .map(|stmt| format!("{}", stmt))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn format_slice<T: fmt::Display>(s: &[T]) -> String {
    s.iter()
        .map(|stmt| format!("{}", stmt))
        .collect::<Vec<String>>()
        .join("\n")
}

fn match_inner_symbol(a: &Token, b: &Symbol) -> bool {
    match a {
        Token::SymbolLit(a_s) if discriminant(a_s) == discriminant(&b) => true,
        _ => false,
    }
}

pub fn split_and_parse<T>(
    tokens: &[Token],
    delimiter: &Symbol,
    expected_final: bool,
    parser: fn(&[Token]) -> Option<T>,
) -> Vec<T> {
    let mut result = Vec::new();

    #[allow(unused_assignments)]
    let (mut head, mut tail) = tokens.split_at(0);
    let mut split_len = 0;

    while !tail.is_empty() {
        if split_len == tail.len() - (if expected_final { 1 } else { 0 }) {
            (head, _) = tail.split_at(split_len);
            result.push(parser(head).unwrap());
            break;
        } else if match_inner_symbol(&tail[split_len], delimiter) {
            (head, tail) = tail.split_at(split_len);
            result.push(parser(head).unwrap());
            (_, tail) = tail.split_at(1);
            split_len = 0;
        } else {
            split_len += 1
        }
    }

    result
}
