use pest::iterators::Pair;
use std::{cell::RefCell, collections::LinkedList, convert::identity, fmt::Display, hash::Hash, iter::FromIterator, sync::Arc};

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
