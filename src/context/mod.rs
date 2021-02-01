pub mod default;
pub mod display;
pub mod find_name;
pub mod get_name;
pub mod logic_path;
pub mod partial_eq;

use std::{
    cell::RefCell,
    collections::{HashMap, LinkedList},
    sync::RwLock,
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
    pub name: Handle<Symbol>,
    pub from_module: Handle<Module>,
    pub pairs: Vec<(Value, Value)>,
}

#[derive(Debug)]
pub struct ProcessMacro {
    pub name: Handle<Symbol>,
    pub from_module: Handle<Module>,
    pub body: Handle<FunctionDef>,
}

#[derive(Debug, PartialEq)]
pub enum FunctionDef {
    UserFunction(UserFunctionDef),
    NativeFunction(NativeFunctionDef),
}

#[derive(Debug)]
pub struct UserFunctionDef {
    pub name: Handle<Symbol>,
    pub from_module: Handle<Module>,
    pub parent: Option<Handle<FunctionDef>>,
    pub sub_function: RwLock<HashMap<Handle<Symbol>, Handle<FunctionDef>>>,
    pub params: Vec<Handle<Symbol>>,
    pub body: Vec<Value>,
}

type NativeInterface = fn(Vec<Value>) -> CResult;

#[derive(Debug)]
pub struct NativeFunctionDef {
    pub name: Handle<Symbol>,
    pub from_module: Handle<Module>,
    // pub params: Option<Vec<Handle<Symbol>>>,
    pub is_pure: bool,
    pub body: NativeInterface,
}

#[derive(Debug)]
pub struct Module {
    pub name: Handle<Symbol>,
    pub parent: Option<Handle<Module>>,
    pub module_table: RwLock<HashMap<Handle<Symbol>, Handle<Module>>>,
    pub macro_table: RwLock<HashMap<Handle<Symbol>, Handle<MacroDef>>>,
    pub function_table: RwLock<HashMap<Handle<Symbol>, Handle<FunctionDef>>>,
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
