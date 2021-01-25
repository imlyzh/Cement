use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::*;

use super::values::*;
use crate::syntax::utils::{escape_str, str2char};
use pest::error::Error;

#[derive(Parser)]
#[grammar = "./syntax/grammar.pest"]
pub struct Cement {}

impl ParseFrom<Rule> for Value {
    fn parse_from(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
			Rule::list => 		Value::List(List::parse_from(pair)),
			Rule::symbol => 	Value::Sym(Symbol::parse_from(pair)),
            Rule::string_lit => Value::Str(escape_str(pair.as_str())),
            Rule::uint_lit => 	Value::Uint(pair.as_str().parse().unwrap()),
			Rule::int_lit => 	Value::Int(pair.as_str().parse().unwrap()),
			Rule::float_lit => 	Value::Float(pair.as_str().parse().unwrap()),
			Rule::bool_lit => 	Value::Bool(pair.as_str().parse().unwrap()),
			Rule::char_lit => 	Value::Char(str2char(&escape_str(pair.as_str()))),
			Rule::nil_lit =>	Value::Nil,
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for List {
    fn parse_from(pair: Pair<Rule>) -> Self {
        let lst: ListPia = pair
            .into_inner()
            .flat_map(|x| x.into_inner())
            .map(Value::parse_from)
            .collect();
        List(lst)
    }
}

impl ParseFrom<Rule> for Symbol {
    fn parse_from(pair: Pair<Rule>) -> Self {
        let (line, colum) = pair.as_span().start_pos().line_col();
        let pos = pair.as_span().start_pos().pos();
        Symbol {
            id: pair.as_str().to_string(),
            line,
            colum,
            pos,
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

pub type ParseError = Error<Rule>;

#[derive(Debug)]
pub struct CompilerError(pub ParseError);

pub fn parse(input: &str) -> Result<ListPia, CompilerError> {
    let pairs: Pairs<Rule> = Cement::parse(Rule::unit, input).map_err(|e| CompilerError(e))?;
	let result =
		pairs.flat_map(|x| x.into_inner()).filter_map(parse_unit);
    Ok(result.collect())
}

pub fn repl_parse(input: &str) -> Result<Value, CompilerError> {
	let pair = Cement::parse(Rule::repl_unit, input)
		.map_err(|e| CompilerError(e))?
		.next().unwrap().into_inner()
		.next().unwrap().into_inner()
		.next().unwrap();
    Ok(Value::parse_from(pair))
}
