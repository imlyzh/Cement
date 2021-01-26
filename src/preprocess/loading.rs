use std::sync::Arc;

use lazy_static::lazy_static;

use crate::error::SyntaxMatchError;
use crate::syntax::parser::repl_parse;
use crate::context::{Module, ANONYMOUS_MODULE_NAME};
use crate::values::*;

use super::match_template::*;


lazy_static!{
	static ref MODULE_MATCH_TEMP: Value =
	repl_parse("((quote module) name body ...)").unwrap();
}

impl Module {
	pub fn loading(parent: Option<Arc<Module>>, i: ListPia) -> Result<Self, SyntaxMatchError> {
		if i.len() == 1 {
			let mut ctx = MatchRecord::default();
			match_template(
				&mut ctx,
				&MODULE_MATCH_TEMP,
				i.front().unwrap())?;
			
			let name = ctx.maps.borrow()
				.get(&Arc::new(Symbol::new("name"))).unwrap()
				.get_sym().unwrap();
			let body = ctx.extend_maps.borrow()
				.get(&Arc::new(Symbol::new("name"))).unwrap();

			let m = Module::new(name, parent);
			todo!()
		} else {
			let r = Module::new(ANONYMOUS_MODULE_NAME.clone(), parent);
			todo!()
		}
	}
}