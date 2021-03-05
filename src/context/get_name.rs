use super::*;

pub trait GetName {
    fn get_name(&self) -> Handle<Symbol>;
}

macro_rules! impl_get_name {
    ($tp:path) => {
        impl GetName for $tp {
            fn get_name(&self) -> Handle<Symbol> {
                self.name.clone()
            }
        }
    };
}

impl_get_name!(TempMacro);
impl_get_name!(ProcessMacro);
impl_get_name!(UserFunctionDef);
impl_get_name!(NativeFunctionDef);
impl_get_name!(Module);

impl GetName for MacroDef {
    fn get_name(&self) -> Handle<Symbol> {
        match self {
            MacroDef::TempMacro(x) => x.get_name(),
            MacroDef::ProcessMacro(x) => x.get_name(),
        }
    }
}

impl GetName for FunctionDef {
    fn get_name(&self) -> Handle<Symbol> {
        match self {
            FunctionDef::UserFunction(x) => x.get_name(),
            FunctionDef::NativeFunction(x) => x.get_name(),
            FunctionDef::Closure(_, x) => x.get_name(),
        }
    }
}
