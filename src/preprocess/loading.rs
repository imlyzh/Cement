use std::vec;

use super::symbols::*;
use crate::context::*;
use crate::error::SyntaxMatchError;
use crate::values::*;

use super::match_template::*;

#[derive(Debug, Clone)]
struct UseSentence(pub Vec<Handle<Symbol>>);

#[derive(Debug)]
enum ModuleItem {
    UseSentence(UseSentence),
    FunctionDef(FunctionDef),
    MacroDef(MacroDef),
    ModuleDef(Module),
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
    type Output = Self;

    fn loading(
        _: Option<Handle<FunctionDef>>,
        _from_module: Handle<Module>,
        _i: &Value,
    ) -> Result<Self::Output, SyntaxMatchError> {
        todo!()
    }
}

impl Loading for FunctionDef {
    type Output = Self;

    fn loading(
        _parent: Option<Handle<Self>>,
        _from_module: Handle<Module>,
        _i: &Value,
    ) -> Result<Self::Output, SyntaxMatchError> {
        todo!()
    }
}

impl Loading for UseSentence {
    type Output = Self;

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
            return Ok(UseSentence(vec![r]));
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
            return Ok(UseSentence(r));
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

impl Module {
    pub fn loading(
        _: Option<Handle<FunctionDef>>,
        from_module: Handle<Module>,
        i: &Value,
    ) -> Result<Self, SyntaxMatchError> {
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

        let modu = Module::new(name, Some(from_module));
        let body: Vec<Value> = body.0.map_or([].iter().map(Value::clone).collect(), |x| {
            x.iter().collect()
        });
        let result: Result<Vec<ModuleItem>, SyntaxMatchError> = body
            .iter()
            .map(|i| ModuleItem::loading(None, modu.clone(), i))
            .collect();
        result?.iter().for_each(|_x| {});
        todo!()
    }
}
