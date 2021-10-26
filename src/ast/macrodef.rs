use sexpr_ir::gast::symbol::Symbol;

use super::{Pair, callable::Lambda};



#[derive(Debug, Clone)]
pub enum Macro {
    Temp(Symbol, TempMacro),
    Proc(Symbol, Lambda),
}


#[derive(Debug, Clone)]
pub struct TempMacro(pub Pair<Symbol>, ); // todo
