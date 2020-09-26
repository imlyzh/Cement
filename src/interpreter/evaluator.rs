use crate::syntax::sexpr::{List, RSExpr, SExpr};

use super::values::{AnyValue, Pair};
use std::{any::Any, collections::VecDeque};


fn quote_eval(l: &VecDeque<SExpr>) -> Option<AnyValue> {
    if l.len() != 2 {
        return None;
    }
    let v = l.front().unwrap();
    let r = v.0
    .get_atomic()?.get_sym()?;
    if r == "quote" {
        let mut a = l.iter();
        a.next();
        a.next().map(|x| AnyValue::sexpr2mvalue(x))
    } else {
        None
    }
}

fn fun_call(l: &VecDeque<SExpr>) -> Option<AnyValue> {
    let v = l.front().unwrap();
    let r = v.0.get_atomic()?.get_sym()?;
    todo!("fun call")
}

pub fn evaluation(i: &SExpr) -> AnyValue {
    let SExpr(ri, _) = i;
    match ri {
        RSExpr::NonAtomic(l) => {
            let List(l) = l;
            if l.len() == 0 {
                return AnyValue::sexpr2mvalue(i);
            }
            if let Some(r) = quote_eval(l) {
                return r
            }
            todo!("evaluation ...");
            if let Some(r) = fun_call(l) {
                return r
            }
            unimplemented!()
        },
        RSExpr::Atomic(_) => AnyValue::sexpr2mvalue(i),
    }
}
