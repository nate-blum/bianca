use crate::parser::Expr;
use phf::phf_map;

pub static BUILTINS: phf::Map<&'static str, fn(&Vec<Expr>) -> String> = phf_map! {
    "exit" => gen_exit_call,
};

pub fn gen_exit_call(args: &Vec<Expr>) -> String {
    match args.first() {
        Some(Expr::IntLit(code)) =>
            String::from(format!("\tmov rax, 60\n\tmov rdi, {}\n\tsyscall\n", code)),
        _ => panic!("Improper exit call")
    }
}