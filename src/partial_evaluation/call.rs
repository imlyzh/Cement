use std::sync::Arc;

use crate::{ast::{Ast, Pair, Params, callable::Lambda}, runtime::{NameSpace, value::{NativeInterface, Value}}};

pub trait PartialCall {
    fn partial_call(&self, env: Arc<NameSpace>, params: Pair<Params<Result<Value, Ast>>>) -> Result<Value, Ast>;
}

impl PartialCall for Lambda {
    fn partial_call(&self, env: Arc<NameSpace>, params: Pair<Params<Result<Value, Ast>>>) -> Result<Value, Ast> {
        todo!()
    }
}

impl PartialCall for NativeInterface {
    fn partial_call(&self, _env: Arc<NameSpace>, params: Pair<Params<Result<Value, Ast>>>) -> Result<Value, Ast> {
        (self.pe.unwrap())(params)
    }
}
