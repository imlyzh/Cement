
use std::sync::Arc;

use sexpr_ir::gast::{constant::Constant, symbol::Symbol};

use crate::{ast::{Ast, Pair, Params, callable::Lambda}, type_infer::types::CallableType};

use super::NameSpace;


#[derive(Debug, Clone)]
pub enum Value {
    Const(Constant),
    Pair(Arc<Value>, Arc<Value>),
    Closure(Arc<Lambda>, Arc<NameSpace>),
    NativeInterface(NativeInterface)
}


#[derive(Debug, Clone)]
pub struct NativeInterface {
    pub ptr: HLNI,
    pub pe: Option<PENI>,
    pub name: Symbol,
    pub is_pure: bool,
    pub type_: CallableType,
}


// low level native interface
// type LLNI = fn(); // todo

// high level native interface
type HLNI = fn(Vec<Value>) -> Result<Value, ()>;

// partial eval native interface
type PENI = fn(Vec<Result<Value, Ast>>) -> Result<Value, Ast>;