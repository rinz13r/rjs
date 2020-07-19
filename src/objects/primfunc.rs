use super::*;
use crate::vm::value::Value;
use gc::{Finalize, Gc, GcCell, Trace};

pub type RJSFunc = fn(&mut VM, &Vec<Value>) -> JSResult;

#[derive(Trace, Finalize)]
pub struct PrimFunction {
    #[unsafe_ignore_trace]
    func: RJSFunc,
    name: &'static str,
    dict: JSDict,
}

impl std::fmt::Debug for PrimFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "builtin function: {}", self.name)
    }
}
impl PrimFunction {
    pub fn new(func: RJSFunc, name: &'static str) -> Self {
        Self {
            func,
            name,
            dict: JSDict::new(),
        }
    }
}

impl Objectable for PrimFunction {
    fn get(&self, prop: &String) -> Value {
        Value::Undefined
    }
    fn put(&mut self, prop: &String, val: Value) {}
    fn call(&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        let func = self.func;
        func(vm, args)
    }
    fn toString(&self, _vm: &mut VM) -> JSResult {
        Ok(Value::String(format!("[builtin {}]", self.name)))
    }
    fn setPrototype(&mut self, prototype: GcBox<Object>) {}
}
