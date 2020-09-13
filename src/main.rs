mod preprocesser;
mod syntax;

use crate::syntax::parser::*;
use std::io::{stdin, stdout, Write};

fn main() {
    println!("Hello, world!");
    println!("out: {:?}", parse("+ +"));
    loop {
        stdout().write(">>> ".as_bytes()).unwrap();
        stdout().flush().unwrap();
        let mut a = String::new();
        stdin().read_line(&mut a).unwrap();
        println!("out: {:?}", parse(&a));
    }
}
