use gc::{Gc, GcCell, Trace, Finalize};
use crate::vm::value::Value;
use super::*;

pub type RJSFunc = fn (&Vec<Value>) -> JSResult;

#[derive(Trace, Finalize)]
pub struct PrimFunction {
    #[unsafe_ignore_trace]
    func: RJSFunc,
    name: &'static str
}

impl std::fmt::Debug for PrimFunction {
    fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write! (f, "builtin function: {}", self.name)
    }
}
impl PrimFunction {
    pub fn new (func: RJSFunc, name: &'static str) -> Self {
        Self {func, name}
    }
}

impl Objectable for PrimFunction {
    fn get (&self, prop: &String) -> Value {
        Value::Undefined
    }
    fn put (&mut self, prop: &String, val: Value) {}
    fn call (&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        let func = self.func;
        func(args)
    }
}
