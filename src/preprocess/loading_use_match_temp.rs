use std::{collections::HashMap, iter::FromIterator, sync::RwLock};

use get_name::GetName;
use logic_path::get_path;

use super::symbols::*;
use crate::context::*;
use crate::error::SyntaxMatchError;
use crate::values::*;

use super::match_template::*;

pub enum Define {}
pub enum Expr {}
pub enum ModuleItem {}

pub trait Loading {
    fn loading(
        parent: Option<Handle<FunctionDef>>,
        from_module: Handle<Module>,
        i: &Value,
    ) -> Result<Value, SyntaxMatchError>;
}

impl Loading for MacroDef {
    fn loading(
        _: Option<Handle<FunctionDef>>,
        from_module: Handle<Module>,
        i: &Value,
    ) -> Result<Value, SyntaxMatchError> {
        let mut ctx = MatchRecord::default();
        if match_template(&mut ctx, &USE_MATCH_TEMP1, i).is_ok() {
            let name = ctx
                .maps
                .borrow()
                .get(&NAME_SYM.clone())
                .unwrap()
                .get_sym()
                .unwrap();
            let path = get_path(None, from_module.clone());
            name.scope.replace(path);
            let match_value = ctx
                .extend_maps
                .borrow()
                .get(&TEMP_SYM.clone())
                .unwrap()
                .clone();
            let temp = ctx
                .extend_maps
                .borrow()
                .get(&MATCH_SYM.clone())
                .unwrap()
                .clone();

            let match_value: NodeIter = match_value.into();
            let temp: NodeIter = temp.into();

            let pairs = match_value.zip(temp).collect();

            let mcr = TempMacro {
                name: name.clone(),
                from_module: from_module.clone(),
                pairs,
            };
            return from_module
                .macro_table
                .write()
                .unwrap()
                .insert(name.clone(), Handle::new(MacroDef::TempMacro(mcr)))
                .map_or(Ok(Value::Nil), |_| {
                    Err(SyntaxMatchError::RepeatedMacro(name))
                });
        }
        Err(SyntaxMatchError::MatchError)
    }
}

impl Loading for FunctionDef {
    fn loading(
        parent: Option<Handle<Self>>,
        from_module: Handle<Module>,
        i: &Value,
    ) -> Result<Value, SyntaxMatchError> {
        let mut ctx = MatchRecord::default();
        match_template(&mut ctx, &FUNCTION_DEF_TEMP, i)?;
        let name = ctx
            .maps
            .borrow()
            .get(&NAME_SYM.clone())
            .unwrap()
            .get_sym()
            .unwrap();
        let path = get_path(parent.clone(), from_module.clone());
        name.scope.replace(path);
        let params = ctx
            .extend_maps
            .borrow()
            .get(&PARAMS_SYM.clone())
            .unwrap()
            .clone();
        let bodys = ctx
            .extend_maps
            .borrow()
            .get(&BODYS_SYM.clone())
            .unwrap()
            .clone();
        let bodys: Result<Vec<Value>, SyntaxMatchError> = NodeIter::from(bodys)
            .map(|x| Expr::loading(parent.clone(), from_module.clone(), &x))
            .collect();
        let body = bodys?;
        let f = UserFunctionDef {
            name,
            from_module,
            parent,
            body,
            constant_table: RwLock::new(HashMap::new()),
            params: NodeIter::from(params)
                .map(|x| x.get_sym().unwrap())
                .collect(),
        };
        let f = Handle::new(FunctionDef::UserFunction(f));
        Ok(Value::Function(f))
    }
}

impl Loading for Expr {
    fn loading(
        parent: Option<Handle<FunctionDef>>,
        from_module: Handle<Module>,
        i: &Value,
    ) -> Result<Value, SyntaxMatchError> {
        if let Ok(x) = FunctionDef::loading(parent.clone(), from_module.clone(), i) {
            return Ok(x);
        }
        if let Value::Pair(x) = i {
            let r: Result<Vec<Value>, SyntaxMatchError> = x
                .iter()
                .map(|x| Expr::loading(parent.clone(), from_module.clone(), &x))
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
        parent: Option<Handle<FunctionDef>>,
        from_module: Handle<Module>,
        i: &Value,
    ) -> Result<Value, SyntaxMatchError> {
        let mut ctx = MatchRecord::default();
        match_template(&mut ctx, &DEFINE_TEMP, i)?;
        let name = ctx
            .maps
            .borrow()
            .get(&NAME_SYM.clone())
            .unwrap()
            .get_sym()
            .unwrap();
        let path = get_path(parent.clone(), from_module.clone());
        name.scope.replace(path);
        let value = ctx.maps.borrow().get(&VALUE_SYM.clone()).unwrap().clone();
        let value = Expr::loading(parent.clone(), from_module.clone(), &value)?;
        if let Some(parent) = parent {
            if let FunctionDef::UserFunction(parent) = &*parent {
                return parent
                    .constant_table
                    .write()
                    .unwrap()
                    .insert(name.clone(), value)
                    .map_or(Ok(Value::Nil), |_| {
                        Err(SyntaxMatchError::RepeatedSymbol(name.clone()))
                    });
            }
            unreachable!("???你tm是怎么做到Native Function下边定义UserFunction的");
        }
        from_module
            .constant_table
            .write()
            .unwrap()
            .insert(name.clone(), value)
            .map_or(Ok(Value::Nil), |_| {
                Err(SyntaxMatchError::RepeatedFunction(name))
            })
    }
}

impl Loading for ModuleItem {
    fn loading(
        parent: Option<Handle<FunctionDef>>,
        from_module: Handle<Module>,
        i: &Value,
    ) -> Result<Value, SyntaxMatchError> {
        if Define::loading(parent.clone(), from_module.clone(), i).is_ok()
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
        _: Option<Handle<FunctionDef>>,
        from_module: Handle<Module>,
        i: &Value,
    ) -> Result<Value, SyntaxMatchError> {
        let mut ctx = MatchRecord::default();
        match_template(&mut ctx, &MODULE_MATCH_TEMP, i)?;
        let name = ctx
            .maps
            .borrow()
            .get(&NAME_SYM.clone())
            .unwrap()
            .get_sym()
            .unwrap();
        let path = get_path(None, from_module.clone());
        name.scope.replace(path);
        let body = ctx
            .extend_maps
            .borrow()
            .get(&NAME_SYM.clone())
            .unwrap()
            .clone();
        let body: Vec<Value> = body.0.map_or([].iter().map(Value::clone).collect(), |x| {
            x.iter().collect()
        });

        let modu = Module::new(name.clone(), Some(from_module.clone()));
        from_module
            .module_table
            .write()
            .unwrap()
            .insert(name, modu.clone())
            .map_or(Ok(()), |_| {
                Err(SyntaxMatchError::RepeatedModule(modu.get_name()))
            })?;
        for i in body {
            ModuleItem::loading(None, modu.clone(), &i)?;
        }
        Ok(Value::Nil)
    }
}
