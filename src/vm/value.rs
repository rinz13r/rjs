use crate::vm::code::Code;
use crate::vm::payload::*;
use crate::vm::vm::VM;

extern crate gc;
use gc::{Finalize, Gc, GcCell, Trace};

use std::rc::Rc;

#[derive(Clone, Trace, Finalize, Debug)]
pub enum Value {
    Null,
    Undefined,
    Number(Number),
    Boolean(bool),
    String(String),
    Object(GcCell<Gc<Object>>),
}

pub type JSResult = Result<Value, &'static str>;

#[derive(Trace, Finalize, Debug)]
struct Property {
    val: Value,
}

#[derive(Trace, Finalize, Debug)]
pub struct Object {
    prototype: Option<GcCell<Gc<Object>>>,
    payload: ObjectPayload,
}

impl Object {
    pub fn call(&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        match &self.payload {
            ObjectPayload::BuiltInFunction(BuiltInFunction { func }) => func(args),
            ObjectPayload::Function(func) => vm.call_code(func.code.clone(), func.arity, args),
            _ => Err("object not callable"),
        }
    }
    pub fn new_builtin_function(func: JSRustFunc) -> Object {
        Object {
            prototype: None,
            payload: ObjectPayload::BuiltInFunction(BuiltInFunction { func }),
        }
    }
    pub fn new_function(code: Rc<Code>, arity: usize) -> Object {
        Object {
            prototype: None,
            payload: ObjectPayload::Function(Function { code, arity }),
        }
    }
    pub fn spawn(&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        match &self.payload {
            ObjectPayload::Function(f) => f.spawn(vm, args),
            _ => Err("New cannot be called"),
        }
    }
    pub fn new_regular_object(prototype: Option<Value>) -> Self {
        Object {
            prototype: if let Some(prototype) = prototype {
                match &prototype {
                    Value::Object(o) => Some(o.clone()),
                    _ => None,
                }
            } else {
                None
            },
            payload: ObjectPayload::RegularObject(RegularObject::new()),
        }
    }
}

impl std::fmt::Debug for ObjectPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ObjectPayLoad")
    }
}

#[derive(Clone, Trace, Finalize, Debug)]
pub enum Number {
    NaN,
    IaN(f64),
}

impl std::string::ToString for Number {
    fn to_string(&self) -> String {
        match self {
            Number::NaN => "NaN".to_string(),
            Number::IaN(n) => n.to_string(),
        }
    }
}

impl Value {
    fn to_boolean(&self) -> Value {
        match self {
            Value::Undefined | Value::Null => Value::Boolean(false),
            Value::Boolean(_) => self.clone(),
            Value::Number(n) => match n {
                Number::NaN | Number::IaN(0.) => Value::Boolean(false),
                _ => Value::Boolean(true),
            },
            Value::String(s) => Value::Boolean(s.len() > 0),
            Value::Object(_) => Value::Boolean(true),
        }
    }
    fn to_number(&self) -> Value {
        match self {
            Value::Undefined => Value::Number(Number::NaN),
            Value::Null => Value::Number(Number::IaN(0.)),
            Value::Number(_) => self.clone(),
            Value::Boolean(b) => Value::Number(Number::IaN(if *b { 1. } else { 0. })),
            // TODO: impl the spec
            Value::String(s) => Value::Number(Number::IaN(s.parse::<f64>().unwrap_or_default())),
            Value::Object(_) => Value::Number(Number::NaN),
        }
    }
    fn to_string(&self) -> Value {
        Value::String(match self {
            Value::Undefined => String::from("undefined"),
            Value::Null => String::from("null"),
            Value::Boolean(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            // TODO: impl Object A/C spec
            Value::Object(_) => String::from("object"),
        })
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Null => String::from("Null"),
                Value::Undefined => String::from("Undefined"),
                Value::Number(n) => match n {
                    Number::NaN => String::from("NaN"),
                    Number::IaN(num) => num.to_string(),
                },
                Value::Boolean(b) => b.to_string(),
                Value::String(s) => s.clone(),
                Value::Object(_) => String::from("object"),
            }
        )
    }
}
impl std::ops::Add for Value {
    type Output = Self;
    // TODO: correct impl after adding other primitives
    fn add(self, other: Self) -> Self {
        match (&self, &other) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(match (n1, n2) {
                (Number::IaN(n1), Number::IaN(n2)) => Number::IaN(n1 + n2),
                _ => Number::NaN,
            }),
            _ => Value::Undefined,
        }
    }
}

impl Value {
    pub fn from_object(o: Object) -> Self {
        Value::Object(GcCell::new(Gc::new(o)))
    }
    pub fn from_f64(f: f64) -> Self {
        Value::Number(if f.is_nan() {
            Number::NaN
        } else {
            Number::IaN(f)
        })
    }
    pub fn from_gcobject(obj: GcCell<Gc<Object>>) -> Self {
        Value::Object(obj)
    }
    pub fn spawn(&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        match self {
            Value::Object(o) => o.borrow().spawn(vm, args), // o.spawn (vm, args),
            _ => Err("Cannot call New on primitive data"),
        }
    }
}
