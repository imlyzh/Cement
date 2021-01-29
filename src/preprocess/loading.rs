use get_name::GetName;

use super::symbols::*;
use crate::context::*;
use crate::error::SyntaxMatchError;
use crate::values::*;

use super::match_template::*;

#[derive(Debug, Clone)]
struct UseSentence(pub Vec<Handle<Symbol>>);

#[derive(Debug)]
enum ModuleItem {
    UseSentence(Handle<UseSentence>),
    FunctionDef(Handle<FunctionDef>),
    MacroDef(Handle<MacroDef>),
    ModuleDef(Handle<Module>),
}

pub trait Loading {
    type Output;
    fn loading(
        parent: Option<Handle<FunctionDef>>,
        from_module: Handle<Module>,
        i: &Value,
    ) -> Result<Self::Output, SyntaxMatchError>;
}

impl Loading for MacroDef {
    type Output = Handle<Self>;

    fn loading(
        _: Option<Handle<FunctionDef>>,
        from_module: Handle<Module>,
        i: &Value,
    ) -> Result<Self::Output, SyntaxMatchError> {
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
            return Ok(Handle::new(MacroDef::TempMacro(mcr)));
        }
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
                name,
                from_module,
                pairs,
            };
            return Ok(Handle::new(MacroDef::TempMacro(mcr)));
        }
        Err(SyntaxMatchError::MatchError)
    }
}

impl Loading for FunctionDef {
    type Output = Handle<Self>;

    fn loading(
        _parent: Option<Handle<Self>>,
        _from_module: Handle<Module>,
        _i: &Value,
    ) -> Result<Self::Output, SyntaxMatchError> {
        todo!()
    }
}

impl Loading for UseSentence {
    type Output = Handle<Self>;

    fn loading(
        _: Option<Handle<FunctionDef>>,
        _from_module: Handle<Module>,
        i: &Value,
    ) -> Result<Self::Output, SyntaxMatchError> {
        let mut ctx = MatchRecord::default();
        if match_template(&mut ctx, &USE_MATCH_TEMP, i).is_ok() {
            let r = ctx
                .maps
                .borrow()
                .get(&NAME_SYM.clone())
                .unwrap()
                .get_sym()
                .unwrap();
            return Ok(Handle::new(UseSentence(vec![r])));
        }
        let mut ctx = MatchRecord::default();
        if match_template(&mut ctx, &USE_MATCH_TEMP1, i).is_ok() {
            let r = ctx
                .extend_maps
                .borrow()
                .get(&NAME_SYM.clone())
                .unwrap()
                .clone();
            let r: Vec<Handle<Symbol>> = NodeIter::from(r).map(|x| x.get_sym().unwrap()).collect();
            return Ok(Handle::new(UseSentence(r)));
        }
        Err(SyntaxMatchError::MatchError)
    }
}

impl Loading for ModuleItem {
    type Output = Self;

    fn loading(
        parent: Option<Handle<FunctionDef>>,
        from_module: Handle<Module>,
        i: &Value,
    ) -> Result<Self::Output, SyntaxMatchError> {
        if let Ok(x) = FunctionDef::loading(parent.clone(), from_module.clone(), i) {
            return Ok(ModuleItem::FunctionDef(x));
        }
        if let Ok(x) = UseSentence::loading(parent.clone(), from_module.clone(), i) {
            return Ok(ModuleItem::UseSentence(x));
        }
        if let Ok(x) = Module::loading(parent.clone(), from_module.clone(), i) {
            return Ok(ModuleItem::ModuleDef(x));
        }
        if let Ok(x) = MacroDef::loading(parent, from_module, i) {
            return Ok(ModuleItem::MacroDef(x));
        }
        Err(SyntaxMatchError::MatchError)
    }
}

impl Loading for Module {
    type Output = Handle<Self>;
    fn loading(
        _: Option<Handle<FunctionDef>>,
        from_module: Handle<Module>,
        i: &Value,
    ) -> Result<Self::Output, SyntaxMatchError> {
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

        let modu = Module::new(name, Some(from_module));
        let result: Result<Vec<ModuleItem>, SyntaxMatchError> = body
            .iter()
            .map(|i| ModuleItem::loading(None, modu.clone(), i))
            .collect();
        result?.iter().try_for_each(|x| match x {
            ModuleItem::UseSentence(_) => {
                todo!("register use module");
            }
            ModuleItem::FunctionDef(x) => {
                modu.function_table
                    .write()
                    .unwrap()
                    .insert(x.get_name(), x.clone())
                    .map_or(Ok(()), |_| {
                        Err(SyntaxMatchError::RepeatedFunction(x.get_name()))
                    })?;
                Ok(())
            }
            ModuleItem::MacroDef(x) => {
                modu.macro_table
                    .write()
                    .unwrap()
                    .insert(x.get_name(), x.clone())
                    .map_or(Ok(()), |_| {
                        Err(SyntaxMatchError::RepeatedMacro(x.get_name()))
                    })?;
                Ok(())
            }
            ModuleItem::ModuleDef(x) => {
                modu.module_table
                    .write()
                    .unwrap()
                    .insert(x.get_name(), x.clone())
                    .map_or(Ok(()), |_| {
                        Err(SyntaxMatchError::RepeatedModule(x.get_name()))
                    })?;
                Ok(())
            }
        })?;

        return Ok(modu);
    }
}
