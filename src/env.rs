use std::{collections::HashMap, sync::Arc};

use sexpr_ir::gast::symbol::Symbol;


#[derive(Debug, Clone, Default)]
pub struct Env<T>(pub HashMap<Symbol, T>, Option<Arc<Env<T>>>);

impl<T> Env<T> {
    pub fn get_item(&self, k: &Symbol) -> Option<&T> {
        if let Some(x) = self.0.get(k) {
            Some(x)
        } else {
            self.1.as_ref().and_then(|x| x.get_item(k))
        }
    }
    pub fn add(self, k: Symbol, v: T) -> Self {
        let Env(mut map, f) = self;
        map.insert(k, v);
        Self(map, f)
    }
    pub fn new_level(self: Arc<Self>) -> Self {
        Self(Default::default(), Some(self))
    }
}