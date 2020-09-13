use crate::syntax::sexpr::SExpr;
use std::collections::{HashMap, VecDeque};

pub type MatchRule = (
    VecDeque<(String, SExpr)>,
    VecDeque<(String, VecDeque<SExpr>)>,
);
