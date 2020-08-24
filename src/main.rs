
pub mod syntax;

use crate::syntax::parser::*;

fn main() {
    println!("Hello, world!");
    println!("out: {:?}", parse("+ +"));
}
