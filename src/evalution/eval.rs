use crate::context::find_symbol::*;
use crate::context::*;
use crate::error::*;
use crate::values::*;

pub trait Evalable {
    fn eval(&self, env: &mut ThreadContext) -> CResult;
}

impl Evalable for Value {
    fn eval(&self, env: &mut ThreadContext) -> CResult {
        match self {
            Value::Sym(id) => find_symbol(env, id),
            Value::Pair(x) => x.eval(env),
            _ => Ok(self.clone()),
        }
    }
}

type Expr = Handle<Node>;

// fast(xD
macro_rules! fast_return {
	($e:expr) => {
		if let Ok(res) = $e {
			return Ok(res);
		}
	};
}

impl Evalable for Expr {
    fn eval(&self, env: &mut ThreadContext) -> CResult {
        fast_return!(quote_eval(env, self));
		// todo!()
		Err(RuntimeError::SyntaxError(SyntaxMatchError::SyntaxRuleIsNotExist))
    }
}

fn quote_eval(_env: &mut ThreadContext, expr: &Expr) -> CResult {
    let keyword = expr
        .car()
        .get_sym()
        .ok_or(RuntimeError::SyntaxError(SyntaxMatchError::MatchError))?;
    {
        if *keyword == Symbol::new("quote") {
            return Err(RuntimeError::SyntaxError(
                SyntaxMatchError::SyntaxMatchError(keyword),
            ));
        }
    }
    let expr = expr
        .cdr()
        .get_list()
        .ok_or(RuntimeError::SyntaxError(
			SyntaxMatchError::SyntaxMatchError(keyword.clone())))?;
    let value = expr.cdr();
    expr.cdr()
        .get_nil()
        .ok_or(RuntimeError::SyntaxError(SyntaxMatchError::SyntaxMatchError(keyword)))?;
    Ok(value)
}
