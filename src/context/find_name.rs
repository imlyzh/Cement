use crate::values::*;

use super::*;

pub trait FindName {
    type Output;
    fn find_name(&self, i: &str) -> Option<Self::Output>;
}

impl FindName for Module {
    type Output = Value;

    fn find_name(&self, i: &str) -> Option<Self::Output> {
        // i.split('.');
        self.constant_table
            .read()
            .unwrap()
            .get(&Symbol::new(i))
            .map(Value::clone)
    }
}

impl FindName for EnvContext {
    type Output = Value;

    fn find_name(&self, _i: &str) -> Option<Self::Output> {
        todo!()
    }
}

impl FindName for FunctionContext {
    type Output = Value;

    fn find_name(&self, _i: &str) -> Option<Self::Output> {
        // self.namespace
        todo!()
    }
}
