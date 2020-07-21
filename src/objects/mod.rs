#![allow(non_snake_case)]

pub mod array;
pub mod function;
pub mod primfunc;
pub mod proto;
pub mod regobject;

use crate::vm::code::Code;
use crate::vm::context::Context;
use crate::vm::value::Value;
use crate::vm::vm::VM;
use gc::{Finalize, Gc, GcCell, Trace};
use std::rc::Rc;

pub type GcBox<T> = Gc<GcCell<T>>;
pub type JSDict = std::collections::HashMap<String, Value>;
// TODO: Update after adding Error object
pub type JSResult = Result<Value, Value>;
pub type RJSFunc = fn(&mut VM, &Vec<Value>) -> JSResult;

pub trait Objectable {
    fn get(&self, prop: &String) -> Value;
    fn put(&mut self, prop: &String, val: Value) {}
    fn call(&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        Err(Value::from_str("Object not callable"))
    }
    // TODO: Override
    fn toString(&self, _vm: &mut VM) -> JSResult {
        Ok(Value::String(String::from("[object Object]")))
    }
    fn spawn(&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        Err(Value::from_str("Not constructible"))
    }
    fn setPrototype(&mut self, prototype: GcBox<Object>);
}

pub enum DefaultValueHint {
    String,
    Number,
}

pub trait DefaultOperations {
    fn toString(&self) -> Value; // return Value::String
    fn valueOf(&self) -> Value;
    fn defaultValue(&self, hint: DefaultValueHint) -> Value;
}

#[derive(Trace, Finalize, Debug)]
pub enum Object {
    ArrayObject(array::ArrayObject),
    ProtoObject(proto::ProtoObject),
    PrimFunction(primfunc::PrimFunction),
    FunctionObject(function::FunctionObject),
    RegObject(regobject::RegObject),
}

impl Object {
    pub fn from_rjsfunc(func: RJSFunc, name: &'static str) -> Self {
        Object::PrimFunction(primfunc::PrimFunction::new(func, name))
    }
    pub fn new_functionobject(ctx: &Context, code: Rc<Code>, length: usize) -> Self {
        Object::FunctionObject(function::FunctionObject::new(
            ctx,
            code,
            length,
            &String::from("func"),
        ))
    }
    pub fn new_regobject() -> Self {
        Object::RegObject(regobject::RegObject::new())
    }
    pub fn new_arrayobject(ctx: &Context, els: Vec<Value>) -> Self {
        Object::ArrayObject(array::ArrayObject::new(ctx, els))
    }
}

impl Objectable for Object {
    fn get(&self, prop: &String) -> Value {
        match self {
            Object::ArrayObject(o) => o.get(prop),
            Object::ProtoObject(o) => o.get(prop),
            Object::PrimFunction(o) => o.get(prop),
            Object::FunctionObject(o) => o.get(prop),
            Object::RegObject(o) => o.get(prop),
        }
    }
    fn put(&mut self, prop: &String, val: Value) {
        match self {
            Object::ArrayObject(o) => o.put(prop, val),
            Object::ProtoObject(o) => o.put(prop, val),
            Object::PrimFunction(o) => o.put(prop, val),
            Object::FunctionObject(o) => o.put(prop, val),
            Object::RegObject(o) => o.put(prop, val),
        }
    }
    fn call(&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        match self {
            Object::ArrayObject(o) => o.call(vm, args),
            Object::ProtoObject(o) => o.call(vm, args),
            Object::PrimFunction(o) => o.call(vm, args),
            Object::FunctionObject(o) => o.call(vm, args),
            Object::RegObject(o) => o.call(vm, args),
        }
    }
    fn spawn(&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        match self {
            Object::ArrayObject(o) => o.spawn(vm, args),
            Object::ProtoObject(o) => o.spawn(vm, args),
            Object::PrimFunction(o) => o.spawn(vm, args),
            Object::FunctionObject(o) => o.spawn(vm, args),
            Object::RegObject(o) => o.spawn(vm, args),
        }
    }
    fn toString(&self, vm: &mut VM) -> JSResult {
        match self {
            Object::ArrayObject(o) => o.toString(vm),
            Object::ProtoObject(o) => o.toString(vm),
            Object::PrimFunction(o) => o.toString(vm),
            Object::FunctionObject(o) => o.toString(vm),
            Object::RegObject(o) => o.toString(vm),
        }
    }
    fn setPrototype(&mut self, prototype: GcBox<Object>) {
        match self {
            Object::ArrayObject(o) => o.setPrototype(prototype),
            Object::ProtoObject(o) => o.setPrototype(prototype),
            Object::PrimFunction(o) => o.setPrototype(prototype),
            Object::FunctionObject(o) => o.setPrototype(prototype),
            Object::RegObject(o) => o.setPrototype(prototype),
        }
    }
}
