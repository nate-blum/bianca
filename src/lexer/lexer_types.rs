use phf::phf_map;
use std::fmt;
use std::str::FromStr;

#[derive(Clone)]
pub enum Keyword {}

static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
};

impl FromStr for Keyword {
    type Err = ();

    fn from_str(kw: &str) -> Result<Keyword, Self::Err> {
        KEYWORDS.get(kw).cloned().ok_or(())
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        // match *self {
        //
        // }
        unimplemented!()
    }
}

#[derive(Clone)]
pub enum Symbol {
    OpenParen,
    CloseParen,
    Semicolon,
    Comma,
}

pub static SYMBOLS: phf::Map<char, Symbol> = phf_map! {
    '(' => Symbol::OpenParen,
    ')' => Symbol::CloseParen,
    ';' => Symbol::Semicolon,
    ',' => Symbol::Comma,
};

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::OpenParen => write!(f, "[S] ("),
            Self::CloseParen => write!(f, "[S] )"),
            Self::Semicolon => write!(f, "[S] ;"),
            Self::Comma => write!(f, "[S] ,"),
        }
    }
}

#[derive(Clone)]
pub enum Token {
    KeywordLit(Keyword),
    Identifier(String),
    IntLit(i64),
    SymbolLit(Symbol),
}

impl FromStr for Token {
    type Err = &'static str;

    fn from_str(str: &str) -> Result<Token, Self::Err> {
        let first = str.chars().next().unwrap();
        if first.is_alphabetic() {
            match Keyword::from_str(str) {
                Ok(kw) => Ok(Token::KeywordLit(kw)),
                _ => Ok(Token::Identifier(str.to_string()))
            }
        } else if first.is_numeric() {
            Ok(Token::IntLit(str.parse::<i64>().expect("Improper integer literal")))
        } else {
            Err("Unidentifiable token")
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::KeywordLit(ref kw) => write!(f, "[T KW] {}", kw),
            Token::Identifier(ref s) => write!(f, "[T IDENT] {}", s),
            Token::IntLit(ref i) => write!(f, "[T INT] {}", i),
            Token::SymbolLit(ref sym) => write!(f, "[T SYM] {}", sym),
        }
    }
}