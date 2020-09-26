// use std::alloc::{alloc, Layout};
use std::ptr;
use std::any::Any;

use std::fmt::Debug;

// use any::TypeId;
use crate::syntax::sexpr::{Atom, RSExpr, SExpr};

#[derive(Debug)]
pub struct Pair {
    pub car: AnyValue,
    pub cdr: AnyValue,
}

#[derive(Debug)]
pub struct AnyValue(pub *const dyn Any);

pub const NIL: *const dyn Any = ptr::null::<()>();

impl AnyValue {
    pub fn sexpr2mvalue(i: &SExpr) -> Self {
        let SExpr(i, _) = i;
        match i {
            RSExpr::Atomic(v) => match v {
                Atom::Bool(v) => AnyValue(Box::into_raw(Box::new(*v))),
                Atom::Char(v) => AnyValue(Box::into_raw(Box::new(*v))),
                Atom::Num(v) => AnyValue(Box::into_raw(Box::new(v.clone()))),
                Atom::Str(v) => AnyValue(Box::into_raw(Box::new(v.clone()))),
                Atom::Sym(v) => AnyValue(Box::into_raw(Box::new(v.clone()))),
            },
            RSExpr::NonAtomic(v) => v.0.iter().rev().fold(AnyValue(NIL), |cdr, car| {
                let car = AnyValue::sexpr2mvalue(car);
                AnyValue(Box::into_raw(Box::new(Pair { car, cdr })))
            }),
        }
    }

    // pub fn cast_to_ref<'a>(&self) -> &'a dyn any::Any {
    // unsafe {
    // self.0.as_ref().unwrap()
    // }
    // }

    // pub fn get_type_name(&self) -> TypeId {
    // self.cast_to_ref().type_id()
    // }
}
