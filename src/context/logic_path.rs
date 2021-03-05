use std::collections::VecDeque;

use super::*;
use crate::values::*;

pub trait LogicPath {
    fn logic_path(&self) -> SymbolList;
}

impl LogicPath for MacroDef {
    fn logic_path(&self) -> SymbolList {
        match self {
            MacroDef::TempMacro(x) => x.logic_path(),
            MacroDef::ProcessMacro(x) => x.logic_path(),
        }
    }
}

impl LogicPath for TempMacro {
    fn logic_path(&self) -> SymbolList {
        let mut r = VecDeque::new();
        r.append(&mut self.from_module.logic_path());
        r.push_back(self.name.clone());
        r
    }
}

impl LogicPath for ProcessMacro {
    fn logic_path(&self) -> SymbolList {
        let mut r = VecDeque::new();
        r.append(&mut self.from_module.logic_path());
        r.push_back(self.name.clone());
        r
    }
}

impl LogicPath for FunctionDef {
    fn logic_path(&self) -> SymbolList {
        match self {
            FunctionDef::UserFunction(x) => x.logic_path(),
            FunctionDef::NativeFunction(x) => x.logic_path(),
            FunctionDef::Closure(_, x) => x.logic_path(),
        }
    }
}

impl LogicPath for UserFunctionDef {
    fn logic_path(&self) -> SymbolList {
        let mut r = VecDeque::new();
        if let Some(x) = self.parent.clone() {
            r.append(&mut x.logic_path());
        } else {
            r.append(&mut self.from_module.logic_path())
        }
        r.push_back(self.name.clone());
        r
    }
}

impl LogicPath for NativeFunctionDef {
    fn logic_path(&self) -> SymbolList {
        let mut r = VecDeque::new();
        r.append(&mut self.from_module.logic_path());
        r.push_back(self.name.clone());
        r
    }
}

impl LogicPath for Module {
    fn logic_path(&self) -> SymbolList {
        let mut r = VecDeque::new();
        if let Some(x) = self.parent.clone() {
            r.append(&mut x.logic_path());
        }
        r.push_back(self.name.clone());
        r
    }
}

pub fn get_path(parent: Option<Handle<FunctionDef>>, from_module: Handle<Module>) -> SymbolList {
    parent.map_or_else(|| from_module.logic_path(), |x| x.logic_path())
}
