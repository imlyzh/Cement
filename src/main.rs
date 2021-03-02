pub mod context;
pub mod error;
pub mod evalution;
pub mod preprocess;
pub mod syntax;
pub mod utils;
pub mod values;

use std::io::{stdin, stdout, Write};
// use evalution::context::ThreadContext;


use crate::{context::MacroDef, preprocess::sexpr_parser::SexprParser};
use preprocess::symbols::*;
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
		println!("res: {}", res);
		let r = MacroDef::sexpr_parse(&res);
		match r {
			Ok((name, pairs)) => {
				println!("macro name: {:?}; pairs: {:?}", name, pairs);
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
