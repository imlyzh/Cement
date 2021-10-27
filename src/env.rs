use std::{cell::RefCell, collections::HashMap, sync::{Arc, RwLock}};

use sexpr_ir::gast::symbol::Symbol;

use crate::{ast::{FunctionDef, callable::Lambda}, type_infer::types::{CallableType, Type}};


#[derive(Debug, Clone)]
pub struct Env<T>(pub Arc<RwLock<HashMap<Symbol, T>>>, Option<Arc<Env<T>>>);

impl<T> Env<T> {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(HashMap::new())), None)
    }
}

impl<T: Clone> Env<T> {
    pub fn get_item(&self, k: &Symbol) -> Option<T> {
        if let Some(x) = self.0.read().unwrap().get(k) {
            Some((*x).clone())
        } else {
            self.1.as_ref().and_then(|x| x.get_item(k))
        }
    }
    pub fn add(&self, k: Symbol, v: T) {
        self.0.write().unwrap().insert(k, v);
    }
    pub fn new_level(self: Arc<Self>) -> Self {
        Self(Default::default(), Some(self))
    }
}

impl<T: Clone> Default for Env<T> {
    fn default() -> Self {
        Self::new()
    }
}
