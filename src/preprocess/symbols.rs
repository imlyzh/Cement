use lazy_static::lazy_static;

use crate::syntax::parser::repl_parse;
use crate::values::*;

lazy_static! {
    pub static ref ANONYMOUS_MODULE_NAME: Handle<Symbol> =
        Handle::new(Symbol::new("anonymous-module"));
    pub static ref EXTEND_SYM: Value = Value::Sym(Handle::new(Symbol::new("...")));
    pub static ref NAME_SYM: Handle<Symbol> = Handle::new(Symbol::new("name"));
    pub static ref MATCH_SYM: Handle<Symbol> = Handle::new(Symbol::new("match"));
    pub static ref TEMP_SYM: Handle<Symbol> = Handle::new(Symbol::new("temp"));
    pub static ref PARAMS_SYM: Handle<Symbol> = Handle::new(Symbol::new("params"));
    pub static ref BODYS_SYM: Handle<Symbol> = Handle::new(Symbol::new("bodys"));
    pub static ref VALUE_SYM: Handle<Symbol> = Handle::new(Symbol::new("value"));
}

lazy_static! {
    pub static ref MODULE_MATCH_TEMP: Value =
        repl_parse("((quote module) ($sym name) body ...)").unwrap();
    pub static ref USE_MATCH_TEMP: Value = repl_parse("((quote use) ($sym name))").unwrap();
    pub static ref USE_MATCH_TEMP1: Value = repl_parse("((quote use) (($sym name) ...))").unwrap();
    pub static ref MACRO_DEF_TEMP: Value =
        repl_parse("((quote macro) ($sym name) match temp)").unwrap();
    pub static ref MACRO_DEF_TEMP1: Value =
        repl_parse("((quote syntax-rules) ($sym name) [match temp] ...)").unwrap();
    pub static ref FUNCTION_DEF_TEMP1: Value =
        repl_parse("((quote fun) [($sym name) ($sym params) ...] bodys ...)").unwrap();
    pub static ref DEFINE_TEMP: Value = repl_parse("((quote define) ($sym name) value").unwrap();
}
