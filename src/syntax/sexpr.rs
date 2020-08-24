use std::collections::VecDeque;
use pest::iterators::Pair;

#[derive(Debug, Clone, Default)]
pub struct Pos {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct SExpr (pub RSExpr, pub Pos);

#[derive(Debug, Clone)]
pub enum RSExpr {
    NonAtomic(List),
    Atomic(Atom),
}

#[derive(Debug, Clone)]
pub struct List (pub ListPia);

pub type ListPia = VecDeque<SExpr>;

#[derive(Debug, Clone)]
pub enum Atom {
    // Int(&'a str),
    // Float(&'a str),
    // Fraction(&'a str),
    Bool(bool),
    Char(char),
    Num(String),
    Str(String),
    Sym(String),
}

pub trait ParseFrom<T>
    where
        Self: std::marker::Sized,
{
    fn parse(pair: Pair<T>) -> Self;
}