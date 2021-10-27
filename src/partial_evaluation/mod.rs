pub mod call;

use std::sync::Arc;

use sexpr_ir::gast::{constant::Constant, symbol::Symbol};

use crate::{ast::{Ast, Call, Cond, Lets, Pair, Params}, partial_evaluation::call::PartialCall, runtime::{NameSpace, value::Value}};



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
        match callee {
            Ok(v) => if let Value::Closure(c, ns) = v {
                c.partial_call(ns, params)
            } else if let Value::NativeInterface(ni) = v.clone() {
                if ni.is_pure {
                    let p = params.clone().to_value();
                    if let Some(p) = p {
                        // Degenerate to normal eval/apply
                        todo!("get params")
                        // (ni.ptr)(p)
                    } else if ni.pe.is_some() {
                        return ni.partial_call(Arc::new(NameSpace::default()), params);
                    }
                }
                Err(Ast::Call(Call(Box::new(Ast::Value(v)), params.to_ast())))
            } else {
                Err(Ast::Call(Call(Box::new(Ast::Value(v)), params.to_ast())))
            },
            Err(f) => Err(Ast::Call(Call(Box::new(f), params.to_ast()))),
        }
    }
}

impl Pair<Params<Ast>> {
    fn partial_eval(&self, env: Arc<NameSpace>) -> Pair<Params<Result<Value, Ast>>> {
        let Pair(car, cdr) = self;
        let car = car.partial_eval(env.clone());
        let cdr = cdr.as_ref().map(|cdr| Box::new(cdr.partial_eval(env.clone())));
        Pair(Box::new(car), cdr)
    }
}

impl Params<Ast> {
    fn partial_eval(&self, env: Arc<NameSpace>) -> Params<Result<Value, Ast>> {
        match self {
            Params::Value(v) => Params::Value(v.partial_eval(env)),
            Params::Pair(v) => Params::Pair(v.partial_eval(env)),
        }
    }
}

impl Pair<Params<Result<Value, Ast>>> {
    pub fn to_ast(self) -> Pair<Params<Ast>> {
        let Pair(car, cdr) = self;
        Pair(Box::new(car.to_ast()), cdr.map(|x| Box::new(x.to_ast())))
    }

    pub fn to_value(self) -> Option<Pair<Params<Value>>> {
        let Pair(car, cdr) = self;
        let car = car.to_value()?;
        let cdr = cdr.map(|x| x.to_value())?;
        Some(Pair(Box::new(car), cdr.map(|x| Box::new(x))))
    }
}

impl Params<Result<Value, Ast>> {
    pub fn to_ast(self) -> Params<Ast> {
        match self {
            Params::Value(v) => Params::Value(result2ast(v)),
            Params::Pair(v) => Params::Pair(v.to_ast()),
        }
    }

    pub fn to_value(self) -> Option<Params<Value>> {
        match self {
            Params::Value(v) => v.ok().map(Params::Value),
            Params::Pair(v) => Some(Params::Pair(v.to_value()?)),
        }
    }
}


#[inline]
fn result2ast(e: Result<Value, Ast>) -> Ast {
    match e {
        Ok(v) => Ast::Value(v),
        Err(v) => v,
    }
}
