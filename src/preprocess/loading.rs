use get_name::GetName;

use super::symbols::*;
use crate::context::*;
use crate::error::SyntaxMatchError;
use crate::values::*;

use super::match_template::*;

#[derive(Debug, Clone)]
struct UseSentence(pub Vec<Handle<Symbol>>);

#[derive(Debug)]
enum ModuleItem {}

pub trait Loading {
    fn loading(
        parent: Option<Handle<FunctionDef>>,
        from_module: Handle<Module>,
        i: &Value,
    ) -> Result<(), SyntaxMatchError>;
}

impl Loading for MacroDef {
    fn loading(
        _: Option<Handle<FunctionDef>>,
        from_module: Handle<Module>,
        i: &Value,
    ) -> Result<(), SyntaxMatchError> {
        /*
        let mut ctx = MatchRecord::default();
        if match_template(&mut ctx, &MACRO_DEF_TEMP, i).is_ok() {
            let name = ctx
                .maps
                .borrow()
                .get(&NAME_SYM.clone())
                .unwrap()
                .get_sym()
                .unwrap();
            let match_value = ctx.maps.borrow().get(&MATCH_SYM.clone()).unwrap().clone();
            let temp = ctx.maps.borrow().get(&TEMP_SYM.clone()).unwrap().clone();

            let mcr = TempMacro {
                name,
                from_module,
                pairs: vec![(match_value, temp)],
            };
            from_module.macro_table.write().unwrap()
            .insert(name, Handle::new(MacroDef::TempMacro(mcr)))
            .map_or(Ok(()), |_| {
                Err(SyntaxMatchError::RepeatedMacro(name))
            })?;
        }
        */
        let mut ctx = MatchRecord::default();
        if match_template(&mut ctx, &USE_MATCH_TEMP1, i).is_ok() {
            let name = ctx
                .maps
                .borrow()
                .get(&NAME_SYM.clone())
                .unwrap()
                .get_sym()
                .unwrap();
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
				.map_or(Ok(()), |_| Err(SyntaxMatchError::RepeatedMacro(name)));
        }
        Err(SyntaxMatchError::MatchError)
    }
}

impl Loading for FunctionDef {
    fn loading(
        _parent: Option<Handle<Self>>,
        _from_module: Handle<Module>,
        _i: &Value,
    ) -> Result<(), SyntaxMatchError> {
        todo!()
    }
}

impl Loading for UseSentence {
    fn loading(
        _: Option<Handle<FunctionDef>>,
        _from_module: Handle<Module>,
        i: &Value,
    ) -> Result<(), SyntaxMatchError> {
        /*
        let mut ctx = MatchRecord::default();
        if match_template(&mut ctx, &USE_MATCH_TEMP, i).is_ok() {
            let r = ctx
                .maps
                .borrow()
                .get(&NAME_SYM.clone())
                .unwrap()
                .get_sym()
                .unwrap();
        }*/
        let mut ctx = MatchRecord::default();
        if match_template(&mut ctx, &USE_MATCH_TEMP1, i).is_ok() {
            let r = ctx
                .extend_maps
                .borrow()
                .get(&NAME_SYM.clone())
                .unwrap()
                .clone();
            let _import_names: Vec<Handle<Symbol>> = NodeIter::from(r).map(|x| x.get_sym().unwrap()).collect();
            todo!("use register");
            /*
            from_module.macro_table.write().unwrap()
            .insert(name, Handle::new(MacroDef::TempMacro(mcr)))
            .map_or(Ok(()), |_| {
                Err(SyntaxMatchError::RepeatedMacro(name))
            })?;
            */
        }
        Err(SyntaxMatchError::MatchError)
    }
}

impl Loading for ModuleItem {
    fn loading(
        parent: Option<Handle<FunctionDef>>,
        from_module: Handle<Module>,
        i: &Value,
    ) -> Result<(), SyntaxMatchError> {
        if let Ok(_) = FunctionDef::loading(parent.clone(), from_module.clone(), i) {
        } else if let Ok(_) = FunctionDef::loading(parent.clone(), from_module.clone(), i) {
        } else if let Ok(_) = FunctionDef::loading(parent.clone(), from_module.clone(), i) {
        } else {
            return Err(SyntaxMatchError::MatchError);
        }
        Ok(())
    }
}

impl Loading for Module {
    fn loading(
        _: Option<Handle<FunctionDef>>,
        from_module: Handle<Module>,
        i: &Value,
    ) -> Result<(), SyntaxMatchError> {
        let mut ctx = MatchRecord::default();
        match_template(&mut ctx, &MODULE_MATCH_TEMP, i)?;
        let name = ctx
            .maps
            .borrow()
            .get(&NAME_SYM.clone())
            .unwrap()
            .get_sym()
            .unwrap();
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
        body.iter()
            .try_for_each(|i| ModuleItem::loading(None, modu.clone(), i))?;
        Ok(())
    }
}
