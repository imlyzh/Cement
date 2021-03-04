use crate::values::Handle;

use super::*;

impl Module {
    pub fn new(name: &Handle<Symbol>, parent: &Option<Handle<Module>>) -> Handle<Self> {
        Handle::new(Module {
            name: name.clone(),
            parent: parent.clone(),
			file_path: Handle::new(Symbol::new("prelude")),
            module_table: RwLock::new(HashMap::new()),
            macro_table: RwLock::new(HashMap::new()),
            constant_table: RwLock::new(HashMap::new()),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModuleHandle(pub Handle<Module>);

impl Default for ModuleHandle {
    fn default() -> Self {
        ModuleHandle(Module::new(&ANONYMOUS_MODULE_NAME, &None))
    }
}
