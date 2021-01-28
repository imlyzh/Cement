use super::*;
use crate::values::*;

pub trait LogicPath {
    fn logic_path(&self) -> ListPia;
}

impl LogicPath for MacroDef {
    fn logic_path(&self) -> ListPia {
        match self {
            MacroDef::TempMacro(x) => x.logic_path(),
            MacroDef::ProcessMacro(x) => x.logic_path(),
        }
    }
}

impl LogicPath for TempMacro {
    fn logic_path(&self) -> ListPia {
        Arc::new(Node(
            Value::Sym(self.name.clone()),
            Value::Pair(self.from_module.logic_path()),
        ))
    }
}

impl LogicPath for ProcessMacro {
    fn logic_path(&self) -> ListPia {
        Arc::new(Node(
            Value::Sym(self.name.clone()),
            Value::Pair(self.from_module.logic_path()),
        ))
    }
}

impl LogicPath for FunctionDef {
    fn logic_path(&self) -> ListPia {
        match self {
            FunctionDef::UserFunction(x) => x.logic_path(),
            FunctionDef::NativeFunction(x) => x.logic_path(),
        }
    }
}

impl LogicPath for UserFunctionDef {
    fn logic_path(&self) -> ListPia {
        self.parent.clone().map_or(
            Arc::new(Node(
                Value::Sym(self.name.clone()),
                Value::Pair(self.from_module.logic_path()),
            )),
            |x| {
                Arc::new(Node(
                    Value::Sym(self.name.clone()),
                    Value::Pair(x.logic_path()),
                ))
            },
        )
    }
}

impl LogicPath for NativeFunctionDef {
    fn logic_path(&self) -> ListPia {
        Arc::new(Node(
            Value::Sym(self.name.clone()),
            Value::Pair(self.from_module.logic_path()),
        ))
    }
}

impl LogicPath for Module {
    fn logic_path(&self) -> ListPia {
        self.parent.clone().map_or(
            Arc::new(Node(Value::Sym(self.name.clone()), Value::Nil)),
            |x| {
                Arc::new(Node(
                    Value::Sym(self.name.clone()),
                    Value::Pair(x.logic_path()),
                ))
            },
        )
    }
}
