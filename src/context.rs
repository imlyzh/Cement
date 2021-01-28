use std::{
    cell::RefCell,
    collections::{HashMap, LinkedList},
    fmt::Display,
    sync::{Arc, Mutex},
};

use crate::values::{Node, Symbol, Value};
use crate::{preprocess::symbols::*, values::ListPia};

macro_rules! impl_partial_eq {
    ($tp:path) => {
        impl PartialEq for $tp {
            fn eq(&self, other: &Self) -> bool {
                self.name == other.name && self.from_module == other.from_module
            }
        }
    };
}

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

#[derive(Debug)]
pub struct RuntimeError();

#[derive(Debug)]
pub struct CResult(pub Result<Value, RuntimeError>);

pub trait LogicPath {
    fn logic_path(&self) -> ListPia;
}

#[derive(Debug, PartialEq)]
pub enum MacroDef {
    TempMacro(TempMacro),
    ProcessMacro(ProcessMacro),
}

impl LogicPath for MacroDef {
    fn logic_path(&self) -> ListPia {
        match self {
            MacroDef::TempMacro(x) => x.logic_path(),
            MacroDef::ProcessMacro(x) => x.logic_path(),
        }
    }
}

#[derive(Debug)]
pub struct TempMacro {
    name: Arc<Symbol>,
    from_module: Arc<Module>,
    pairs: Vec<(Value, Value)>,
}

impl_partial_eq!(TempMacro);
impl_display!(TempMacro);

impl LogicPath for TempMacro {
    fn logic_path(&self) -> ListPia {
        Arc::new(Node(
            Value::Sym(self.name.clone()),
            Value::Pair(self.from_module.logic_path()),
        ))
    }
}

#[derive(Debug)]
pub struct ProcessMacro {
    name: Arc<Symbol>,
    from_module: Arc<Module>,
    body: Arc<FunctionDef>,
}

impl_partial_eq!(ProcessMacro);
impl_display!(ProcessMacro);

impl LogicPath for ProcessMacro {
    fn logic_path(&self) -> ListPia {
        Arc::new(Node(
            Value::Sym(self.name.clone()),
            Value::Pair(self.from_module.logic_path()),
        ))
    }
}

#[derive(Debug, PartialEq)]
pub enum FunctionDef {
    UserFunction(UserFunctionDef),
    NativeFunction(NativeFunctionDef),
}

impl LogicPath for FunctionDef {
    fn logic_path(&self) -> ListPia {
        match self {
            FunctionDef::UserFunction(x) => x.logic_path(),
            FunctionDef::NativeFunction(x) => x.logic_path(),
        }
    }
}

#[derive(Debug)]
pub struct UserFunctionDef {
    name: Arc<Symbol>,
    from_module: Arc<Module>,
    parent: Option<Arc<FunctionDef>>,
    params: Vec<Arc<Symbol>>,
    body: Vec<Value>,
}

impl_partial_eq!(UserFunctionDef);
impl_display!(UserFunctionDef);

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

#[derive(Debug)]
pub struct NativeFunctionDef {
    name: Arc<Symbol>,
    from_module: Arc<Module>,
    params: Option<Vec<Arc<Symbol>>>,
    is_pure: bool,
    body: extern "C" fn(Vec<Value>) -> CResult,
}

impl_partial_eq!(NativeFunctionDef);
impl_display!(NativeFunctionDef);

impl LogicPath for NativeFunctionDef {
    fn logic_path(&self) -> ListPia {
        Arc::new(Node(
            Value::Sym(self.name.clone()),
            Value::Pair(self.from_module.logic_path()),
        ))
    }
}

#[derive(Debug)]
pub struct Module {
    name: Arc<Symbol>,
    parent: Option<Arc<Module>>,
    module_table: Mutex<HashMap<Arc<Symbol>, Arc<Module>>>,
    macro_table: Mutex<HashMap<Arc<Symbol>, Arc<MacroDef>>>,
    function_table: Mutex<HashMap<Arc<Symbol>, Arc<FunctionDef>>>,
}

impl Module {
    pub fn new(name: Arc<Symbol>, parent: Option<Arc<Module>>) -> Self {
        Module {
            name,
            parent,
            module_table: Mutex::new(HashMap::new()),
            macro_table: Mutex::new(HashMap::new()),
            function_table: Mutex::new(HashMap::new()),
        }
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

impl Default for Module {
    fn default() -> Self {
        Self::new(ANONYMOUS_MODULE_NAME.clone(), None)
    }
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.parent == other.parent
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#<function {}>", self.name.to_string())
    }
}

#[derive(Debug, Default)]
pub struct EnvContext {
    pub module_table: HashMap<Arc<Symbol>, Arc<Module>>,
}

#[derive(Debug, Default)]
pub struct ThreadContext {
    pub env_context: Arc<EnvContext>,
    pub frame_stack: RefCell<LinkedList<FunctionContext>>,
}

#[derive(Debug)]
pub struct FunctionContext {
    pub namespace: HashMap<Arc<Symbol>, Value>,
    pub funcinfo: Arc<FunctionDef>,
}
