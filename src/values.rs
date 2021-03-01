use std::{
    cell::RefCell,
    collections::VecDeque,
    fmt::{Debug, Display},
    hash::Hash,
    iter::FromIterator,
    sync::Arc,
};

use crate::{context::FunctionDef, utils::string_intern};

pub type Handle<T> = Arc<T>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Char(char),
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(Handle<String>),
    Sym(Handle<Symbol>),
    // List(Handle<List>),
    Pair(Handle<Node>),
    Vec(Handle<Vec<Value>>),
    Function(Handle<FunctionDef>),
}

macro_rules! impl_is_type {
    ($name:ident, $item:ident) => {
        pub fn $name(&self) -> bool {
            if let Value::$item(_) = self {
				true
            } else {
                false
            }
        }
    };
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
	impl_is_type!(is_bool, Bool);
	impl_is_type!(is_char, Char);
    impl_is_type!(is_int, Int);
    impl_is_type!(is_uint, Uint);
    impl_is_type!(is_float, Float);
    impl_is_type!(is_str, Str);
    impl_is_type!(is_sym, Sym);
    impl_is_type!(is_pair, Pair);
    impl_is_type!(is_vec, Vec);
    impl_is_type!(is_fun, Function);
	pub fn is_nil(&self) -> bool {
		if let Value::Nil = self {
			true
		} else {
			false
		}
	}
	pub fn is_list(&self) -> bool {
		self.get_pair()
		.map_or(false, |x| x.1.is_pair() || x.1.is_nil())
	}
}

impl Value {
    impl_get_item!(get_bool, Bool, bool);
	impl_get_item!(get_char, Char, char);
    impl_get_item!(get_int, Int, i64);
    impl_get_item!(get_uint, Uint, u64);
    impl_get_item!(get_float, Float, f64);
    impl_get_item!(get_str, Str, Handle<String>);
    impl_get_item!(get_sym, Sym, Handle<Symbol>);
    impl_get_item!(get_pair, Pair, Handle<Node>);
    impl_get_item!(get_vec, Vec, Handle<Vec<Value>>);
    impl_get_item!(get_fun, Function, Handle<FunctionDef>);
	pub fn get_list(&self) -> Option<Handle<Node>> {
		if self.is_list() {
			Some(self.get_pair().unwrap())
		} else {
			None
		}
	}
}

// /*
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
            Value::Pair(v) => v.fmt(f),
            Value::Vec(v) => todo!("vec fmt"),
            Value::Function(v) => v.fmt(f),
        }
    }
}
//  */
#[derive(Debug, Clone, PartialEq)]
pub struct Node(pub Value, pub Value);

impl Node {
    pub fn cons(car: Value, cdr: Value) -> Node {
        Node(cdr, car)
    }
    pub fn new(car: Value, cdr: Value) -> Node {
        Self::cons(car, cdr)
        // vec![].iter()
    }
    pub fn car(&self) -> Value {
        self.0.clone()
    }
    pub fn cdr(&self) -> Value {
        self.1.clone()
    }

    pub fn last(&self) -> Value {
        match self {
            Node(_, Value::Pair(cdr)) => cdr.last(),
            Node(car, Value::Nil) => car.clone(),
            Node(_, cdr) => cdr.clone(),
        }
    }

    pub fn len(&self) -> usize {
        let tail_len = match self.cdr() {
            Value::Nil => 0,
            Value::Pair(x) => x.len(),
            _ => 1,
        };
        tail_len + 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> NodeIter {
        NodeIter::new(Handle::new(self.clone()))
    }

    pub fn rev(i: &Value) -> Value {
        match i {
            Value::Nil => i.clone(),
            Value::Pair(v) => {
                let Node(car, cdr) = &*v.clone();
                let r = Node::cons(
                    Node::rev(cdr),
                    Value::Pair(Handle::new(Node::cons(car.clone(), Value::Nil))),
                );
                Value::Pair(Handle::new(r))
            }
            _ => i.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeExtend(pub Option<Node>);

impl NodeExtend {
    #[inline]
    pub fn into_value(self) -> Value {
        self.0.map_or(Value::Nil, |x| Value::Pair(Handle::new(x)))
    }
}

impl Into<Value> for NodeExtend {
    fn into(self) -> Value {
        self.into_value()
    }
}

impl NodeExtend {
    fn into_nodeiter(self) -> NodeIter {
        NodeIter::from(self)
    }
}

impl FromIterator<Value> for NodeExtend {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        if let Some(car) = iter.next() {
            let cdr = Self::from_iter(iter).into_value();
            NodeExtend(Some(Node(car, cdr)))
        } else {
            NodeExtend(None)
        }
    }
}

#[derive(Debug)]
pub struct NodeIter(pub RefCell<Option<Handle<Node>>>);

impl NodeIter {
    pub fn new(i: Handle<Node>) -> Self {
        NodeIter(RefCell::new(Some(i)))
    }
}

impl From<NodeExtend> for NodeIter {
    fn from(i: NodeExtend) -> Self {
        let r = i.0.map(Handle::new);
        NodeIter(RefCell::new(r))
    }
}

impl Iterator for NodeIter {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        let r = &*self.0.borrow().clone()?;
        match r.clone() {
            Node(car, Value::Pair(cdr)) => {
                self.0.replace(Some(cdr));
                Some(car)
            }
            Node(car, Value::Nil) => {
                self.0.replace(None);
                Some(car)
            }
            Node(_, _) => None,
        }
    }
}

pub type ListPia = Handle<Node>;

// #[derive(Debug, Clone, PartialEq)]
// pub struct List(pub ListPia);

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
    pub id: Handle<String>,
    pub line: usize,
    pub colum: usize,
    pub pos: usize,
    pub scope: RefCell<SymbolList>,
    // pub value: RefCell<Option<Value>>,
}

pub type SymbolList = VecDeque<Handle<Symbol>>;

unsafe impl Sync for Symbol {}
unsafe impl Send for Symbol {}

impl Symbol {
    pub fn new(i: &str) -> Self {
        Symbol {
            id: string_intern(i),
            line: 0,
            colum: 0,
            pos: 0,
            scope: RefCell::new(VecDeque::new()),
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
        std::fmt::Display::fmt(&self.id, f)
    }
}

impl Hash for Symbol {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
