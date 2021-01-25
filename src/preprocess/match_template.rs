use std::{cell::RefCell, collections::{HashMap}, sync::Arc};

use crate::values::*;


#[derive(Debug)]
pub enum SyntaxMatchError {
	MatchError,
	RepeatedSymbol
}

#[derive(Debug, Default)]
pub struct MatchRecord {
	pub maps: RefCell<HashMap<Arc<Symbol>, Value>>,
	pub multi_maps: RefCell<HashMap<Arc<Symbol>, ListPia>>,
}

pub fn match_template(record: &mut MatchRecord, temp: &Value, inp: &Value) -> Result<(), SyntaxMatchError> {
	match (temp.clone(), inp.clone()) {
		(Value::Sym(id), v) => {
			record.maps.get_mut().insert(id, v);
		},
		(Value::List(a), Value::List(b)) => {
			a.as_ref().0.iter()
			.zip(b.as_ref().0.iter())
			.try_for_each(|(x, y)| match_template(record, x, y))?;
		}
		_ => if temp != inp {
			return Err(SyntaxMatchError::MatchError);
		}
	}
	Ok(())
}