pub mod context;
pub mod error;
pub mod evalution;
pub mod preprocess;
pub mod syntax;
pub mod utils;
pub mod values;

use std::io::{stdin, stdout, Write};
// use evalution::context::ThreadContext;


use context::*;
use preprocess::{loading::Define, sexpr_parser::SexprParser};
// use context::FunctionDef;
// use preprocess::symbols::*;
use syntax::parser::*;


fn main() -> ! {
    // let modu = Module::new(&Handle::new(Symbol::new("repl-module")), &None);
    loop {
        // read
        stdout().write_all(">>> ".as_bytes()).unwrap();
        stdout().flush().unwrap();
        let mut a = String::new();
        stdin().read_line(&mut a).unwrap();
        let a = a.trim();
        if a.is_empty() {
            continue;
        }
        // parse
        let res = repl_parse(a).unwrap();
		// println!("res: {}", res);
		let r = Define::sexpr_parse(&res);
		match r {
			Ok((name, value)) => {
				println!("fun name: {:?}; value: {:?}", name, value);
			},
			Err(e) => {
				println!("exception: {:?}", e);
			}
		}
        // stdout().write_all("query: ".as_bytes()).unwrap();
        // stdout().flush().unwrap();
        // let mut query = String::new();
        // stdin().read_line(&mut query).unwrap();
        // let query_result = modu.find_name(query.trim());
        // println!("query result: {:?}", query_result);
    }
}
