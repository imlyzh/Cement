use crate::values::Symbol;
use crate::{syntax::parser::ParseError, values::Handle};

#[derive(Debug)]
pub enum SyntaxMatchError {
    MatchError,
    MatchListSizeError,
    ExtendInMiddleError(Handle<Symbol>),
    RepeatedSymbol(Handle<Symbol>),
    RepeatedModule(Handle<Symbol>),
    RepeatedMacro(Handle<Symbol>),
    RepeatedFunction(Handle<Symbol>),
    SExprTypeCheckError(Handle<Symbol>),
}

#[derive(Debug)]
pub enum CompilerError {
    ParseError(ParseError),
    SyntaxMatchError(SyntaxMatchError),
}
