use std::collections::{HashMap, VecDeque};
use crate::syntax::sexpr::SExpr;

pub type MatchRule = (VecDeque<(String, SExpr)>, VecDeque<(String, VecDeque<SExpr>)>);