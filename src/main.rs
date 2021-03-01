pub mod context;
pub mod error;
pub mod evalution;
pub mod preprocess;
pub mod syntax;
pub mod utils;
pub mod values;

use std::io::{stdin, stdout, Write};
// use evalution::context::ThreadContext;


use preprocess::symbols::*;
use preprocess::*;
use match_template::*;
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
        let temp = &FUNCTION_DEF_TEMP;
        let res = repl_parse(a).unwrap();

        // let _r = ModuleItem::loading(None, modu.clone(), &res);
        let mut ctx = MatchRecord::default();
        match_template(&mut ctx, temp, &res).unwrap();
        println!("ctx: {}", ctx);
        // stdout().write_all("query: ".as_bytes()).unwrap();
        // stdout().flush().unwrap();
        // let mut query = String::new();
        // stdin().read_line(&mut query).unwrap();
        // let query_result = modu.find_name(query.trim());
        // println!("query result: {:?}", query_result);
    }
}
