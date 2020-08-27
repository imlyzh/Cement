mod macrodef;

use super::syntax::sexpr::*;
use crate::preprocesser::macrodef::MatchRule;
use std::collections::{VecDeque};
use std::convert::identity;

fn match_atom(rule: &Atom, expr: &Atom) -> Result<Option<(String, RSExpr)>, ()> {
    match (rule, expr) {
        (Atom::Sym(name), b) => {
            Ok(Some((name.clone(), RSExpr::Atomic(b.clone()))))
        },
        (a, b) => if a == b { Ok(None) } else { Err(()) }
    }
}

fn rule_match(rule: &SExpr, rexpr: &SExpr) -> Result<MatchRule, ()> {
    let (rule, expr) = (rule.get_raw(), rexpr.get_raw());
    match (rule, expr) {
        (RSExpr::NonAtomic(a), RSExpr::NonAtomic(b)) => {
            let a =
                a.0.iter()
                    .zip(b.0.iter())
                    .map(|(a, b)| rule_match(a, b))
                    .collect::<Result<Vec<MatchRule>, ()>>()?;
            unimplemented!()
        }
        (RSExpr::Atomic(a), RSExpr::Atomic(b)) => {
            let mut r = VecDeque::new();
            if let Some((n, v)) = match_atom(&a, &b)? {
                r.push_back((n, SExpr(v, rexpr.1.clone())));
            }
            Ok((r, VecDeque::new()))
        }
        _ => unimplemented!(),
    }
}

fn pre_process(_expr: SExpr) -> SExpr {
    unimplemented!()
}
