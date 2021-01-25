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
		let res = repl_parse(&a);
		println!("> {:?}", res);
		// println!("> {}", res.map_or("error.".to_string(), |v| v.to_string()));
	}
}
