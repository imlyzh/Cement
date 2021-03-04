use std::{collections::HashMap, iter::FromIterator, path::Path, sync::RwLock};


use crate::values::Handle;
use crate::{error::CompilerError, values::Symbol};
use crate::syntax::parser::file_parse;
use logic_path::get_path;

// use super::symbols::*;
use super::sexpr_parser::*;
use crate::context::*;
use crate::error::SyntaxMatchError;
use crate::values::*;

pub enum Define {}
pub enum Expr {}
pub enum ModuleItem {}

pub trait Loading {
    fn loading(
        parent: &Option<Handle<FunctionDef>>,
        from_module: &Handle<Module>,
        i: &Value,
    ) -> Result<Value, CompilerError>;
}

impl Loading for MacroDef {
    fn loading(
        _: &Option<Handle<FunctionDef>>,
        from_module: &Handle<Module>,
        i: &Value,
    ) -> Result<Value, CompilerError> {
        let (name, pairs) = MacroDef::sexpr_parse(i)
			.map_err(CompilerError::SyntaxMatchError)?;
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
            .map_or(Err(CompilerError::RepeatedMacro(name)), 
			|_| { Ok(Value::Nil) })
    }
}

impl Loading for FunctionDef {
    fn loading(
        parent: &Option<Handle<Self>>,
        from_module: &Handle<Module>,
        i: &Value,
    ) -> Result<Value, CompilerError> {
        let (name, params, body) = FunctionDef::sexpr_parse(i)
			.map_err(CompilerError::SyntaxMatchError)?;
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
            .ok_or(CompilerError::RepeatedFunction(name))
    }
}

impl Loading for Expr {
    fn loading(
        parent: &Option<Handle<FunctionDef>>,
        from_module: &Handle<Module>,
        i: &Value,
    ) -> Result<Value, CompilerError> {
        if let Ok(x) = FunctionDef::loading(parent, from_module, i) {
            return Ok(x);
        }
        if let Value::Pair(x) = i {
            let r: Result<Vec<Value>, _> = x
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
    ) -> Result<Value, CompilerError> {
        let (name, value) = Define::sexpr_parse(i)
			.map_err(CompilerError::SyntaxMatchError)?;
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
                    .ok_or(CompilerError::RepeatedFunction(name));
            }
        }
        from_module
            .constant_table
            .write()
            .unwrap()
            .insert(name.clone(), value)
            .ok_or(CompilerError::RepeatedFunction(name))
    }
}

impl Loading for ModuleItem {
    fn loading(
        parent: &Option<Handle<FunctionDef>>,
        from_module: &Handle<Module>,
        i: &Value,
    ) -> Result<Value, CompilerError> {
        if Define::loading(parent, from_module, i).is_ok()
            || MacroDef::loading(parent, from_module, i).is_ok()
        {
            Ok(Value::Nil)
        } else {
            Err(CompilerError::SyntaxMatchError(SyntaxMatchError::MatchError))
        }
    }
}

fn loading_module(from: &Handle<Module>, path: &Path) -> Result<Handle<Module>, CompilerError> {
	let file_path = Handle::new(Symbol::new(path.to_str().unwrap()));
	let name = path.file_stem().map_or(
		Err(CompilerError::FileOpenError(file_path.clone())),
		|x| Ok(x.to_str().unwrap()))?;
	let name = Handle::new(Symbol::new(name));
	let modu = Handle::new(Module {
			name,
			file_path,
			parent: Some(from.clone()),
			module_table: RwLock::new(HashMap::new()),
			macro_table: RwLock::new(HashMap::new()),
			constant_table: RwLock::new(HashMap::new()),
		});

	if path.is_file() {
		file_parse(path)?
			.try_for_each(|x| ModuleItem::loading(&None, &modu, &x).map(|_| ()))?;
		return Ok(modu);
	}
	if path.is_dir() {
		let file_paths = path.read_dir().unwrap()
			.filter(|x| x.is_ok())
			.map(|x| x.unwrap())
			.filter(|x| x.path().extension()
				.map_or(false, |x| x == "ce0"));
		let record: Result<HashMap<Handle<Symbol>, Handle<Module>>, _> = file_paths
			.map(|x| loading_module(&modu, x.path().as_path()))
			.map(|x| x.map(|x| (x.name.clone(), x)))
			.collect();
		let record = record?;
		modu.module_table.write().unwrap().extend(record);
		return Ok(modu);
	}
	unimplemented!()
}

pub fn loading_package(env: &EnvContext, path: &Path) -> Result<(), CompilerError> {
	let mut mg = env.module_table.lock().unwrap();
	let from = mg.get(&Symbol::new("built-in")).unwrap();
	let modu = loading_module(from, path)?;
	let name = modu.name.clone();
	mg.insert(name.clone(), modu)
		.map_or(Err(CompilerError::RepeatedModule(name)), |_|Ok(()))
}

/* impl Loading for Module {
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
            .write()
            .unwrap()
            .insert(modu_name.clone(), module)
            .ok_or(SyntaxMatchError::RepeatedModule(modu_name))?;
        Ok(Value::Nil)
    }
} */

