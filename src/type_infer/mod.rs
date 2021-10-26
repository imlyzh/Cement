pub mod types;

use std::{collections::HashSet, sync::Arc};

use sexpr_ir::gast::constant::Constant;

use crate::{ast::{Ast, Call, Cond, Lets, callable::{Callable, Lambda}}, env::Env};

use self::types::Type;



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
        }
    }
}

impl TypeInfer for Cond {
    fn type_infer(&self, env: Arc<Env<Type>>) -> Type {
        let Cond(cond_exprs, else_expr) = self;
        for i in cond_exprs.iter().map(|(x, _)|x) {
            i.type_infer(env.clone());
        }
        let mut rts: Vec<Type> = cond_exprs
            .iter()
            .map(|(_, x)|x.type_infer(env.clone()))
            .collect();
        else_expr
            .iter()
            .for_each(|x| rts.push(x.type_infer(env.clone())));
        // reduce rts
        todo!()
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
        let funtype = callee.type_infer(env);
        todo!()
    }
}

impl TypeInfer for Callable {
    fn type_infer(&self, env: Arc<Env<Type>>) -> Type {
        match self {
            Callable::Lambda(l) => l.type_infer(env),
            Callable::NativeInterface(n) => Type::Callable(n.type_.clone()),
        }
    }
}

impl TypeInfer for Lambda {
    fn type_infer(&self, env: Arc<Env<Type>>) -> Type {
        let Lambda(params, body) = self;
        let env = Arc::new(env.new_level());
        body.type_infer(env);
        todo!()
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