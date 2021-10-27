use std::sync::Arc;

use crate::{ast::{Ast, Pair, Params, callable::Lambda}, runtime::{NameSpace, value::{NativeInterface, Value}}};

use super::PartialEval;

pub trait PartialCall {
    fn partial_call(&self, env: Arc<NameSpace>, params: Pair<Params<Result<Value, Ast>>>) -> Option<Result<Value, Ast>>;
}

impl PartialCall for Lambda {
    fn partial_call(&self, env: Arc<NameSpace>, params: Pair<Params<Result<Value, Ast>>>) -> Option<Result<Value, Ast>> {
        let env = Arc::new(env.new_level());
        let params = self.0.partial_matchs(&params)?;
        for (k, v) in params {
            if let Ok(v) = v {
                env.add(k, v);
            }
        }
        Some(self.1.partial_eval(env))
    }
}

impl PartialCall for NativeInterface {
    fn partial_call(&self, _env: Arc<NameSpace>, params: Pair<Params<Result<Value, Ast>>>) -> Option<Result<Value, Ast>> {
        Some((self.pe.unwrap())(params))
    }
}
