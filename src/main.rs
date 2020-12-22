mod syntax;
mod evalution;

use std::{
    any::Any,
    io::{stdin, stdout, Write},
};
use crate::syntax::parser::*;
use crate::evalution::{context::ReplEnv, eval::eval};

fn main() {
    let mut env = ReplEnv::new();

    loop {
        // read
        stdout().write(">>> ".as_bytes()).unwrap();
        stdout().flush().unwrap();
        let mut a = String::new();
        stdin().read_line(&mut a).unwrap();
        // parse
        let ast = parse(&a);
        let ast = ast.expect("syntax error");
        for i in ast {
            // eval
            // let res = eval(&mut env, &i).0;
            let res: Result<_, ()> = Ok(i);
            // print
            println!("> {}", res.map_or("error.".to_string(), |v| v.to_string()));
        }
    }
}
