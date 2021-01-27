use std::sync::Arc;

use crate::syntax::parser::ParseError;
use crate::values::Symbol;

#[derive(Debug)]
pub enum SyntaxMatchError {
    MatchError,
    MatchListSizeError,
    ExtendInMiddleError(Arc<Symbol>),
    RepeatedSymbol(Arc<Symbol>),
    SExprTypeCheckError(Arc<Symbol>),
}

#[derive(Debug)]
pub enum CompilerError {
    ParseError(ParseError),
    SyntaxMatchError(SyntaxMatchError),
}
