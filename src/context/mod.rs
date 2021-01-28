pub mod display;
pub mod logic_path;
pub mod partial_eq;

use std::{
    cell::RefCell,
    collections::{HashMap, LinkedList},
    sync::{Arc, Mutex},
};

use crate::preprocess::symbols::*;
use crate::values::{Symbol, Value};

#[derive(Debug)]
pub struct RuntimeError();

#[derive(Debug)]
pub struct CResult(pub Result<Value, RuntimeError>);

#[derive(Debug, PartialEq)]
pub enum MacroDef {
    TempMacro(TempMacro),
    ProcessMacro(ProcessMacro),
}

#[derive(Debug)]
pub struct TempMacro {
    name: Arc<Symbol>,
    from_module: Arc<Module>,
    pairs: Vec<(Value, Value)>,
}

#[derive(Debug)]
pub struct ProcessMacro {
    name: Arc<Symbol>,
    from_module: Arc<Module>,
    body: Arc<FunctionDef>,
}

#[derive(Debug, PartialEq)]
pub enum FunctionDef {
    UserFunction(UserFunctionDef),
    NativeFunction(NativeFunctionDef),
}

#[derive(Debug)]
pub struct UserFunctionDef {
    name: Arc<Symbol>,
    from_module: Arc<Module>,
    parent: Option<Arc<FunctionDef>>,
    params: Vec<Arc<Symbol>>,
    body: Vec<Value>,
}

#[derive(Debug)]
pub struct NativeFunctionDef {
    name: Arc<Symbol>,
    from_module: Arc<Module>,
    params: Option<Vec<Arc<Symbol>>>,
    is_pure: bool,
    body: extern "C" fn(Vec<Value>) -> CResult,
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
        Self::new(ANONYMOUS_MODULE_NAME.clone(), None)
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
