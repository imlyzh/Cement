use super::sexpr::*;
use pest::iterators::Pair;
use pest_derive::*;
use std::collections::VecDeque;
use std::io::Chain;
use std::str::FromStr;
use std::iter::FromIterator;

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

#[inline]
fn escape_char(i: char) -> char {
    match i {
        '\\' => '\\',
        '\"' => '\"',
        '\'' => '\'',
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        _ => unreachable!()
    }
}

fn state_machine(
    (mut prev, mut is_escape): (VecDeque<char>, bool),
    item: char,
) -> (VecDeque<char>, bool) {
    if is_escape {
        prev.push_back(escape_char(item));
        is_escape = false;
    } else {
        if item == '\\' {
            is_escape = true;
        } else {
            prev.push_back(item);
            is_escape = false;
        }
    }
    (prev, is_escape)
}

#[inline]
fn escape_str(i: &str) -> String {
    let (char_string, is_escape) = i.chars().fold((VecDeque::new(), false), state_machine);
    assert_eq!(is_escape, false);
    String::from_iter(char_string.iter())
}

#[inline]
fn str2char(i: &str) -> char {
    i.chars().nth(0).unwrap()
}

#[inline]
fn register_intern_str(i: &str) -> String {
    // FIXME
    i.to_string()
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
