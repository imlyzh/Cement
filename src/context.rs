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
	logic_path: Arc<Symbol>,
	parent: Option<Arc<Module>>,
	macro_table: Mutex<HashMap<Arc<Symbol>, Arc<MacroDef>>>,
	function_table: Mutex<HashMap<Arc<Symbol>, Arc<FunctionDef>>>,
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.logic_path == other.logic_path
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#<function {}>", self.logic_path.to_string())
    }
}

impl Default for Module {
    fn default() -> Self {
        Module {
			logic_path: Arc::new(Symbol::new("anonymous-module")),
			parent: None,
            macro_table: Mutex::new(HashMap::new()),
            function_table: Mutex::new(HashMap::new()),
		}
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