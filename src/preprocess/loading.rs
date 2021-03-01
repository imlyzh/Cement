use std::{collections::HashMap, iter::FromIterator, sync::RwLock};

use crate::values::Handle;
use logic_path::get_path;

// use super::symbols::*;
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
		let (name, pairs) = MacroDef::sexpr_parse(i)?;
		let r = Handle::new(MacroDef::TempMacro(TempMacro {
		    name: name.clone(),
		    pairs,
			from_module: from_module.clone(),
		}));
		from_module
			.macro_table
			.write()
			.unwrap()
			.insert(name.clone(), r)
			.map_or(Err(SyntaxMatchError::RepeatedMacro(name)), |_| Ok(Value::Nil))
	}
}

impl Loading for FunctionDef {
	fn loading(
		parent: &Option<Handle<Self>>,
		from_module: &Handle<Module>,
		i: &Value,
	) -> Result<Value, SyntaxMatchError> {
		let (name, params, body) = FunctionDef::sexpr_parse(i)?;
		let f = Handle::new(FunctionDef::UserFunction(UserFunctionDef {
		    name: name.clone(),
		    params,
		    body,
			from_module: from_module.clone(),
		    parent: parent.clone(),
		    constant_table: RwLock::new(HashMap::new()),
		}));
		from_module
			.constant_table
			.write()
			.unwrap()
			.insert(name.clone(), Value::Function(f))
			.ok_or(SyntaxMatchError::RepeatedFunction(name))
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
		let (name, value) = Define::sexpr_parse(i)?;
		let path = get_path(parent.clone(), from_module.clone());
		name.scope.replace(path);
		let value = Expr::loading(parent, from_module, &value)?;
		if let Some(parent) = parent {
			if let FunctionDef::UserFunction(parent) = &**parent {
				return parent
					.constant_table
					.write()
					.unwrap()
					.insert(name.clone(), value)
					.ok_or(SyntaxMatchError::RepeatedFunction(name));
			}
		}
		from_module
			.constant_table
			.write()
			.unwrap()
			.insert(name.clone(), value)
			.ok_or(SyntaxMatchError::RepeatedFunction(name))
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
