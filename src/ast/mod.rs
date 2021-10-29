pub mod callable;
pub mod macrodef;

use std::sync::Arc;

use sexpr_ir::gast::{constant::Constant, symbol::Symbol};

use self::{callable::Lambda, macrodef::Macro};
use super::runtime::value::Value;


#[derive(Debug, Clone)]
pub enum TopLevel {
    Import(Vec<Symbol>),
    FunctionDef(FunctionDef),
    // VarDef(Symbol, Constant),
    Macro(Macro),
}

#[derive(Debug, Clone)]
pub struct FunctionDef(pub Symbol, pub Lambda);

#[derive(Debug, Clone)]
pub enum Ast {
    Var(Symbol),
    Const(Constant),
    Value(Value),
    // If(Box<Ast>, Box<Ast>, Box<Ast>),
    Cond(Cond),
    Lets(Lets),
    Begin(Vec<Ast>),
    Lambda(Arc<Lambda>),
    Call(Call),
}

#[derive(Debug, Clone)]
pub struct Call(pub Box<Ast>, pub Vec<Ast>);

#[derive(Debug, Clone)]
pub enum Params<T> {
    Value(T),
    Pair(Pair<Params<T>>),
}

#[derive(Debug, Clone)]
pub struct Cond(
    pub Vec<(Ast, Ast)>,
    // pub Option<Box<Ast>>
);

#[derive(Debug, Clone)]
pub struct Lets(pub Vec<(Symbol, Ast)>, pub Box<Ast>);


// args Pattern

#[derive(Debug, Clone)]
pub struct Pair<T>(pub Box<T>, pub Option<Box<T>>);
