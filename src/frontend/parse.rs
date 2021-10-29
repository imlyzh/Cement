

use std::sync::Arc;
use std::vec;
use std::{fs::File, io::Read};

use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::*;
use sexpr_ir::gast::constant::Constant;
use sexpr_ir::gast::symbol::{Location, Symbol};

use crate::ast::{Ast, Call, Cond, FunctionDef, Lets, TopLevel};
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

impl ParseFrom<Rule> for TopLevel {
    fn parse_from(pair: Pair<Rule>, path: Arc<String>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::module_item);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::import => TopLevel::Import(import_parse_from(pair, path)),
            Rule::fundef => TopLevel::FunctionDef(FunctionDef::parse_from(pair, path)),
            Rule::macrodef => todo!(),
            _ => unreachable!()
        }
    }
}

fn import_parse_from(pair: Pair<Rule>, path: Arc<String>) -> Vec<Symbol> {
    debug_assert_eq!(pair.as_rule(), Rule::import);
    let pair = pair.into_inner().next().unwrap();
    debug_assert_eq!(pair.as_rule(), Rule::path);
    pair.into_inner().map(|x| Symbol::parse_from(x, path.clone())).collect()
}

impl ParseFrom<Rule> for FunctionDef {
    fn parse_from(pair: Pair<Rule>, path: Arc<String>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::fundef);
        let mut pairs = pair.into_inner();
        let name = pairs.next().unwrap();
        let params = pairs.next().unwrap();
        let body = pairs.next().unwrap();
        let name = Symbol::parse_from(name, path.clone());
        let (params, is_var_length) = params_parse_from(params, path.clone());
        let body = Ast::parse_from(body, path);
        let l = Lambda(params, is_var_length, body);
        FunctionDef(name, l)
    }
}

impl ParseFrom<Rule> for Ast {
    fn parse_from(pair: Pair<Rule>, path: Arc<String>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::ast);
        let mut pairs = pair.into_inner();
        let pair = pairs.next().unwrap();
        let pair = pair.into_inner().next().unwrap();
        let mut r = match pair.as_rule() {
            Rule::cond => Ast::Cond(Cond::parse_from(pair, path.clone())),
            Rule::lets => Ast::Lets(Lets::parse_from(pair, path.clone())),
            Rule::begin => begin_parse_from(pair, path.clone()),
            Rule::lambda => Ast::Lambda(Arc::new(Lambda::parse_from(pair, path.clone()))),
            Rule::consts => Ast::Const(Constant::parse_from(pair, path.clone())),
            Rule::symbol => Ast::Var(Symbol::parse_from(pair, path.clone())),
            _ => unreachable!()
        };
        for i in pairs {
            debug_assert_eq!(i.as_rule(), Rule::ast_extend);
            let i = i.into_inner().next().unwrap();
            match i.as_rule() {
                Rule::extend_call => {
                    let (callee, mut params) = extend_call_parse_from(i, path.clone());
                    params.insert(0, r);
                    r = Ast::Call(Call(Box::new(callee), params));
                },
                Rule::call_params => {
                    let params = call_params_parse_from(i, path.clone());
                    r = Ast::Call(Call(Box::new(r), params));
                },
                _ => unreachable!()
            }
        }
        r
    }
}

fn extend_call_parse_from(pair: Pair<Rule>, path: Arc<String>) -> (Ast, Vec<Ast>) {
    debug_assert_eq!(pair.as_rule(), Rule::extend_call);
    let mut pairs = pair.into_inner();
    let e = pairs.next().unwrap();
    let p = pairs.next().unwrap();
    let e = Ast::parse_from(e, path.clone());
    let p = call_params_parse_from(p, path);
    (e, p)
}

fn call_params_parse_from(pair: Pair<Rule>, path: Arc<String>) -> Vec<Ast> {
    debug_assert_eq!(pair.as_rule(), Rule::call_params);
    pair.into_inner().map(|x| Ast::parse_from(x, path.clone())).collect()
}

impl ParseFrom<Rule> for Cond {
    fn parse_from(pair: Pair<Rule>, path: Arc<String>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::cond);
        let mut pairs = pair.into_inner();
        let pair = pairs.next().unwrap();
        let r = cond_pairs_parse_from(pair, path);
        Cond(r)
    }
}

