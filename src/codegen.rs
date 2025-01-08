pub mod builtins;

use crate::codegen::builtins::*;
use crate::parser::*;
use crate::tables;

fn gen_expr(expr: &Expr) -> String {
    match expr {
        Expr::IntLit(_) => todo!(),
        Expr::FuncCall(name, args) => {
            // create scope system
            if let Some(fn_impl) = tables::FUNCS.get(name) {
                match fn_impl {
                    FuncImpl::BuiltIn => BUILTINS.get(name).unwrap()(args),
                    FuncImpl::Defined(_) => todo!()
                }
            } else {
                panic!("Undefined function {}.", name);
            }
        }
    }
}

fn gen_stmt(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Expression(expr) => gen_expr(expr),
        Stmt::FuncDecl(..) => todo!(),
    }
}

pub fn codegen(prog: Program) -> String {
    let mut code = String::from("global _start\n_start:\n");
    code.push_str(&prog.0.iter().map(|stmt| gen_stmt(stmt)).collect::<Vec<String>>().join("\n"));
    code.push_str(&BUILTINS.get("exit").unwrap()(&Vec::from([Expr::IntLit(0)])));
    code
}