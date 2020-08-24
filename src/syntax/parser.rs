use super::sexpr::*;
use crate::syntax::utils::{escape_str, register_intern_str, str2char};
use pest::iterators::Pair;
use pest_derive::*;
use std::collections::VecDeque;
use std::io::Chain;
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "./syntax/grammar.pest"]
struct Cement {}

impl ParseFrom<Rule> for SExpr {
    fn parse(pair: Pair<Rule>) -> Self {
        let rs_expr = match pair.as_rule() {
            Rule::list => RSExpr::NonAtomic(List::parse(pair.clone().into_inner().next().unwrap())),
            Rule::atomic => RSExpr::Atomic(Atom::parse(pair.clone().into_inner().next().unwrap())),
            _ => unreachable!(),
        };
        let pos = Pos {
            start: pair.as_span().start(),
            end: pair.as_span().end(),
        };
        SExpr(rs_expr, pos)
    }
}

impl ParseFrom<Rule> for List {
    fn parse(pair: Pair<Rule>) -> Self {
        let lst: ListPia = pair.into_inner().map(SExpr::parse).collect();
        List(lst)
    }
}

impl ParseFrom<Rule> for Atom {
    fn parse(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::bool_lit => Atom::Bool(bool::from_str(pair.as_str()).unwrap()),
            Rule::char_lit => Atom::Char(str2char(&escape_str(pair.as_str()))),
            Rule::string_lit => Atom::Str(escape_str(pair.as_str())),
            Rule::number_lit => Atom::Num(pair.as_str().to_string()),
            Rule::symbol => Atom::Sym(register_intern_str(pair.as_str())),
            _ => unreachable!(),
        }
    }
}

fn parser<'a>(pair: Pair<Rule>) -> ListPia {
    match pair.as_rule() {
        Rule::unit => List::parse(pair.into_inner().next().unwrap()).0,
        _ => unreachable!(),
    }
}
