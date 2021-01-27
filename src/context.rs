use std::{
    cell::RefCell,
    collections::{HashMap, LinkedList},
    fmt::Display,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;

use crate::values::{Symbol, Value};

macro_rules! impl_partial_eq {
    ($tp:path) => {
        impl PartialEq for $tp {
            fn eq(&self, other: &Self) -> bool {
                self.name == other.name && self.from_module == other.from_module
            }
        }
    };
}

#[derive(Debug)]
pub struct RuntimeError();

#[derive(Debug)]
pub struct CResult(pub Result<Value, RuntimeError>);

#[derive(Debug, PartialEq)]
pub struct MacroDef {
    name: Arc<Symbol>,
    body: Value,
}

#[derive(Debug)]
pub struct TempMacro {
    name: Arc<Symbol>,
    from_module: Arc<Module>,
    pairs: Vec<(Value, Value)>,
}

impl_partial_eq!(TempMacro);

#[derive(Debug)]
pub struct ProcessMacro {
    name: Arc<Symbol>,
    from_module: Arc<Module>,
    body: Arc<FunctionDef>,
}

impl_partial_eq!(ProcessMacro);

#[derive(Debug, PartialEq)]
pub enum FunctionDef {
    UserFunction(UserFunctionDef),
    NativeFunction(NativeFunctionDef),
}

#[derive(Debug)]
pub struct UserFunctionDef {
    name: Arc<Symbol>,
    from_module: Arc<Module>,
    parent: Arc<FunctionDef>,
    params: Vec<Arc<Symbol>>,
    body: Vec<Value>,
}

impl_partial_eq!(UserFunctionDef);

impl Display for UserFunctionDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#<function {:?}.{}>",
            todo!("module name"),
            self.name.to_string()
        )
    }
}

#[derive(Debug)]
pub struct NativeFunctionDef {
    name: Arc<Symbol>,
    from_module: Arc<Module>,
    parent: Arc<FunctionDef>,
    params: Option<Vec<Arc<Symbol>>>,
    is_pure: bool,
    body: extern "C" fn(Vec<Value>) -> CResult,
}

impl_partial_eq!(NativeFunctionDef);

impl Display for NativeFunctionDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#<function {:?}.{}>",
            todo!("module name"),
            self.name.to_string()
        )
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

lazy_static! {
    pub static ref ANONYMOUS_MODULE_NAME: Arc<Symbol> = Arc::new(Symbol::new("anonymous-module"));
}

impl Default for Module {
    fn default() -> Self {
        Self::new(Arc::new(Symbol::new("anonymous-module")), None)
    }
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.parent == other.parent
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
