use std::sync::{Arc};

use crate::{error::SyntaxMatchError, syntax::parser::repl_parse};

use crate::{context::Module, values::*};

use super::match_template::*;



impl Module {
	pub fn loading(parent: Option<Arc<Module>>, i: ListPia) -> Result<Self, SyntaxMatchError> {
		if i.len() == 1 {
			let mut ctx = MatchRecord::default();
			match_template(
				&mut ctx,
				&repl_parse("((quote module) name body ...)").unwrap(),
				i.front().unwrap())?;
			
			let name = ctx.maps.borrow()
				.get(&Arc::new(Symbol::new("name"))).unwrap()
				.get_sym().unwrap();
			let body = ctx.extend_maps.borrow()
				.get(&Arc::new(Symbol::new("name"))).unwrap();

			let m = Module::new(name, parent);
			todo!()
		} else {
			let r = Module::new(Arc::new(Symbol::new("anonymous-module")), parent);
			todo!()
		}
	}
}