


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Nil,
    Bool,
    Int,
    Uint,
    Float,
    String,
    Symbol,
    Vector(Box<Type>),
    Pair(Box<(Type, Type)>),
    List(Box<Type>),
    Callable(),
    Union(Vec<Type>),
    Any,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallableType(pub Vec<Type>, pub bool);
