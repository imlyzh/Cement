use crate::{
    context::{FunctionDef, MacroDef},
    values::Symbol,
};

use crate::values::{Handle, NodeIter};

use crate::error::SyntaxMatchError;

use crate::{context::Module, values::Value};

use super::loading::Define;

pub trait SexprParser {
    type Output;
    fn sexpr_parse(i: &Value) -> Result<Self::Output, SyntaxMatchError>;
}

type List2Result = Result<(Value, Value), SyntaxMatchError>;

fn parse_list2(i: &Value) -> List2Result {
    let expr_list = i.get_list().ok_or(SyntaxMatchError::MatchError)?;
    let r0 = expr_list.car();
    let expr_list = expr_list
        .cdr()
        .get_list()
        .ok_or(SyntaxMatchError::MatchError)?;
    let r1 = expr_list.car();
    expr_list
        .cdr()
        .get_nil()
        .ok_or(SyntaxMatchError::MatchError)?;
    Ok((r0, r1))
}

impl SexprParser for MacroDef {
    type Output = (Handle<Symbol>, Vec<(Value, Value)>);

    fn sexpr_parse(i: &Value) -> Result<Self::Output, SyntaxMatchError> {
        let expr_list = i.get_list().ok_or(SyntaxMatchError::MatchError)?;
        {
            let is_module_sym = expr_list
                .car()
                .get_sym()
                .map(|x| *x == Symbol::new("macro"))
                .ok_or(SyntaxMatchError::MatchError)?;
            if !is_module_sym {
                return Err(SyntaxMatchError::MatchError);
            }
        }
        let expr_list = expr_list
            .cdr()
            .get_list()
            .ok_or(SyntaxMatchError::MatchError)?;
        let name = expr_list
            .car()
            .get_sym()
            .ok_or(SyntaxMatchError::MatchError)?;
        let bodys = if expr_list.cdr().is_nil() {
            Ok(vec![])
        } else {
            let bodys = expr_list
                .cdr()
                .get_list()
                .ok_or(SyntaxMatchError::MatchError)?;
            NodeIter::new(bodys).map(|x| parse_list2(&x)).collect()
        };
        Ok((name, bodys?))
    }
}

impl SexprParser for FunctionDef {
    type Output = (Handle<Symbol>, Vec<Value>, Vec<Value>);

    fn sexpr_parse(i: &Value) -> Result<Self::Output, SyntaxMatchError> {
        let expr_list = i.get_list().ok_or(SyntaxMatchError::MatchError)?;
        {
            let is_module_sym = expr_list
                .car()
                .get_sym()
                .map(|x| *x == Symbol::new("fun"))
                .ok_or(SyntaxMatchError::MatchError)?;
            if !is_module_sym {
                return Err(SyntaxMatchError::MatchError);
            }
        }
        let expr_list = expr_list
            .cdr()
            .get_list()
            .ok_or(SyntaxMatchError::MatchError)?;
        let arg_list = expr_list
            .car()
            .get_list()
            .ok_or(SyntaxMatchError::MatchError)?;
        let name = arg_list
            .car()
            .get_sym()
            .ok_or(SyntaxMatchError::MatchError)?;
        let args = if expr_list.cdr().is_nil() {
            vec![]
        } else {
            let args = arg_list
                .cdr()
                .get_list()
                .ok_or(SyntaxMatchError::MatchError)?;
            NodeIter::new(args).collect()
        };
        let bodys = if expr_list.cdr().is_nil() {
            vec![]
        } else {
            let bodys = expr_list
                .cdr()
                .get_list()
                .ok_or(SyntaxMatchError::MatchError)?;
            NodeIter::new(bodys).collect()
        };
        Ok((name, args, bodys))
    }
}

impl SexprParser for Define {
    type Output = (Handle<Symbol>, Value);

    fn sexpr_parse(i: &Value) -> Result<Self::Output, SyntaxMatchError> {
        let expr_list = i.get_list().ok_or(SyntaxMatchError::MatchError)?;
        {
            let is_module_sym = expr_list
                .car()
                .get_sym()
                .map(|x| *x == Symbol::new("def"))
                .ok_or(SyntaxMatchError::MatchError)?;
            if !is_module_sym {
                return Err(SyntaxMatchError::MatchError);
            }
        }
        let expr_list = expr_list
            .cdr()
            .get_list()
            .ok_or(SyntaxMatchError::MatchError)?;
        let name = expr_list
            .car()
            .get_sym()
            .ok_or(SyntaxMatchError::MatchError)?;
        let expr_list = expr_list
            .cdr()
            .get_list()
            .ok_or(SyntaxMatchError::MatchError)?;
        let value = expr_list.car();
        expr_list
            .cdr()
            .get_nil()
            .ok_or(SyntaxMatchError::MatchError)?;
        Ok((name, value))
    }
}

impl SexprParser for Module {
    type Output = (Handle<Symbol>, Vec<Value>);

    fn sexpr_parse(i: &Value) -> Result<Self::Output, SyntaxMatchError> {
        let expr_list = i.get_list().ok_or(SyntaxMatchError::MatchError)?;
        {
            let is_module_sym = expr_list
                .car()
                .get_sym()
                .map(|x| *x == Symbol::new("module"))
                .ok_or(SyntaxMatchError::MatchError)?;
            if !is_module_sym {
                return Err(SyntaxMatchError::MatchError);
            }
        }
        let expr_list = expr_list
            .cdr()
            .get_list()
            .ok_or(SyntaxMatchError::MatchError)?;
        let module_name = expr_list
            .car()
            .get_sym()
            .ok_or(SyntaxMatchError::MatchError)?;
        if expr_list.cdr().is_nil() {
            return Ok((module_name, vec![]));
        }
        let bodys = expr_list
            .cdr()
            .get_list()
            .ok_or(SyntaxMatchError::MatchError)?;
        let bodys = NodeIter::new(bodys).collect();
        Ok((module_name, bodys))
    }
}
