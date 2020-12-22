use std::collections::HashMap;
use crate::syntax::sexpr::SExpr;

use lazy_static::*;


#[derive(Debug)]
struct ReplEnv (pub HashMap<String, SExpr>);

lazy_static! {

}