

use std::sync::Arc;
use std::vec;
use std::{fs::File, io::Read};

use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::*;
use sexpr_ir::gast::constant::Constant;

use crate::ast::{Ast, Cond, Lets};
use crate::ast::callable::Lambda;


#[derive(Parser)]
#[grammar = "./frontend/grammar.pest"]
pub struct Cement {}

pub type ParseError = Error<Rule>;

pub trait ParseFrom<T>
where
    Self: std::marker::Sized,
{
    fn parse_from(pair: Pair<T>, path: Arc<String>) -> Self;
}

impl ParseFrom<Rule> for Ast {
    fn parse_from(pair: Pair<Rule>, path: Arc<String>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::ast);
        let mut pairs = pair.into_inner();
        let pair = pairs.next().unwrap();
        let r = match pair.as_rule() {
            Rule::cond => Ast::Cond(Cond::parse_from(pair, path)),
            Rule::lets => Ast::Lets(Lets::parse_from(pair, path)),
            Rule::begin => begin_parse_from(pair, path),
            Rule::lambda => Ast::Lambda(Arc::new(Lambda::parse_from(pair, path))),
            Rule::consts => Ast::Const(Constant::parse_from(pair, path)),
            Rule::symbol => todo!(),
            _ => unreachable!()
        };
        // todo
        r
    }
}

impl ParseFrom<Rule> for Cond {
    fn parse_from(pair: Pair<Rule>, path: Arc<String>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::cond);
        todo!()
    }
}

impl ParseFrom<Rule> for Lets {
    fn parse_from(pair: Pair<Rule>, path: Arc<String>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::lets);
        todo!()
    }
}

impl ParseFrom<Rule> for Lambda {
    fn parse_from(pair: Pair<Rule>, path: Arc<String>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::lambda);
        todo!()
    }
}

fn begin_parse_from(pair: Pair<Rule>, path: Arc<String>) -> Ast {
    debug_assert_eq!(pair.as_rule(), Rule::begin);
    let pairs = pair.into_inner();
    let r = pairs.map(|x| Ast::parse_from(x, path.clone()));
    Ast::Begin(r.collect())
}

impl ParseFrom<Rule> for Constant {
    fn parse_from(pair: Pair<Rule>, path: Arc<String>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::consts);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::string_lit => todo!(),
            Rule::float => Constant::Float(pair.as_str().parse().unwrap()),
            Rule::int => Constant::Int(pair.as_str().parse().unwrap()),
            Rule::uint => Constant::Uint(pair.as_str().parse().unwrap()),
            Rule::bool_lit => Constant::Bool(pair.as_str().parse().unwrap()),
            Rule::nil_lit => Constant::Nil,
            _ => unreachable!()
        }
    }
}