use frontend::parse::parse_test;

mod ast;
mod frontend;
mod env;
mod runtime;
mod type_infer;
mod partial_evaluation;
mod codegen;

fn main() {
    let r = parse_test(include_str!("../Scripts/test.cmt"));
    println!("out: {:?}", r);
}
