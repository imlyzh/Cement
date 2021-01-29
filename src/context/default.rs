use crate::values::Handle;

use super::*;

impl Module {
    pub fn new(name: Handle<Symbol>, parent: Option<Handle<Module>>) -> Handle<Self> {
        Handle::new(Module {
            name,
            parent,
            module_table: RwLock::new(HashMap::new()),
            macro_table: RwLock::new(HashMap::new()),
            function_table: RwLock::new(HashMap::new()),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModuleHandle(pub Handle<Module>);

impl Default for ModuleHandle {
    fn default() -> Self {
        ModuleHandle(Module::new(ANONYMOUS_MODULE_NAME.clone(), None))
    }
}
