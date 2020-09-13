use pest::iterators::Pair;
use std::collections::VecDeque;

#[derive(Debug, Clone, Default)]
pub struct Pos {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct SExpr(pub RSExpr, pub Pos);

impl SExpr {
    pub fn get_raw(&self) -> RSExpr {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub enum RSExpr {
    NonAtomic(List),
    Atomic(Atom),
}

impl RSExpr {
    pub fn get_atomic(&self) -> Option<Atom> {
        match self {
            RSExpr::Atomic(x) => Some(x.clone()),
            _ => None,
        }
    }
    pub fn get_non_atomic(&self) -> Option<List> {
        match self {
            RSExpr::NonAtomic(x) => Some(x.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct List(pub ListPia);

pub type ListPia = VecDeque<SExpr>;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
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

impl Atom {
    pub fn get_sym(&self) -> Option<String> {
        match self {
            Atom::Sym(s) => Some(s.clone()),
            _ => None,
        }
    }
    pub fn get_str(&self) -> Option<String> {
        match self {
            Atom::Str(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_num(&self) -> Option<String> {
        match self {
            Atom::Num(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_char(&self) -> Option<char> {
        match self {
            Atom::Char(s) => Some(*s),
            _ => None,
        }
    }

    pub fn get_bool(&self) -> Option<char> {
        match self {
            Atom::Char(s) => Some(*s),
            _ => None,
        }
    }
}

pub trait ParseFrom<T>
where
    Self: std::marker::Sized,
{
    fn parse(pair: Pair<T>) -> Self;
}
