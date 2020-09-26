// mod preprocesser;
mod interpreter;
mod syntax;

use crate::syntax::parser::*;
use crate::interpreter::evaluator::evaluation;
use std::{
    any::Any,
    io::{stdin, stdout, Write},
};

fn main() {
    println!("Hello, world!");
    println!("out: {:?}", parse("+ +"));
    loop {
        stdout().write(">>> ".as_bytes()).unwrap();
        stdout().flush().unwrap();
        let mut a = String::new();
        stdin().read_line(&mut a).unwrap();
        let ast = parse(&a);
        println!("out: {:?}", ast);
        if let Ok(x) = ast {
            for i in x {
                println!("eval-value: {:?}", evaluation(&i));
            }
        }
    }
}
