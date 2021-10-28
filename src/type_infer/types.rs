


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
    Pair(Box<Type>, Box<Type>),
    List(Box<Type>),
    Callable(CallableType),
    Union(Vec<Type>),
    Any,
}

impl Type {
    pub fn union(self, other: Type) -> Type {
        match (self.clone(), other.clone()) {
            (Type::Union(mut types), Type::Union(types2)) => {
                for i in types2 {
                    if !types.contains(&i) {
                        types.push(i);
                    }
                }
                Type::Union(types)
            }
            (Type::Union(mut types), other) |
            (other, Type::Union(mut types)) => {
                if !types.contains(&other) {
                    types.push(other);
                }
                Type::Union(types)
            }
            (_, _) => Type::Union(vec![self, other]),
        }
    }
}

/// CallableType ([params type], is_var_len, return type)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallableType(pub Vec<Type>, pub bool, pub Box<Type>);
