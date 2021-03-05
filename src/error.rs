use crate::values::Symbol;
use crate::{syntax::parser::ParseError, values::Handle};

#[derive(Debug)]
pub enum SyntaxMatchError {
    MatchError,
    MatchListSizeError,
	SyntaxRuleIsNotExist,
    ExtendInMiddleError(Handle<Symbol>),
    RepeatedSymbol(Handle<Symbol>),
    SExprTypeCheckError(Handle<Symbol>),
    SyntaxMatchError(Handle<Symbol>),
}

#[derive(Debug)]
pub enum CompilerError {
    ParseError(ParseError),
    SyntaxMatchError(SyntaxMatchError),
	RepeatedModule(Handle<Symbol>),
    RepeatedMacro(Handle<Symbol>),
	RepeatedFunction(Handle<Symbol>),
	FileOpenError(Handle<Symbol>),
}

#[derive(Debug)]
pub enum RuntimeError {
    SymbolNotFound(Symbol),
    SyntaxError(SyntaxMatchError),
	CondIsNotBoolean(Handle<Symbol>),
    FrameStackIsEmpty,
    ModuleIsNotValue,
	FunctionCallIsEmpty,
	CalleeIsNotCallable,
}
