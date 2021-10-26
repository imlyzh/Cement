pub mod types;

use std::sync::Arc;

use sexpr_ir::gast::constant::Constant;

use crate::{ast::{Ast, callable::Lambda}, env::Env};

use self::types::Type;



pub trait TypeInfer {
    fn type_infer(&self, env: Arc<Env<Type>>) -> Type;
}

impl TypeInfer for Ast {
    fn type_infer(&self, env: Arc<Env<Type>>) -> Type {
        match self {
            Ast::Var(s) => env.get_item(s).map_or(Type::Any, |v| v.clone()),
            Ast::Const(c) => c.type_infer(env),
            Ast::Cond(cs, e) => todo!(),
            Ast::Lets(vs, e) => todo!(),
            Ast::Begin(e) => e.iter().map(|x| x.type_infer(env.clone())).last().unwrap_or(Type::Nil),
            Ast::Lambda(c) => todo!(),
            Ast::Call(callee, args) => todo!(),
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