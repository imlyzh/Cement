use sexpr_ir::gast::symbol::Symbol;

use super::{Ast, Pair};


#[derive(Debug, Clone)]
pub enum Callable {
    Lambda(Lambda),
    NativeInterface(NativeInterface),
}


#[derive(Debug, Clone)]
pub struct Lambda(pub Pair<Symbol>, pub Ast);


#[derive(Debug, Clone)]
pub enum NativeInterface {
    Rust(RNI),
    C(CNI),
}

type RNI = ();
type CNI = ();
