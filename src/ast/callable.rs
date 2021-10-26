use sexpr_ir::gast::{constant::Constant, symbol::Symbol};

// use crate::runtime::NativeInterface;

use super::{Ast, Pair};


#[derive(Debug, Clone)]
pub struct Lambda(pub Pair<Pattern>, pub Ast);


#[derive(Debug, Clone)]
pub enum Pattern {
    Ignore,
    Const(Constant),
    Var(Symbol),
    Pair(Box<Pair<Pattern>>),
}
