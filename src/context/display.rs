use std::fmt::Display;

use super::logic_path::*;
use super::*;

macro_rules! impl_display {
    ($tp:path) => {
        impl Display for $tp {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let path = self.logic_path();
                let path = path.iter().map(|x| x.id.as_str()).collect::<Vec<_>>();
                write!(f, "#<function {}>", path.join("."))
            }
        }
    };
}

impl_display!(TempMacro);
impl_display!(ProcessMacro);
impl_display!(UserFunctionDef);
impl_display!(NativeFunctionDef);
impl_display!(Module);
