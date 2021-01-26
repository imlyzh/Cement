use pest::iterators::Pair;
use std::{cell::RefCell, collections::LinkedList, fmt::Display, hash::Hash, sync::Arc};

use crate::context::FunctionDef;


#[derive(Debug, Clone, PartialEq)]
pub enum Value {
	Nil,
	Bool(bool),
	Char(char),
	Int(i64),
	Uint(u64),
	Float(f64),
	Str(Arc<String>),
	Sym(Arc<Symbol>),
	List(Arc<List>),
	Vec(Arc<Vec<Value>>),
	Function(Arc<FunctionDef>),
}

macro_rules! impl_get_item {
	($name:ident, $item:ident, $tp:path) => {
		pub fn $name(&self) -> Option<$tp> {
			if let Value::$item(x) = self {
				Some(x.clone())
			} else {
				None
			}
		}
	};
}


impl Value {
	impl_get_item!(get_bool, Bool, bool);
	impl_get_item!(get_char, Char, char);
	impl_get_item!(get_int, Int, i64);
	impl_get_item!(get_uint, Uint, u64);
	impl_get_item!(get_float, Float, f64);
	impl_get_item!(get_str, Str, Arc<String>);
	impl_get_item!(get_sym, Sym, Arc<Symbol>);
	impl_get_item!(get_list, List, Arc<List>);
	impl_get_item!(get_vec, Vec, Arc<Vec<Value>>);
	impl_get_item!(get_fun, Function, Arc<FunctionDef>);
}

/*
impl std::fmt::Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::Nil => f.write_str("nil"),
			Value::Bool(v) => f.write_str(&v.to_string()),
			Value::Char(v) => f.write_str(&v.to_string()),
			Value::Int(v) => f.write_str(&v.to_string()),
			Value::Uint(v) => f.write_str(&v.to_string()),
			Value::Float(v) => f.write_str(&v.to_string()),
			Value::Str(v) => f.write_str(&v.to_string()),
			Value::Sym(v) => f.write_str(&v.to_string()),
		    Value::List(v) => v.fmt(f),
		    Value::Vec(v) => todo!("vec fmt"),
		}
	}
}
//  */

pub type ListPia = LinkedList<Value>;

#[derive(Debug, Clone, PartialEq)]
pub struct List(pub ListPia);

/*
impl std::fmt::Display for List {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let list = self.0.iter();
		let str_list: Vec<String> = list.map(Value::to_string).collect();
		let retstr = format!("({})", str_list.join(" "));
		f.write_str(&retstr)
	}
}
//  */

#[derive(Debug, Clone, Eq)]
pub struct Symbol {
	pub id: String,
	pub line: usize,
	pub colum: usize,
	pub pos: usize,
	pub scope: RefCell<LinkedList<Arc<Symbol>>>,
	// pub value: RefCell<Option<Value>>,
}

impl Symbol {
	pub fn new(i: &str) -> Self {
		Symbol {
		    id: i.to_string(),
		    line: 0,
		    colum: 0,
		    pos: 0,
		    scope: RefCell::new(LinkedList::new()),
		    // value: RefCell::new(None),
		}
	}
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Display for Symbol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.id.fmt(f)
	}
}

impl Hash for Symbol {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}


pub trait ParseFrom<T>
where
	Self: std::marker::Sized,
{
	fn parse_from(pair: Pair<T>) -> Self;
}
