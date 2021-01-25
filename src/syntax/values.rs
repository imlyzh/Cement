use pest::iterators::Pair;
use std::{collections::LinkedList, fmt::Display};

/*
#[derive(Debug, Clone, Default)]
pub struct Pos {
	pub start: usize,
	pub end: usize,
}
*/

#[derive(Debug, Clone)]
pub enum Value {
	Nil,
	Bool(bool),
	Char(char),
	Int(i64),
	Uint(u64),
	Float(f64),
	Str(String),
	Sym(Symbol),
	List(List),
	Vec(Vec<Value>),
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Symbol {
	pub id: String,
	pub line: usize,
	pub colum: usize,
	pub pos: usize,
}

impl Display for Symbol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.id.fmt(f)
	}
}


pub trait ParseFrom<T>
where
	Self: std::marker::Sized,
{
	fn parse_from(pair: Pair<T>) -> Self;
}
