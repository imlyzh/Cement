pub mod default;
pub mod display;
pub mod logic_path;
pub mod partial_eq;

use std::{
    cell::RefCell,
    collections::{HashMap, LinkedList},
    sync::Mutex,
};

use crate::preprocess::symbols::*;
use crate::values::{Handle, Symbol, Value};

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
    name: Handle<Symbol>,
    from_module: Handle<Module>,
    pairs: Handle<(Value, Value)>,
}

#[derive(Debug)]
pub struct ProcessMacro {
    name: Handle<Symbol>,
    from_module: Handle<Module>,
    body: Handle<FunctionDef>,
}

#[derive(Debug, PartialEq)]
pub enum FunctionDef {
    UserFunction(UserFunctionDef),
    NativeFunction(NativeFunctionDef),
}

#[derive(Debug)]
pub struct UserFunctionDef {
    name: Handle<Symbol>,
    from_module: Handle<Module>,
    parent: Option<Handle<FunctionDef>>,
    params: Vec<Handle<Symbol>>,
    body: Vec<Value>,
}

type NativeInterface = fn(Vec<Value>) -> CResult;

#[derive(Debug)]
pub struct NativeFunctionDef {
    name: Handle<Symbol>,
    from_module: Handle<Module>,
    params: Option<Vec<Handle<Symbol>>>,
    is_pure: bool,
    body: NativeInterface,
}

#[derive(Debug)]
pub struct Module {
    name: Handle<Symbol>,
    parent: Option<Handle<Module>>,
    module_table: Mutex<HashMap<Handle<Symbol>, Handle<Module>>>,
    macro_table: Mutex<HashMap<Handle<Symbol>, Handle<MacroDef>>>,
    function_table: Mutex<HashMap<Handle<Symbol>, Handle<FunctionDef>>>,
}

#[derive(Debug, Default)]
pub struct EnvContext {
    pub module_table: HashMap<Handle<Symbol>, Handle<Module>>,
}

#[derive(Debug, Default)]
pub struct ThreadContext {
    pub env_context: Handle<EnvContext>,
    pub frame_stack: RefCell<LinkedList<FunctionContext>>,
}

#[derive(Debug)]
pub struct FunctionContext {
    pub namespace: HashMap<Handle<Symbol>, Value>,
    pub funcinfo: Handle<FunctionDef>,
}
