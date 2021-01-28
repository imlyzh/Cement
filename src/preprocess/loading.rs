use std::sync::Arc;

use super::symbols::*;
use crate::context::*;
use crate::error::SyntaxMatchError;
use crate::values::*;

use super::match_template::*;

impl MacroDef {
    pub fn loading(_from_module: Arc<Module>, _i: &Value) -> Result<Self, SyntaxMatchError> {
        todo!()
    }
}

impl FunctionDef {
    pub fn loading(
        _parent: Option<Arc<Self>>,
        _from_module: Arc<Module>,
        _i: &Value,
    ) -> Result<Self, SyntaxMatchError> {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct UseSentence(pub Arc<Symbol>);

enum ModuleItem {
    FunctionDef(FunctionDef),
    MacroDef(MacroDef),
    UseSentence(UseSentence),
}

impl Module {
    pub fn loading(parent: Option<Arc<Self>>, i: &ListPia) -> Result<Self, SyntaxMatchError> {
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

            let m = Module::new(name, parent);
            let body: Vec<Value> = body.0.map_or([].iter().map(Value::clone).collect(), |x| {
                x.iter().collect()
            });
            /*
            let body: Result<Vec<ModuleItem>, SyntaxMatchError> = body.iter()
                .map(|i| FunctionDef::loading(None, m.clone(), i))
                .collect();

                .for_each(|_x| {

                });
             */
            todo!()
        } else {
            let _r = Module::new(ANONYMOUS_MODULE_NAME.clone(), parent);
            todo!()
        }
    }
}
