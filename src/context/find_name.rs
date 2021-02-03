use crate::values::*;

use super::*;

trait FindName {
    type Output;
    fn find_name(i: &str) -> Option<Self::Output>;
}

impl FindName for Module {
    type Output = Value;

    fn find_name(i: &str) -> Option<Self::Output> {
        todo!()
    }
}

impl FindName for EnvContext {
    type Output = Value;

    fn find_name(i: &str) -> Option<Self::Output> {
        todo!()
    }
}

impl FindName for FunctionContext {
    type Output = Value;

    fn find_name(i: &str) -> Option<Self::Output> {
        todo!()
    }
}
