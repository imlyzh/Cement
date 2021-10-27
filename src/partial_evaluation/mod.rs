use std::sync::Arc;

use sexpr_ir::gast::{constant::Constant, symbol::Symbol};

use crate::{ast::{Ast, Call, Cond, Lets, Pair, Params}, runtime::{NameSpace, value::Value}};



pub trait PartialEval {
    fn partial_eval(&self, env: Arc<NameSpace>) -> Result<Value, Ast>;
}

impl PartialEval for Ast {
    fn partial_eval(&self, env: Arc<NameSpace>) -> Result<Value, Ast> {
        match self {
            Ast::Value(v) => Ok(v.clone()),
            Ast::Const(v) => Ok(Value::Const(v.clone())),
            Ast::Var(k) => env.get_item(k).ok_or(self.clone()),
            Ast::Lambda(l) => Ok(Value::Closure(l.clone(), env.clone())),
            Ast::Begin(b) => {
                if b.len() == 0 {
                    Ok(Value::Const(Constant::Nil))
                } else if b.len() == 1 {
                    b.get(0).unwrap().partial_eval(env)
                } else {
                    let mut r: Vec<Ast> = b[..b.len()-1]
                        .iter()
                        .map(|x| x.partial_eval(env.clone()))
                        .filter(|x| x.is_err())
                        .map(|x| x.unwrap_err())
                        .collect();
                    let e = b.last().unwrap().partial_eval(env);
                    if r.is_empty() {
                        e
                    } else {
                        let e = result2ast(e);
                        r.push(e);
                        Err(Ast::Begin(r))
                    }
                }
            }
            Ast::Cond(c) => c.partial_eval(env),
            Ast::Lets(l) => l.partial_eval(env),
            Ast::Call(c) => c.partial_eval(env),
        }
    }
}

impl PartialEval for Cond {
    fn partial_eval(&self, env: Arc<NameSpace>) -> Result<Value, Ast> {
        let Cond(pairs,
            // else_expr
        ) = self;
        let pairs: Vec<_> = pairs
            .iter()
            .map(|(k, v)| (k.partial_eval(env.clone()), v))
            .collect();
        let pe_count = pairs
            .iter()
            .filter(|(c, _)| c.is_err())
            .count();
        if pe_count == 0 {
            for (c, v) in pairs.clone() {
                if let Value::Const(Constant::Bool(true)) = c.unwrap() {
                    return v.partial_eval(env);
                }
            }
        }
        let pairs: Vec<_> = pairs
            .iter()
            .map(|(k, v)| (result2ast(k.clone()), result2ast(v.partial_eval(env.clone()))))
            .collect();
            Err(Ast::Cond(Cond(pairs)))
    }
}

impl PartialEval for Lets {
    fn partial_eval(&self, env: Arc<NameSpace>) -> Result<Value, Ast> {
        let env = Arc::new(env.new_level());
        let Lets(ls, e) = self;
        let eval_items: Vec<(&Symbol, Result<Value, Ast>)> = ls
            .iter()
            .map(|(k, v)| (k, v.partial_eval(env.clone())))
            .collect();
        let mut let_item = vec![];
        for (k, v) in eval_items {
            match v {
                Ok(v) => env.add(k.clone(), v),
                Err(v) => let_item.push((k.clone(), v)),
            }
        }
        let e = e.partial_eval(env);
        if let_item.is_empty() {
            e
        } else {
            let e = result2ast(e);
            let r = Ast::Lets(Lets(let_item, Box::new(e)));
            Err(r)
        }
    }
}

impl PartialEval for Call {
    fn partial_eval(&self, env: Arc<NameSpace>) -> Result<Value, Ast> {
        let Call(callee, params) = self;
        let callee = callee.partial_eval(env.clone());
        let params = params.partial_eval(env);
        todo!()
    }
}

impl PartialEval for Pair<Params<Ast>> {
    fn partial_eval(&self, env: Arc<NameSpace>) -> Result<Value, Ast> {
        let Pair(car, cdr) = self;
        // car.partial_eval(env)
        todo!()
    }
}

impl PartialEval for Params<Ast> {
    fn partial_eval(&self, env: Arc<NameSpace>) -> Result<Value, Ast> {
        todo!()
    }
}


#[inline]
fn result2ast(e: Result<Value, Ast>) -> Ast {
    match e {
        Ok(v) => Ast::Value(v),
        Err(v) => v,
    }
}
