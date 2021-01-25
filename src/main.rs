mod syntax;
mod evalution;

use std::{
	io::{stdin, stdout, Write},
};
// use evalution::context::ThreadContext;


use crate::syntax::parser::*;

fn main() {
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
			
			let res: Result<_, ()> = Ok(i);
			
			println!("> {:?}", res);
			// println!("> {}", res.map_or("error.".to_string(), |v| v.to_string()));
		}
	}
}
