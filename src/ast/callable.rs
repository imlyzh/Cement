use sexpr_ir::gast::{constant::Constant, symbol::Symbol};

// use crate::runtime::NativeInterface;

use crate::runtime::value::Value;

use super::{Ast, Pair, Params};


/// Lambda(params, is_var_len, capture, body)
#[derive(Debug, Clone)]
pub struct Lambda(pub Vec<Symbol>, pub bool, pub Ast);


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
        let mut car = car.matchs(&i.0)?;
        let icdr = i.1.clone()?;
        let mut cdr = (*cdr).as_ref().and_then(|x| x.matchs(&icdr))?;
        car.append(&mut cdr);
        Some(car)
    }
    pub fn partial_matchs(&self, i: &Pair<Params<Result<Value, Ast>>>)
    -> Option<Vec<(Symbol, Result<Value, Ast>)>> {
        let Pair(car, cdr) = self;
        let mut car = car.partial_matchs(&i.0)?;
        let icdr = i.1.clone()?;
        let mut cdr = (*cdr).as_ref().and_then(|x| x.partial_matchs(&icdr))?;
        car.append(&mut cdr);
        Some(car)
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
    pub fn partial_matchs(&self, i: &Params<Result<Value, Ast>>)
    -> Option<Vec<(Symbol, Result<Value, Ast>)>> {
        match (self, i) {
            (Pattern::Ignore, _) => Some(vec![]),
            (Pattern::Const(c), Params::Value(Ok(Value::Const(c1)))) if c == c1 => Some(vec![]),
            (Pattern::Var(s), Params::Value(v)) => Some(vec![(s.clone(), v.clone())]),
            (Pattern::Var(s), Params::Pair(v)) => todo!(),
            (Pattern::Pair(p), Params::Pair(p1)) => p.partial_matchs(p1),
            _ => None,
        }
    }
}