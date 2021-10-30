use std::{
    collections::HashMap,
    slice::{Iter, SliceIndex},
    sync::Arc,
};

use sexpr_ir::gast::{constant::Constant, symbol::Symbol};

use crate::{
    ast::{callable::Lambda, Ast, Call},
    env::Env,
    partial_evaluation::result2ast,
    runtime::{
        value::{NativeInterface, Value},
        NameSpace,
    },
};

use super::PartialEval;

pub trait PartialCall {
    fn partial_call(
        self: Arc<Self>,
        env: Arc<NameSpace>,
        params: Vec<Result<Value, Ast>>,
    ) -> Result<Value, Ast>;
}

impl PartialCall for Lambda {
    fn partial_call(
        self: Arc<Self>,
        env: Arc<NameSpace>,
        params: Vec<Result<Value, Ast>>,
    ) -> Result<Value, Ast> {
        // if params len error
        if params.len() != self.0.len() {
            return Err(Ast::Call(Call(
                Box::new(Ast::Var(Symbol::new("type_error"))),
                vec![Ast::Const(Constant::Str(Arc::new("invilid function call params length".to_string())))]
            )))
        }
        let env = Arc::new(env.new_level());
        let p = params_collect(&self.0, &params);
        let partial_eval_params: Vec<(Symbol, Ast)> = if self.0.len() == params.len() {
            params.iter()
        } else {
            params[..self.0.len()].iter()
        }
        .enumerate()
        .filter(|(_, x)| x.is_err())
        .map(|(u, x)| (self.0.get(u).unwrap().clone(), x.clone().unwrap_err()))
        .collect();
        let env = Arc::new(Env::from(p, env));
        match self.1.partial_eval(env) {
            Ok(r) => Ok(r),
            Err(e) => {
                let mut params_table: Vec<Symbol> = partial_eval_params.iter().map(|(k, _)| k.clone()).collect();
                let mut params_body: Vec<Ast> = partial_eval_params.into_iter().map(|(_, v)| v).collect();
                let fun = Lambda(params_table, e);
                Err(Ast::Call(Call(
                    Box::new(Ast::Lambda(Arc::new(fun))),
                    params_body,
                )))
            }
        }
    }
}

pub fn params_collect(
    p: &Vec<Symbol>,
    params: &Vec<Result<Value, Ast>>,
) -> HashMap<Symbol, Value> {
    if p.is_empty() && params.is_empty() {
        return HashMap::new();
    }
    if params.len() != p.len() {
        panic!("prarms is invilid");
    }
    let r: HashMap<Symbol, Value> = p
        .into_iter()
        .zip(params.into_iter())
        .filter(|(_, y)| y.is_ok())
        .map(|(x, y)| (x.clone(), y.clone().unwrap().clone()))
        .collect();
    r
}

fn collect_extend_params(ep: &mut Iter<Result<Value, Ast>>) -> Option<Value> {
    let i1 = ep.next();
    let i1 = match i1 {
        Some(Ok(v)) => v,
        Some(Err(_)) => return None,
        None => return Some(Value::Const(Constant::Nil)),
    };
    let i2 = collect_extend_params(ep)?;
    Some(Value::Pair(Arc::new(i1.clone()), Arc::new(i2)))
}

impl PartialCall for NativeInterface {
    fn partial_call(
        self: Arc<Self>,
        _env: Arc<NameSpace>,
        params: Vec<Result<Value, Ast>>,
    ) -> Result<Value, Ast> {
        (self.pe.unwrap())(params)
    }
}
