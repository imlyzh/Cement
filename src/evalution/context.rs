use std::collections::HashMap;
use crate::syntax::values::SExpr;


#[derive(Debug)]
pub struct RuntimeError ();

#[derive(Debug)]
pub struct CResult (pub Result<SExpr, RuntimeError>);

#[derive(Debug)]
pub struct ReplEnv (pub HashMap<String, SExpr>);

impl ReplEnv {
    pub fn new() -> Self {
        ReplEnv(HashMap::new())
    }
}

/*
lazy_static! {
    static ref REPL_ENV: ReplEnv = ReplEnv::new();
}
*/