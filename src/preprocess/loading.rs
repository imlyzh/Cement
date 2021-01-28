use super::symbols::*;
use crate::context::*;
use crate::error::SyntaxMatchError;
use crate::values::*;

use super::match_template::*;

#[derive(Debug, Clone)]
struct UseSentence(pub Handle<Symbol>);

#[derive(Debug)]
enum ModuleItem {
    FunctionDef(FunctionDef),
    MacroDef(MacroDef),
    UseSentence(UseSentence),
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

		}
		let mut ctx = MatchRecord::default();
		if match_template(&mut ctx, &USE_MATCH_TEMP1, i).is_ok() {

		}
		todo!()
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
        if let Ok(x) = MacroDef::loading(parent, from_module, i) {
            return Ok(ModuleItem::MacroDef(x));
        }
        Err(SyntaxMatchError::MatchError)
    }
}

impl Module {
    pub fn loading(parent: Option<Handle<Self>>, i: &ListPia) -> Result<Self, SyntaxMatchError> {
        if i.len() == 1 {
            let mut ctx = MatchRecord::default();
            match_template(&mut ctx, &MODULE_MATCH_TEMP, &i.car())?;
            let name = ctx
                .maps
                .borrow()
                .get(&Handle::new(Symbol::new("name")))
                .unwrap()
                .get_sym()
                .unwrap();
            let body = ctx
                .extend_maps
                .borrow()
                .get(&Handle::new(Symbol::new("name")))
                .unwrap()
                .clone();

            let modu = Module::new(name, parent);
            let body: Vec<Value> = body.0.map_or([].iter().map(Value::clone).collect(), |x| {
                x.iter().collect()
            });
            let result: Result<Vec<ModuleItem>, SyntaxMatchError> = body
                .iter()
                .map(|i| ModuleItem::loading(None, modu.clone(), i))
                .collect();
            result?.iter().for_each(|_x| {});
            todo!()
        } else {
            let _r = Module::new(ANONYMOUS_MODULE_NAME.clone(), parent);
            todo!()
        }
    }
}
