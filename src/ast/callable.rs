use sexpr_ir::gast::{constant::Constant, symbol::Symbol};

use crate::type_infer::types::CallableType;

use super::{Ast, Pair};



#[derive(Debug, Clone)]
pub enum Callable {
    Lambda(Lambda),
    NativeInterface(NativeInterface),
}


#[derive(Debug, Clone)]
pub struct Lambda(pub Pair<Pattern>, pub Ast);


#[derive(Debug, Clone)]
pub enum Pattern {
    Ignore,
    Const(Constant),
    Var(Symbol),
    Pair(Box<Pair<Pattern>>),
}



#[derive(Debug, Clone)]
pub struct NativeInterface {
    pub ptr: RNI,
    pub is_pure: bool,
    pub type_: CallableType,
}

type RNI = ();
// type CNI = ();
