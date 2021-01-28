use crate::values::Handle;

use super::*;

impl Module {
    pub fn new(name: Arc<Symbol>, parent: Option<Arc<Module>>) -> Handle<Self> {
        Handle::new(Module {
            name,
            parent,
            module_table: Mutex::new(HashMap::new()),
            macro_table: Mutex::new(HashMap::new()),
            function_table: Mutex::new(HashMap::new()),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModuleHandle (pub Handle<Module>);

impl Default for ModuleHandle {
    fn default() -> Self {
		ModuleHandle(Module::new(ANONYMOUS_MODULE_NAME.clone(), None))
    }
}
