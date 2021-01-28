use lazy_static::lazy_static;

use crate::syntax::parser::repl_parse;
use crate::values::*;

lazy_static! {
    pub static ref ANONYMOUS_MODULE_NAME: Handle<Symbol> =
        Handle::new(Symbol::new("anonymous-module"));
    pub static ref EXTEND_SYM: Value = Value::Sym(Handle::new(Symbol::new("...")));
    pub static ref MODULE_MATCH_TEMP: Value =
        repl_parse("((quote module) ($sym name) body ...)").unwrap();
    pub static ref USE_MATCH_TEMP: Value =
		repl_parse("((quote use) ($sym name))").unwrap();
	pub static ref USE_MATCH_TEMP1: Value =
        repl_parse("((quote use) ($list name_list))").unwrap();
}
