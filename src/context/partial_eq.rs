use super::*;

macro_rules! impl_partial_eq {
    ($tp:path) => {
        impl PartialEq for $tp {
            fn eq(&self, other: &Self) -> bool {
                self.name == other.name && self.from_module == other.from_module
            }
        }
    };
}

impl_partial_eq!(TempMacro);
impl_partial_eq!(ProcessMacro);
impl_partial_eq!(UserFunctionDef);
impl_partial_eq!(NativeFunctionDef);

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.parent == other.parent
    }
}
