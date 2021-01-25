use crate::syntax::values::Value;


#[derive(Debug)]
pub struct RuntimeError ();

#[derive(Debug)]
pub struct CResult (pub Result<Value, RuntimeError>);

#[derive(Debug)]
pub struct ThreadContext {

}

impl ThreadContext {
	pub fn new() {}
}