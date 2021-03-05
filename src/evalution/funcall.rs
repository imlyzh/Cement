use crate::values::*;
use crate::context::*;



pub trait Callable {
	fn call(&self, ctx: &ThreadContext, args: &[Value]) -> CResult;
}

impl Callable for FunctionDef {
    fn call(&self, ctx: &ThreadContext, args: &[Value]) -> CResult {
        match self {
            FunctionDef::UserFunction(x) => x.call(ctx, args),
            FunctionDef::NativeFunction(x) => x.call(ctx, args),
            FunctionDef::Closure(_, _) => unimplemented!()
        }
    }
}

impl Callable for NativeFunctionDef {
    fn call(&self, _ctx: &ThreadContext, args: &[Value]) -> CResult {
        (self.body)(args.to_vec())
    }
}

impl Callable for UserFunctionDef {
    fn call(&self, _ctx: &ThreadContext, _args: &[Value]) -> CResult {
        todo!()
    }
}