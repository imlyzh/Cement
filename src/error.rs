use std::sync::Arc;

use crate::values::Symbol;
use crate::syntax::parser::ParseError;

#[derive(Debug)]
pub enum SyntaxMatchError {
    MatchError,
    MatchListSizeError,
    ExtendInMiddleError,
    RepeatedSymbol(Arc<Symbol>),
}


#[derive(Debug)]
pub enum CompilerError {
	ParseError(ParseError),
	SyntaxMatchError(SyntaxMatchError),
}
