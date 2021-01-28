use std::fmt::Display;

use super::logic_path::*;
use super::*;

macro_rules! impl_display {
    ($tp:path) => {
        impl Display for $tp {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut path = self
                    .logic_path()
                    .iter()
                    .map(|x| (*x.get_sym().unwrap().id).clone())
                    .collect::<Vec<_>>();
                path.reverse();
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
