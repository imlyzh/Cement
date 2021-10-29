use std::sync::Arc;

use frontend::parse::parse_test;

use crate::{ast::{FunctionDef, TopLevel}, runtime::NameSpace};

mod ast;
mod frontend;
mod env;
mod runtime;
mod type_infer;
mod partial_evaluation;
mod codegen;

use partial_evaluation::PartialEval;

fn main() {
    let r = parse_test(include_str!("../Scripts/test.cmt"));
    println!("out: {:?}", r);
    println!("-------------------------------------");
    let env = Arc::new(NameSpace::new());
    for r in r {
        if let TopLevel::FunctionDef(FunctionDef(name, f)) = r {
            let r = f.2.partial_eval(env.clone());
            println!("partial_eval: {:?}", r);
        }
    }
}
