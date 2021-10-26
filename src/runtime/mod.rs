pub mod value;
pub mod native_value;

use std::{cell::RefCell, collections::HashMap, sync::{Arc, RwLock}};

use sexpr_ir::gast::symbol::Symbol;

use crate::{ast::FunctionDef, env::Env, type_infer::types::{CallableType, Type}};

use self::value::Value;


pub type NameSpace = Env<Value>;


#[derive(Debug, Clone, Default)]
pub struct GlobalEnv {
    values: Arc<RwLock<HashMap<Symbol, RefCell<Vec<FunctionDef>>>>>,
    compile_types: Arc<RwLock<HashMap<Symbol, CallableType>>>,
}

impl GlobalEnv {
    pub fn type_infer_env(&self) -> Env<Type> {
        todo!()
    }
}
