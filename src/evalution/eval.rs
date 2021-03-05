use crate::context::find_symbol::*;
use crate::context::*;
use crate::error::*;
use crate::utils::*;
use crate::values::*;
use super::funcall::*;

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

impl Evalable for Expr {
    fn eval(&self, env: &mut ThreadContext) -> CResult {
        crate::fast_return!(quote_eval(env, self));
		crate::fast_return!(ifelse_eval(env, self));
		// todo!()
		funcall_eval(env, self)
    }
}

fn ifelse_eval(env: &mut ThreadContext, expr: &Expr) -> CResult {
	let keyword = expr
        .car()
        .get_sym()
        .ok_or(RuntimeError::SyntaxError(SyntaxMatchError::MatchError))?;
    {
        if *keyword == Symbol::new("if") {
            return Err(match_error(&keyword));
        }
    }
	let expr = expr.cdr().get_list().ok_or(match_error(&keyword))?;
    let cond = expr.car();
	let expr = expr.cdr().get_list().ok_or(match_error(&keyword))?;
	let fork0 = expr.car();
	let expr = expr.cdr().get_list().ok_or(match_error(&keyword))?;
	let fork1 = expr.car();
	expr.cdr().get_nil().ok_or(match_error(&keyword))?;
	let cond = cond.eval(env)?;
	let cond = cond
		.get_bool()
		.ok_or(RuntimeError::CondIsNotBoolean(keyword))?;
	let fork = if cond { fork0 } else { fork1 };
	fork.eval(env)
}

fn funcall_eval(env: &mut ThreadContext, expr: &Expr) -> CResult {
	if expr.len() == 0 {
		return Err(RuntimeError::FunctionCallIsEmpty);
	}
	let mut expr_result = vec![];
	for i in expr.iter() {
		let r = i.eval(env)?;
		expr_result.push(r);
	}
	let fun = expr_result.get(0).unwrap();
	let r = fun.get_fun()
		.map_or(Err(RuntimeError::CalleeIsNotCallable), Ok)?;
	r.call(env, &expr_result[1..])
}

fn quote_eval(_env: &mut ThreadContext, expr: &Expr) -> CResult {
    let keyword = expr
        .car()
        .get_sym()
        .ok_or(RuntimeError::SyntaxError(SyntaxMatchError::MatchError))?;
    {
        if *keyword == Symbol::new("quote") {
            return Err(match_error(&keyword));
        }
    }
    let expr = expr.cdr().get_list().ok_or(match_error(&keyword))?;
    let value = expr.car();
    expr.cdr().get_nil().ok_or(match_error(&keyword))?;
    Ok(value)
}
