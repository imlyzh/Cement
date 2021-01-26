use std::{cell::RefCell, collections::{HashMap, LinkedList}, fmt::Display, sync::{Arc, Mutex}};

use crate::values::{Symbol, Value};


#[derive(Debug)]
pub struct RuntimeError ();

#[derive(Debug)]
pub struct CResult (pub Result<Value, RuntimeError>);

#[derive(Debug, PartialEq)]
pub struct MacroDef {
	name: Arc<Symbol>,
	body: Value,
}

#[derive(Debug)]
pub struct FunctionDef {
	id: Arc<Symbol>,
	from_module: Arc<Module>,
	parent: Arc<FunctionDef>,
	params: Vec<Arc<Symbol>>,
	body: Vec<Value>,
}

impl PartialEq for FunctionDef {
    fn eq(&self, other: &Self) -> bool {
		self.id == other.id &&
		self.from_module == other.from_module
    }
}

impl Display for FunctionDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#<function {}>", self.id.to_string())
    }
}

#[derive(Debug)]
pub struct Module {
	name: Arc<Symbol>,
	parent: Option<Arc<Module>>,
	module_table: Mutex<HashMap<Arc<Symbol>, Arc<Module>>>,
	macro_table: Mutex<HashMap<Arc<Symbol>, Arc<MacroDef>>>,
	function_table: Mutex<HashMap<Arc<Symbol>, Arc<FunctionDef>>>,
}

impl Module {
	pub fn new(name: Arc<Symbol>, parent: Option<Arc<Module>>) -> Self {
		Module {
		    name,
		    parent,
		    module_table: Mutex::new(HashMap::new()),
            macro_table: Mutex::new(HashMap::new()),
            function_table: Mutex::new(HashMap::new()),
		}
	}
}

impl Default for Module {
    fn default() -> Self {
		Self::new(Arc::new(Symbol::new("anonymous-module")), None)
    }
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#<function {}>", self.name.to_string())
    }
}


#[derive(Debug, Default)]
pub struct EnvContext {
	pub module_table: HashMap<Arc<Symbol>, Arc<Module>>,
}

#[derive(Debug, Default)]
pub struct ThreadContext {
	pub env_context: Arc<EnvContext>,
	pub frame_stack: RefCell<LinkedList<FunctionContext>>,
}


#[derive(Debug)]
pub struct FunctionContext {
	pub namespace: HashMap<Arc<Symbol>, Value>,
	pub funcinfo: Arc<FunctionDef>,
}