fn cond_pairs_parse_from(i: Pair<Rule>, path: Arc<String>) -> Vec<(Ast, Ast)> {
    debug_assert_eq!(i.as_rule(), Rule::cond_pairs);
    i.into_inner().map(|x| cond_pair_parse_from(x, path.clone())).collect()
}

fn cond_pair_parse_from(i: Pair<Rule>, path: Arc<String>) -> (Ast, Ast) {
    debug_assert_eq!(i.as_rule(), Rule::cond_pair);
    let mut pairs = i.into_inner();
    let a = pairs.next().unwrap();
    let b = pairs.next().unwrap();
    let a = Ast::parse_from(a, path.clone());
    let b = Ast::parse_from(b, path);
    (a, b)
}


impl ParseFrom<Rule> for Lets {
    fn parse_from(pair: Pair<Rule>, path: Arc<String>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::lets);
        let mut pairs = pair.into_inner();
        let pair = pairs.next().unwrap();
        let r = let_pairs_parse_from(pair, path.clone());
        let body = Ast::parse_from(pairs.next().unwrap(), path);
        Lets(r, Box::new(body))
    }
}

fn let_pairs_parse_from(i: Pair<Rule>, path: Arc<String>) -> Vec<(Symbol, Ast)> {
    debug_assert_eq!(i.as_rule(), Rule::let_pairs);
    i.into_inner().map(|x| let_pair_parse_from(x, path.clone())).collect()
}

fn let_pair_parse_from(i: Pair<Rule>, path: Arc<String>) -> (Symbol, Ast) {
    debug_assert_eq!(i.as_rule(), Rule::let_pair);
    let mut pairs = i.into_inner();
    let a = pairs.next().unwrap();
    let b = pairs.next().unwrap();
    let a = Symbol::parse_from(a, path.clone());
    let b = Ast::parse_from(b, path);
    (a, b)
}

impl ParseFrom<Rule> for Symbol {
    fn parse_from(pair: Pair<Rule>, path: Arc<String>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::symbol);
        let (line, colum) = pair.as_span().start_pos().line_col();
        let pos = pair.as_span().start_pos().pos();
        let loc = Location { path, line, colum, pos };
        Symbol::from(pair.as_str(), &loc)
    }
}

impl ParseFrom<Rule> for Lambda {
    fn parse_from(pair: Pair<Rule>, path: Arc<String>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::lambda);
        let mut pairs = pair.into_inner();
        let params = pairs.next().unwrap();
        let body = pairs.next().unwrap();
        let (params, is_var_length) = params_parse_from(params, path.clone());
        let body = Ast::parse_from(body, path);
        Lambda(params, is_var_length, body)
    }
}

fn params_parse_from(pair: Pair<Rule>, path: Arc<String>) -> (Vec<Symbol>, bool) {
    debug_assert_eq!(pair.as_rule(), Rule::params);
    let mut pairs = pair.into_inner();
    let p = pairs.next();
    if p.is_none() {
        return (vec![], false)
    }
    let p = p.unwrap();
    let p = p_parse_from(p, path);
    let is_vl = pairs.next();
    if is_vl.is_some() {
        (p, true)
    } else {
        (p, false)
    }
}

fn p_parse_from(pair: Pair<Rule>, path: Arc<String>) -> Vec<Symbol> {
    debug_assert_eq!(pair.as_rule(), Rule::params_);
    pair.into_inner().map(|x| Symbol::parse_from(x, path.clone())).collect()
}

fn begin_parse_from(pair: Pair<Rule>, path: Arc<String>) -> Ast {
    debug_assert_eq!(pair.as_rule(), Rule::begin);
    let pairs = pair.into_inner();
    let r: Vec<_> = pairs.map(|x| Ast::parse_from(x, path.clone())).collect();
    if r.len() == 1 {
        r.first().unwrap().clone()
    } else {
        Ast::Begin(r)
    }
}

impl ParseFrom<Rule> for Constant {
    fn parse_from(pair: Pair<Rule>, _path: Arc<String>) -> Self {
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

pub fn parse_test(i: &str) -> Vec<TopLevel> {
    let mut r = Cement::parse(Rule::module, i).unwrap();
    let path = Arc::new("<test>".to_string());
    let r = r.next().unwrap();
    r.into_inner().filter(|x| x.as_rule() != Rule::EOI).map(|r| TopLevel::parse_from(r, path.clone())).collect()
}