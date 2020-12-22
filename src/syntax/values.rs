use pest::iterators::Pair;
use std::collections::VecDeque;

/*
#[derive(Debug, Clone, Default)]
pub struct Pos {
    pub start: usize,
    pub end: usize,
}
*/

#[derive(Debug, Clone)]
pub struct SExpr(pub RSExpr);

impl std::fmt::Display for SExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}


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

impl std::fmt::Display for RSExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RSExpr::Atomic(a) => a.fmt(f),
            RSExpr::NonAtomic(l) => l.fmt(f)
        }
    }
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

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list = self.0.iter();
        let str_list: Vec<String> = list.map(SExpr::to_string).collect();
        let retstr = format!("({})", str_list.join(" "));
        f.write_str(&retstr)
    }
}

pub type ListPia = VecDeque<SExpr>;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Atom {
    // Int(String),
    // Float(String),
    // Fraction(String),
    Bool(bool),
    Char(char),
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(String),
    Sym(String),
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Bool(v) => f.write_str(&v.to_string()),
            Atom::Char(v) => f.write_str(&v.to_string()),
            Atom::Int(v) => f.write_str(&v.to_string()),
            Atom::Uint(v) => f.write_str(&v.to_string()),
            Atom::Float(v) => f.write_str(&v.to_string()),
            Atom::Str(v) => f.write_str(&v.to_string()),
            Atom::Sym(v) => f.write_str(&v.to_string()),
        }
    }
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

    /*pub fn get_num(&self) -> Option<String> {
        match self {
            Atom::Num(s) => Some(s.clone()),
            _ => None,
        }
    }*/

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
