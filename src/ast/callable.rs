use sexpr_ir::gast::symbol::Symbol;

use crate::type_infer::types::CallableType;

use super::{Ast, Pair};


#[derive(Debug, Clone)]
pub enum Callable {
    Lambda(Lambda),
    NativeInterface(NativeInterface),
}


#[derive(Debug, Clone)]
pub struct Lambda(pub Pair<Symbol>, pub Ast);


#[derive(Debug, Clone)]
pub struct NativeInterface {
    pub ptr: RNI,
    pub is_pure: bool,
    pub type_: CallableType,
}

type RNI = ();
// type CNI = ();
