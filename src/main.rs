mod values;
mod context;
mod syntax;
mod evalution;
mod preprocess;
mod error;

use std::{io::{stdin, stdout, Write}};
// use evalution::context::ThreadContext;


use preprocess::match_template::*;

use crate::syntax::parser::*;

fn main() -> ! {
	loop {
		// read
		stdout().write(">>> ".as_bytes()).unwrap();
		stdout().flush().unwrap();
		let mut a = String::new();
		stdin().read_line(&mut a).unwrap();
		// parse
		let temp = repl_parse("((quote module) name body ...)").unwrap();
		let res = repl_parse(&a).unwrap();
		let mut mr =  MatchRecord::default();
		match_template(&mut mr, &temp, &res).unwrap();
		println!("> {:?}", res);
		println!("match: {:?}", mr);
		// println!("> {}", res.map_or("error.".to_string(), |v| v.to_string()));
	}
}
