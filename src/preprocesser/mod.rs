mod macrodef;

use super::syntax::sexpr::*;
use crate::preprocesser::macrodef::MatchRule;
use std::collections::VecDeque;
use std::convert::identity;

fn match_atom(rule: &Atom, expr: &Atom) -> Result<Option<(String, RSExpr)>, ()> {
    match (rule, expr) {
        (Atom::Sym(name), b) => Ok(Some((name.clone(), RSExpr::Atomic(b.clone())))),
        (a, b) => {
            if a == b {
                Ok(None)
            } else {
                Err(())
            }
        }
    }
}

fn match_list<'a, T1: Iterator<Item = &'a SExpr>, T2: Iterator<Item = &'a SExpr>>(
    a: T1,
    b: T2,
) -> Result<MatchRule, ()> {
    let r = a
        .zip(b)
        .map(|(a, b)| rule_match(a, b))
        .collect::<Result<Vec<MatchRule>, ()>>()
        .unwrap();
    /*Ok(r.iter().rfold(
        (VecDeque::new(), VecDeque::new()),|
        (mut p, mut extend),
        (mut xp, mut xextend)| {
            p.append(&mut xp.clone());
            extend.append(&mut xextend.clone());
            (p.clone(), extend.clone())
        },
    ))*/
    todo!("别问，问就是鸽了")
}

fn rule_match(rule: &SExpr, rexpr: &SExpr) -> Result<MatchRule, ()> {
    let (rule, expr) = (rule.get_raw(), rexpr.get_raw());
    match (rule, expr) {
        (RSExpr::NonAtomic(mut a), RSExpr::NonAtomic(b)) => {
            let b = b.0.iter();
            if a.0.len() >= 2
                && a.0
                    .iter()
                    .last()
                    .unwrap()
                    .0
                    .get_atomic()
                    .and_then(|x| x.get_sym().and_then(|x| Some(()).filter(|_| x == "...")))
                    .is_some()
            {
                a.0.pop_back().unwrap();
                let append_sym = a.0.pop_back().unwrap();
                let append_values = &a.0.iter().collect::<Vec<_>>()[a.0.len() - 2..a.0.len()];
                let a = &a.0.iter().collect::<Vec<_>>()[0..a.0.len() - 2];
                let a = a.iter().map(|x| *x);
                let mlr = match_list(a, b);
            } else {
                let a = a.0.iter().map(identity);
                let mlr = match_list(a, b);
            }
            unimplemented!()
        }
        (RSExpr::Atomic(a), RSExpr::Atomic(b)) => {
            let mut r = VecDeque::new();
            if let Some((n, v)) = match_atom(&a, &b)? {
                r.push_back((n, SExpr(v, rexpr.1.clone())));
            }
            Ok((r, VecDeque::new()))
        }
        _ => Err(()),
    }
}

// fn pre_process(_expr: SExpr) -> SExpr {
// unimplemented!()
// }
