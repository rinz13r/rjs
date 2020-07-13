pub mod proto;
pub mod array;
pub mod primfunc;
pub mod function;

use gc::{Gc, GcCell, Trace, Finalize};
use crate::vm::value::Value;
use crate::vm::vm::VM;
use crate::vm::context::Context;
use crate::vm::code::Code;
use std::rc::Rc;

pub type GcBox<T> = Gc<GcCell<T>>;
pub type JSDict = std::collections::HashMap<String, Value>;
// TODO: Update after adding Error object
pub type JSResult = Result<Value, &'static str>;
pub type RJSFunc = fn (&Vec<Value>) -> JSResult;

pub trait Objectable {
    fn get (&self, prop: &String) -> Value;
    fn put (&mut self, prop: &String, val: Value) {}
    fn call (&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        Err ("Object not callable")
    }
}

#[derive(Trace, Finalize, Debug)]
pub enum Object {
    ArrayObject (array::ArrayObject),
    ProtoObject (proto::ProtoObject),
    PrimFunction (primfunc::PrimFunction),
    FunctionObject (function::FunctionObject),
}

impl Object {
    pub fn from_rjsfunc (func: RJSFunc, name: &'static str) -> Self {
        Object::PrimFunction (primfunc::PrimFunction::new (func, name))
    }
    pub fn new_functionobject (ctx: &Context, code: Rc<Code>, length: usize) -> Self {
        Object::FunctionObject(function::FunctionObject::new (ctx, code, length, &String::from ("func")))
    }
}

impl Objectable for Object {
    fn get (&self, prop: &String) -> Value {
        match self {
            Object::ArrayObject (o) => o.get (prop),
            Object::ProtoObject (o) => o.get (prop),
            Object::PrimFunction(o) => o.get (prop),
            Object::FunctionObject(o) => o.get (prop),
        }
    }
    fn put (&mut self, prop: &String, val: Value) {
        match self {
            Object::ArrayObject (o) => o.put (prop, val), 
            Object::ProtoObject (o) => o.put (prop, val), 
            Object::PrimFunction(o) => o.put (prop, val), 
            Object::FunctionObject(o) => o.put (prop, val),
        }
    }
    fn call (&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        match self {
            Object::ArrayObject (o) => o.call (vm, args), 
            Object::ProtoObject (o) => o.call (vm, args), 
            Object::PrimFunction(o) => o.call (vm, args), 
            Object::FunctionObject(o) => o.call (vm, args),
        }
    }
}
