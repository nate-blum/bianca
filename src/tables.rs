use crate::parser::*;
use phf::phf_map;

pub static FUNCS: phf::Map<&'static str, FuncImpl> = phf_map! {
    "exit" => FuncImpl::BuiltIn,
};