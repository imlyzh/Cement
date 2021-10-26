pub mod callable;
pub mod macrodef;

use sexpr_ir::gast::{constant::Constant, symbol::Symbol};

use self::{callable::{Callable, Lambda}, macrodef::Macro};


#[derive(Debug, Clone)]
pub enum TopLevel {
    Import(Vec<Symbol>),
    FunctionDef(FunctionDef),
    Macro(Macro),
}

#[derive(Debug, Clone)]
pub struct FunctionDef(pub Symbol, pub Lambda);

#[derive(Debug, Clone)]
pub enum Ast {
    Var(Symbol),
    Const(Constant),
    // If(Box<Ast>, Box<Ast>, Box<Ast>),
    Cond(Vec<(Ast, Ast)>, Option<Box<Ast>>),
    Let(Vec<(Symbol, Ast)>, Box<Ast>),
    Begin(Vec<Ast>),
    Lambda(Box<Callable>),
    Call(Box<Ast>, Pair<Ast>),
}

// args Pattern

#[derive(Debug, Clone)]
pub struct Pair<T>(pub Box<T>, pub Pattern<T>);

#[derive(Debug, Clone)]
pub enum Pattern<T> {
    Ignore,
    Const(Constant),
    Pair(Box<Pair<T>>),
}
