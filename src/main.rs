
pub mod syntax;
pub mod preprocesser;

use crate::syntax::parser::*;

fn main() {
    println!("Hello, world!");
    println!("out: {:?}", parse("(+ 1 2 3)"));
}
