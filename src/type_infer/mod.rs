use std::collections::HashSet;

use crate::ast::Ast;



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    None,
    Int,
    Float,
    String,
    Symbol,
    Vector(Box<Type>),
    Pair(Box<(Type, Type)>),
    List(Box<Type>),
    Union(Vec<Type>),
    Any,
}

pub trait TypeInfer {
    fn type_infer(&self) -> Type;
}

impl TypeInfer for Ast {
    fn type_infer(&self) -> Type {
        match self {
            Ast::Var(s) => todo!(),
            Ast::Const(c) => todo!(),
            Ast::Cond(cs, e) => todo!(),
            Ast::Let(vs, e) => todo!(),
            Ast::Begin(e) => todo!(),
            Ast::Lambda(c) => todo!(),
            Ast::Call(callee, args) => todo!(),
        }
    }
}
