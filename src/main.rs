mod syntax;
mod evalution;

use crate::syntax::parser::*;
use std::{
    any::Any,
    io::{stdin, stdout, Write},
};

fn main() {
    loop {
        stdout().write(">>> ".as_bytes()).unwrap();
        stdout().flush().unwrap();
        let mut a = String::new();
        stdin().read_line(&mut a).unwrap();
        let ast = parse(&a).expect("syntax error");
        for i in ast {
            println!("> {}", i);
        }
    }
}
