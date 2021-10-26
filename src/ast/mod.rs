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
    Cond(Cond),
    Lets(Lets),
    Begin(Vec<Ast>),
    Lambda(Box<Callable>),
    Call(Call),
}


#[derive(Debug, Clone)]
pub struct Call(pub Box<Ast>, pub Pair<Ast>);

#[derive(Debug, Clone)]
pub struct Cond(pub Vec<(Ast, Ast)>, pub Option<Box<Ast>>);

#[derive(Debug, Clone)]
pub struct Lets(pub Vec<(Symbol, Ast)>, pub Box<Ast>);


// args Pattern

#[derive(Debug, Clone)]
pub struct Pair<T>(pub Box<T>, pub Option<Box<T>>);
