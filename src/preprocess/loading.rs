use std::{collections::HashMap, iter::FromIterator, sync::RwLock};

use crate::values::Handle;
use get_name::GetName;
use logic_path::get_path;

// use super::symbols::*;
use super::match_template::*;
use crate::context::*;
use crate::error::SyntaxMatchError;
use crate::values::*;
use super::sexpr_parser::*;

pub enum Define {}
pub enum Expr {}
pub enum ModuleItem {}

pub trait Loading {
    fn loading(
        parent: &Option<Handle<FunctionDef>>,
        from_module: &Handle<Module>,
        i: &Value,
    ) -> Result<Value, SyntaxMatchError>;
}


impl Loading for MacroDef {
    fn loading(
        _: &Option<Handle<FunctionDef>>,
        from_module: &Handle<Module>,
        i: &Value,
    ) -> Result<Value, SyntaxMatchError> {
		todo!()
    }
}

impl Loading for FunctionDef {
    fn loading(
        parent: &Option<Handle<Self>>,
        from_module: &Handle<Module>,
        i: &Value,
    ) -> Result<Value, SyntaxMatchError> {
        todo!()
    }
}

impl Loading for Expr {
    fn loading(
        parent: &Option<Handle<FunctionDef>>,
        from_module: &Handle<Module>,
        i: &Value,
    ) -> Result<Value, SyntaxMatchError> {
        if let Ok(x) = FunctionDef::loading(parent, from_module, i) {
            return Ok(x);
        }
        if let Value::Pair(x) = i {
            let r: Result<Vec<Value>, SyntaxMatchError> = x
                .iter()
                .map(|x| Expr::loading(parent, from_module, &x))
                .collect();
            let r = NodeExtend::from_iter(r?).into_value();
            Ok(r)
        } else {
            Ok(i.clone())
        }
    }
}

impl Loading for Define {
    fn loading(
        parent: &Option<Handle<FunctionDef>>,
        from_module: &Handle<Module>,
        i: &Value,
    ) -> Result<Value, SyntaxMatchError> {
        todo!()
	}
}

impl Loading for ModuleItem {
    fn loading(
        parent: &Option<Handle<FunctionDef>>,
        from_module: &Handle<Module>,
        i: &Value,
    ) -> Result<Value, SyntaxMatchError> {
        if Define::loading(parent, from_module, i).is_ok()
            || MacroDef::loading(parent, from_module, i).is_ok()
        {
            Ok(Value::Nil)
        } else {
            Err(SyntaxMatchError::MatchError)
        }
    }
}

impl Loading for Module {
    fn loading(
        _: &Option<Handle<FunctionDef>>,
        from_module: &Handle<Module>,
        i: &Value,
    ) -> Result<Value, SyntaxMatchError> {
		let (modu_name, bodys) = Module::sexpr_parse(i)?;
		let module = Module::new(&modu_name, &Some(from_module.clone()));
        for i in bodys {
            ModuleItem::loading(&None, &module, &i)?;
        }
		from_module
            .module_table
            .write().unwrap()
            .insert(modu_name.clone(), module)
			.ok_or(SyntaxMatchError::RepeatedModule(modu_name))?;
        Ok(Value::Nil)
    }
}
