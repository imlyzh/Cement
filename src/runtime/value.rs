
use std::sync::Arc;

use sexpr_ir::gast::constant::Constant;

use crate::{ast::{Pair, callable::Lambda}, type_infer::types::CallableType};

use super::NameSpace;


#[derive(Debug, Clone)]
pub enum Value {
    Const(Constant),
    Pair(Pair<Value>),
    Closure(Arc<Lambda>, Arc<NameSpace>),
}


#[derive(Debug, Clone)]
pub struct NativeInterface {
    pub ptr: HLNI,
    pub is_pure: bool,
    pub type_: CallableType,
}


// low level native interface
type LLNI = fn(); // todo

// high level native interface
type HLNI = fn(Value) -> Result<Value, ()>;
