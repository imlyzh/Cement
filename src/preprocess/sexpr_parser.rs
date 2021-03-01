use crate::values::Symbol;

use crate::values::{Handle, NodeIter};

use crate::error::SyntaxMatchError;

use crate::{context::Module, values::Value};

use super::loading::Define;


pub trait SexprParser {
	type Output;
	fn sexpr_parse(i: &Value) -> Result<Self::Output, SyntaxMatchError>;
}

impl SexprParser for Define {
    type Output = (Handle<Symbol>, Value);

    fn sexpr_parse(i: &Value) -> Result<Self::Output, SyntaxMatchError> {
        let expr_list = i.get_list().ok_or(SyntaxMatchError::MatchError)?;
		{
			let is_module_sym =
			expr_list.car().get_sym()
				.map(|x| *x == Symbol::new("def"))
				.ok_or(SyntaxMatchError::MatchError)?;
			if !is_module_sym {
				return Err(SyntaxMatchError::MatchError);
			}
		}
		let expr_list = expr_list.cdr().get_list().ok_or(SyntaxMatchError::MatchError)?;
		let name = expr_list.car().get_sym().ok_or(SyntaxMatchError::MatchError)?;
		let expr_list = expr_list.cdr().get_list().ok_or(SyntaxMatchError::MatchError)?;
		let value = expr_list.car();
		expr_list.cdr().get_nil().ok_or(SyntaxMatchError::MatchError)?;
		
		Ok((name, value))
    }
}

impl SexprParser for Module {
	type Output = (Handle<Symbol>, Vec<Value>);

	fn sexpr_parse(i: &Value) -> Result<Self::Output, SyntaxMatchError> {
        let expr_list = i.get_list().ok_or(SyntaxMatchError::MatchError)?;
		{
			let is_module_sym =
			expr_list.car().get_sym()
				.map(|x| *x == Symbol::new("module"))
				.ok_or(SyntaxMatchError::MatchError)?;
			if !is_module_sym {
				return Err(SyntaxMatchError::MatchError);
			}
		}
		let expr_list = expr_list.cdr().get_list().ok_or(SyntaxMatchError::MatchError)?;
		let module_name = expr_list.car().get_sym().ok_or(SyntaxMatchError::MatchError)?;
		let bodys = expr_list.cdr().get_pair()
		.ok_or(SyntaxMatchError::MatchError)?;
		let bodys = NodeIter::new(bodys).collect();
		Ok((module_name, bodys))
    }
}