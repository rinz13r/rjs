#![allow(non_snake_case)]

pub mod function;
pub mod number;
pub mod object;

use crate::vm::code::Code;
use crate::vm::context::Context;
use crate::vm::value::Value;
use crate::vm::vm::VM;
pub use gc::{Finalize, Gc, GcCell, Trace};
use std::collections::HashMap;
use std::rc::Rc;

type GcBox<T> = Gc<GcCell<T>>;
pub type GcObject = GcBox<Object>;
pub type JSDict = std::collections::HashMap<String, Property>;
pub type JSResult = Result<Value, Value>;
pub type RJSFunc = fn(&mut VM, this: Value, &[Value]) -> JSResult;

#[derive(Trace, Finalize, Debug)]
pub struct Property {
    value: Value,
    read_only: bool,
    dont_enum: bool,
    dont_delete: bool,
    internal: bool,
}

impl Property {
    pub fn new(value: Value) -> Self {
        Property {
            value,
            read_only: false,
            dont_enum: false,
            dont_delete: false,
            internal: false,
        }
    }
}

#[derive(Trace, Finalize, Debug)]
pub struct Object {
    pub __proto__: Option<GcObject>,
    pub payload: ObjectPayload,
    pub props: HashMap<String, Property>,
}

#[derive(Trace, Finalize, Debug)]
pub enum ObjectPayload {
    Number(number::Number),
    Function(function::Function),
    PrimitiveFunction(function::PrimitiveFunction),
    Regular,
}

pub trait Objectable {
    fn Get(&self, prop: &String) -> Value;
    fn Put(&mut self, key: String, value: Value);
    fn CanPut(&self, key: &String) -> bool;
    fn HasProperty(&self, key: &String) -> bool;
    fn Construct(&self, _vm: &mut VM, _args: &[Value]) -> JSResult;
    fn Call(&self, _vm: &mut VM, this: Value, _args: &[Value]) -> JSResult;
}

impl Objectable for Object {
    fn Get(&self, key: &String) -> Value {
        match self.props.get(key) {
            Some(Property { value, .. }) => value.clone(),
            None => {
                if let Some(ref proto) = self.__proto__ {
                    proto.borrow().Get(key)
                } else {
                    Value::default()
                }
            }
        }
    }
    fn Put(&mut self, key: String, value: Value) {
        self.props.insert(key, Property::new(value));
    }
    fn CanPut(&self, key: &String) -> bool {
        if let Some(prop) = self.props.get(key) {
            if prop.read_only {
                false
            } else {
                true
            }
        } else {
            if let Some(ref proto) = self.__proto__ {
                proto.borrow().CanPut(key)
            } else {
                true
            }
        }
    }
    fn HasProperty(&self, key: &String) -> bool {
        match self.props.get(key) {
            Some(_) => true,
            _ => false,
        }
    }
    fn Construct(&self, vm: &mut VM, args: &[Value]) -> JSResult {
        match &self.payload {
            // ObjectPayload::PrimFunction(o) => o.Call(vm, args),
            ObjectPayload::Function(o) => o.Construct(vm, args),
            ObjectPayload::PrimitiveFunction(o) => o.Construct(vm, args),
            _ => Err("Object not constructible".into()),
        }
    }
    fn Call(&self, vm: &mut VM, this: Value, args: &[Value]) -> JSResult {
        match &self.payload {
            // ObjectPayload::PrimFunction(o) => o.Call(vm, args),
            ObjectPayload::Function(o) => o.Call(vm, this, args),
            ObjectPayload::PrimitiveFunction(o) => o.Call(vm, this, args),
            _ => Err("Object not callable".into()),
        }
    }
}
