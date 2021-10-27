use sexpr_ir::gast::{constant::Constant, symbol::Symbol};

// use crate::runtime::NativeInterface;

use crate::runtime::value::Value;

use super::{Ast, Pair, Params};


#[derive(Debug, Clone)]
pub struct Lambda(pub Pair<Pattern>, pub Ast);


#[derive(Debug, Clone)]
pub enum Pattern {
    Ignore,
    Const(Constant),
    Var(Symbol),
    Pair(Box<Pair<Pattern>>),
}


impl Pair<Pattern> {
    pub fn matchs(&self, i: &Pair<Params<Value>>) -> Option<Vec<(Symbol, Value)>> {
        let Pair(car, cdr) = self;

        todo!()
    }
}

impl Pattern {
    pub fn matchs(&self, i: &Params<Value>) -> Option<Vec<(Symbol, Value)>> {
        match (self, i) {
            (Pattern::Ignore, _) => Some(vec![]),
            (Pattern::Const(c), Params::Value(Value::Const(c1))) if c == c1 => Some(vec![]),
            (Pattern::Var(s), Params::Value(v)) => Some(vec![(s.clone(), v.clone())]),
            (Pattern::Var(s), Params::Pair(v)) => todo!(),
            (Pattern::Pair(p), Params::Pair(p1)) => p.matchs(p1),
            _ => None,
        }
    }
}