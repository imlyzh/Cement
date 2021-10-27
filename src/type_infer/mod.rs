pub mod types;

use std::sync::Arc;

use sexpr_ir::gast::constant::Constant;

use crate::{ast::{Ast, Call, Cond, Lets, Pair, Params, callable::{Lambda, Pattern}}, env::Env};

use self::types::{CallableType, Type};



pub trait TypeInfer {
    fn type_infer(&self, env: Arc<Env<Type>>) -> Type;
}

impl TypeInfer for Ast {
    fn type_infer(&self, env: Arc<Env<Type>>) -> Type {
        match self {
            Ast::Var(s) => env.get_item(s).map_or(Type::Any, |v| v.clone()),
            Ast::Const(c) => c.type_infer(env),
            Ast::Cond(c) => c.type_infer(env),
            Ast::Lets(l) => l.type_infer(env),
            Ast::Begin(e) => e.iter().map(|x| x.type_infer(env.clone())).last().unwrap_or(Type::Nil),
            Ast::Lambda(c) => c.type_infer(env),
            Ast::Call(c) => c.type_infer(env),
            Ast::Value(_) => todo!("runtime value is not support"),
        }
    }
}

impl TypeInfer for Cond {
    fn type_infer(&self, env: Arc<Env<Type>>) -> Type {
        let Cond(cond_exprs,
            // else_expr
        ) = self;
        for i in cond_exprs.iter().map(|(x, _)|x) {
            i.type_infer(env.clone());
        }
        let mut rts: Vec<Type> = cond_exprs
            .iter()
            .map(|(_, x)|x.type_infer(env.clone()))
            .collect();
        /*
        else_expr
            .iter()
            .for_each(|x| rts.push(x.type_infer(env.clone())));
        // */
        // reduce rts
        rts.into_iter().reduce(Type::union).unwrap_or(Type::Nil)
    }
}

impl TypeInfer for Lets {
    fn type_infer(&self, env: Arc<Env<Type>>) -> Type {
        let Lets(vs, e) = self;
        let env = Arc::new(env.new_level());
        for (k, v) in vs {
            env.add(k.clone(), v.type_infer(env.clone()));
        }
        e.type_infer(env)
    }
}

impl TypeInfer for Call {
    fn type_infer(&self, env: Arc<Env<Type>>) -> Type {
        let Call(callee, params) = self;
        let funtype = callee.type_infer(env.clone());
        let paramstype = params.type_infer(env);
        if let Type::Callable(CallableType(p, t)) = funtype {
            // p and params type union
            *t
        } else {
            panic!("error") // todo
        }
    }
}

impl TypeInfer for Params<Ast> {
    fn type_infer(&self, env: Arc<Env<Type>>) -> Type {
        match self {
            Params::Value(v) => v.type_infer(env),
            Params::Pair(p) => p.type_infer(env),
        }
    }
}

/*
impl TypeInfer for Callable {
    fn type_infer(&self, env: Arc<Env<Type>>) -> Type {
        match self {
            Lambda::Lambda(l) => l.type_infer(env),
            Lambda::NativeInterface(n) => Type::Callable(n.type_.clone()),
        }
    }
}
*/

impl TypeInfer for Lambda {
    fn type_infer(&self, env: Arc<Env<Type>>) -> Type {
        let Lambda(params, body) = self;
        let env = Arc::new(env.new_level());
        let return_type = Box::new(body.type_infer(env.clone()));
        let args_type = Box::new(params.type_infer(env));
        Type::Callable(CallableType(args_type, return_type))
    }
}

impl TypeInfer for Pattern {
    fn type_infer(&self, env: Arc<Env<Type>>) -> Type {
        match self {
            Pattern::Ignore => Type::Any,
            Pattern::Const(c) => c.type_infer(env),
            Pattern::Var(s) => {
                // env.add(s.clone(), Type::Any);
                Type::Any
            },
            Pattern::Pair(p) => p.type_infer(env),
        }
    }
}

impl<T: TypeInfer> TypeInfer for Pair<T> {
    fn type_infer(&self, env: Arc<Env<Type>>) -> Type {
        let Pair(car, cdr) = self;
        let car = Box::new(car.type_infer(env.clone()));
        let cdr = Box::new(cdr.as_ref()
            .map(|x| x.type_infer(env))
            .unwrap_or(Type::Nil));
        Type::Pair(car, cdr)
    }
}

impl TypeInfer for Constant {
    fn type_infer(&self, _env: Arc<Env<Type>>) -> Type {
        match self {
            Constant::Nil => Type::Nil,
            Constant::Bool(_) => Type::Bool,
            // Constant::Char(_) => todo!(),
            Constant::Int(_) => Type::Int,
            Constant::Uint(_) => Type::Uint,
            Constant::Float(_) => Type::Float,
            Constant::Str(_) => Type::String,
            Constant::Sym(_) => Type::Symbol,
            _ => panic!("unsupported type"),
        }
    }
}