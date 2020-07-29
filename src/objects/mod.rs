#![allow(non_snake_case)]

pub mod function;
pub mod number;
pub mod object;
pub mod string;

use crate::vm::value::Value;
use crate::vm::vm::VM;
pub use gc::{Finalize, Gc, GcCell, Trace};
use std::collections::HashMap;

type GcBox<T> = Gc<GcCell<T>>;
pub type GcObject = GcBox<Object>;
pub type JSDict = std::collections::HashMap<String, Property>;
pub type JSResult = Result<Value, Value>;
pub type RJSFunc = fn(&mut VM, &[Value]) -> JSResult;

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
    String(string::String),
    Function(function::Function),
    PrimitiveFunction(function::PrimitiveFunction),
    Regular(object::Regular),
}

use std::cmp::PartialEq;

#[derive(PartialEq, Debug)]
pub enum PreferredType {
    Number,
    String,
}

impl Default for PreferredType {
    fn default() -> Self {
        Self::Number
    }
}

pub trait Objectable {
    fn Get(&self, prop: &String) -> Value;
    fn Put(&mut self, key: String, value: Value);
    fn CanPut(&self, key: &String) -> bool;
    fn HasProperty(&self, key: &String) -> bool;
    fn Construct(&self, _vm: &mut VM, _args: &[Value]) -> JSResult;
    fn Call(&self, _vm: &mut VM, _args: &[Value]) -> JSResult;
    fn valueOf(&self, vm: &mut VM) -> JSResult;
    fn toString(&self, gcobj: &GcObject, vm: &mut VM) -> JSResult;
    fn DefaultValue(&self, hint: Option<PreferredType>, gcobj: &GcObject, vm: &mut VM) -> JSResult;
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
    fn Call(&self, vm: &mut VM, args: &[Value]) -> JSResult {
        match &self.payload {
            ObjectPayload::Function(o) => o.Call(vm, args),
            ObjectPayload::PrimitiveFunction(o) => o.Call(vm, args),
            _ => Err("Object not callable".into()),
        }
    }
    fn valueOf(&self, vm: &mut VM) -> JSResult {
        match &self.payload {
            ObjectPayload::Number(o) => Ok(o.valueOf()),
            ObjectPayload::String(o) => Ok(o.valueOf()),
            ObjectPayload::Regular(o) => o.valueOf(self, vm),
            _ => Err("couldn't compute valueOf".into()),
        }
    }
    fn toString(&self, gcobj: &GcObject, vm: &mut VM) -> JSResult {
        match &self.payload {
            ObjectPayload::Number(o) => Ok(o.toString()),
            ObjectPayload::String(o) => Ok(o.toString()),
            ObjectPayload::Regular(o) => o.toString(gcobj, vm),
            _ => Err("couldn't compute toString".into()),
        }
    }
    fn DefaultValue(&self, hint: Option<PreferredType>, gcobj: &GcObject, vm: &mut VM) -> JSResult {
        let hint = hint.unwrap_or_default();
        let string = self.toString(gcobj, vm);
        let value = self.valueOf(vm);
        if hint == PreferredType::String {
            match string {
                Err(_) | Ok(Value::Object(_)) => (),
                Ok(v) => return Ok(v),
            }
            match value {
                Err(_) | Ok(Value::Object(_)) => Err("runtime error".into()),
                Ok(v) => Ok(v),
            }
        } else {
            match value {
                Err(_) | Ok(Value::Object(_)) => (),
                Ok(v) => return Ok(v),
            }
            match string {
                Err(_) | Ok(Value::Object(_)) => Err("runtime error".into()),
                Ok(v) => Ok(v),
            }
        }
    }
}
