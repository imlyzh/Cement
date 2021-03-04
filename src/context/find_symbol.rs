use crate::error::*;
use crate::utils::*;
use crate::values::*;

use super::*;

pub trait FindSymbol {
    fn find_symbol(&self, k: &str) -> Option<Value>;
}

impl FindSymbol for Module {
    fn find_symbol(&self, k: &str) -> Option<Value> {
        self.constant_table
            .read()
            .unwrap()
            .get(&Symbol::new(k))
            .map(Value::clone)
    }
}

impl FindSymbol for FunctionContext {
    fn find_symbol(&self, k: &str) -> Option<Value> {
        self.namespace.get(&Symbol::new(k)).map(Value::clone)
    }
}

pub trait FindPath {
    fn find_path(&self, k: &[Symbol]) -> Result<Value, RuntimeError>;
}

impl FindPath for Module {
    fn find_path(&self, k: &[Symbol]) -> Result<Value, RuntimeError> {
        let id = k.get(0).map_or(Err(RuntimeError::ModuleIsNotValue), Ok)?;
        if k.len() == 1 {
            let r = self.constant_table.read().unwrap();
            let r = r
                .get(id)
                .map_or(Err(RuntimeError::SymbolNotFound(id.clone())), |x| {
                    Ok(x.clone())
                })?;
            return Ok(r);
        }
        let rg = self.module_table.read().unwrap();
        let module = rg
            .get(id)
            .map_or(Err(RuntimeError::SymbolNotFound(id.clone())), Ok)?;
        module.find_path(&k[1..])
    }
}

impl FindPath for EnvContext {
    fn find_path(&self, k: &[Symbol]) -> Result<Value, RuntimeError> {
        let id = k.get(0).unwrap();
        let mg = self.module_table.lock().unwrap();
        let module = mg
            .get(id)
            .map_or(Err(RuntimeError::SymbolNotFound(id.clone())), Ok)?;
        module.find_path(&k[1..])
    }
}

impl FindPath for ThreadContext {
    fn find_path(&self, k: &[Symbol]) -> Result<Value, RuntimeError> {
        if k.len() == 1 {
            let sym = unsafe { k.get_unchecked(0) };
            let k = &*sym.id;
            return self
                .frame_stack
                .borrow()
                .back()
                .map_or(Err(RuntimeError::SymbolNotFound(sym.clone())), Ok)?
                .find_symbol(k)
                .map_or(Err(RuntimeError::SymbolNotFound(sym.clone())), Ok);
        }
        self.env_context.find_path(k)
    }
}

pub fn find_symbol(env: &ThreadContext, k: &Symbol) -> Result<Value, RuntimeError> {
    let k: Vec<Symbol> =
        k.id.split('.')
            .map(|id| Symbol {
                id: string_intern(id),
                line: k.line,
                colum: k.colum,
                pos: k.pos,
                scope: k.scope.clone(),
            })
            .collect();
    env.find_path(&k)
}
