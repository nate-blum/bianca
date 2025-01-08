use crate::helper::*;
use crate::lexer::lexer_types::*;
use std::fmt;

pub struct Program(pub Vec<Stmt>);

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Program:")?;
        writeln!(f, "{}", format_vec(&self.0))
    }
}

pub trait PrePatternParseable {
    fn parse(_: &[Token]) -> Option<Self>
    where
        Self: Sized;
}

pub enum Expr {
    FuncCall(String, Vec<Expr>),
    IntLit(i64),
}

fn parse_func_args(tokens: &[Token]) -> Vec<Expr> {
    split_and_parse(tokens, &Symbol::Comma, false, Expr::parse)
}

fn parse_func_call(tokens: &[Token]) -> Result<Expr, &'static str> {
    match tokens {
        [Token::Identifier(name), Token::SymbolLit(Symbol::OpenParen), middle @ .., Token::SymbolLit(Symbol::CloseParen)] =>
            Ok(Expr::FuncCall(String::from(name), parse_func_args(middle))),
        _ => Err("Improper function call"),
    }
}

impl PrePatternParseable for Expr {
    fn parse(tokens: &[Token]) -> Option<Expr> {
        match tokens {
            &[Token::Identifier(_), Token::SymbolLit(Symbol::OpenParen), ..] =>
                Some(parse_func_call(tokens).unwrap()),
            &[Token::IntLit(value)] => Some(Expr::IntLit(value)),
            _ => None,
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FuncCall(name, args) =>
                writeln!(f, "[P FUNC CALL] {} {}", name, format_vec(&args)),
            Self::IntLit(val) => writeln!(f, "[P INT] {}", val),
        }
    }
}

pub enum FuncImpl {
    BuiltIn,
    Defined(Vec<Stmt>),
}

pub enum Stmt {
    Expression(Expr),
    FuncDecl(String, Vec<Expr>, FuncImpl),
}

impl PrePatternParseable for Stmt {
    fn parse(tokens: &[Token]) -> Option<Stmt> {
        if let Some(stmt) = Expr::parse(tokens) {
            Some(Stmt::Expression(stmt))
        } else {
            None
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Stmt::Expression(ref expr) => writeln!(f, "[P STMT EXPR] {}", expr),
            Stmt::FuncDecl(ref name, ref args, _) =>
                writeln!(f, "[P STMT FUNCDECL] {} {}", name, format_vec(&args))
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Program {
    Program(split_and_parse(tokens.as_slice(), &Symbol::Semicolon, true, Stmt::parse))
}
