use crate::vm::code::Code;
use crate::vm::value::*;
use crate::vm::vm::VM;

extern crate gc;
use gc::{Finalize, Gc, Trace};

use std::collections::HashMap;
use std::rc::Rc;

#[derive(Trace, Finalize)]
pub struct Function {
    #[unsafe_ignore_trace]
    pub code: Rc<Code>,
    pub arity: usize,
}

impl Function {
    pub fn new(code: Rc<Code>, arity: usize) -> Self {
        Function { code, arity }
    }
}

pub type JSRustFunc = fn(&Vec<Value>) -> Result<Value, &'static str>;

#[derive(Trace, Finalize)]
pub struct BuiltInFunction {
    #[unsafe_ignore_trace]
    pub func: JSRustFunc,
}

#[derive(Trace, Finalize)]
pub struct RegularObject {
    dict: HashMap<String, Value>,
}

#[derive(Trace, Finalize)]
pub enum ObjectPayload {
    BuiltInFunction(BuiltInFunction),
    Function(Function),
    RegularObject(RegularObject),
}

pub trait Objectable {
    fn get(&self, prop: &String) -> Result<Value, &'static str> {
        Err("No such prop")
    }
    fn put(&mut self, prop: &String, val: Value) {}
    fn has_property(&self, prop: &String) -> bool {
        false
    }
    fn delete(&mut self, prop: &String) {}
    fn call(&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        Err("object not callable")
    }
    fn spawn(&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        Err("Cannot call new (...)")
    }
}

impl Objectable for BuiltInFunction {
    fn call(&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        let f = self.func;
        f(args)
    }
}

impl Objectable for Function {
    fn call(&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        let res = vm.call_code(self.code.clone(), self.arity, args);
        res
    }
    fn spawn(&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        let res = vm.call_code(self.code.clone(), self.arity, args);
        res
    }
}

impl RegularObject {
    pub fn new() -> Self {
        RegularObject {
            dict: HashMap::new(),
        }
    }
}

impl Objectable for RegularObject {
    fn get(&self, prop: &String) -> Result<Value, &'static str> {
        if let Some(val) = self.dict.get(prop) {
            Ok(val.clone())
        } else {
            Err("No such prop")
        }
    }
    fn put(&mut self, prop: &String, val: Value) {
        self.dict.insert(prop.clone(), val);
    }
    fn has_property(&self, prop: &String) -> bool {
        match self.dict.get(prop) {
            Some(_) => true,
            None => false,
        }
    }
    fn delete(&mut self, prop: &String) {
        self.dict.remove(prop);
    }
}
