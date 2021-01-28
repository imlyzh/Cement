use std::{
    cell::RefCell,
    collections::{VecDeque},
};

use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::*;

use crate::utils::{escape_str, str2char};
use crate::{utils::string_intern, values::*};

#[derive(Parser)]
#[grammar = "./syntax/grammar.pest"]
pub struct Cement {}

pub type ParseError = Error<Rule>;

impl ParseFrom<Rule> for Value {
    fn parse_from(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::list => NodeExtend::parse_from(pair).into(),
            Rule::symbol => Value::Sym(Handle::new(Symbol::parse_from(pair))),
            Rule::string_lit => Value::Str(Handle::new(escape_str(pair.as_str()))),
            Rule::uint_lit => Value::Uint(pair.as_str().parse().unwrap()),
            Rule::int_lit => Value::Int(pair.as_str().parse().unwrap()),
            Rule::float_lit => Value::Float(pair.as_str().parse().unwrap()),
            Rule::bool_lit => Value::Bool(pair.as_str().parse().unwrap()),
            Rule::char_lit => Value::Char(str2char(&escape_str(pair.as_str()))),
            Rule::nil_lit => Value::Nil,
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for NodeExtend {
    fn parse_from(pair: Pair<Rule>) -> Self {
        pair.into_inner()
            .flat_map(|x| x.into_inner())
            .map(Value::parse_from)
            .collect()
    }
}

impl ParseFrom<Rule> for Symbol {
    fn parse_from(pair: Pair<Rule>) -> Self {
        let (line, colum) = pair.as_span().start_pos().line_col();
        let pos = pair.as_span().start_pos().pos();
        Symbol {
            id: string_intern(pair.as_str()),
            line,
            colum,
            pos,
            scope: RefCell::new(VecDeque::new()),
            // value: RefCell::new(None)
        }
    }
}

pub fn parse_unit(pair: Pair<Rule>) -> Option<Value> {
    match pair.as_rule() {
        Rule::sexpr => Some(Value::parse_from(pair.clone().into_inner().next().unwrap())),
        Rule::EOI => None,
        _ => unreachable!(),
    }
}

pub fn parse(input: &str) -> Result<NodeExtend, ParseError> {
    let pairs: Pairs<Rule> = Cement::parse(Rule::unit, input)?;
    let result = pairs.flat_map(|x| x.into_inner()).filter_map(parse_unit);
    Ok(result.collect())
}

pub fn repl_parse(input: &str) -> Result<Value, ParseError> {
    let pair = Cement::parse(Rule::repl_unit, input)?
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap();
    Ok(Value::parse_from(pair))
}
