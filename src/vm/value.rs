use crate::objects::*;
use crate::vm::code::Code;
use crate::vm::context::Context;
use crate::vm::vm::VM;
use gc::{Finalize, Gc, GcCell, Trace};
use std::rc::Rc;

#[derive(Trace, Finalize, Clone, Debug)]
pub enum Value {
    Null,
    Undefined,
    Number(Number),
    Boolean(bool),
    String(String),
    Object(GcBox<Object>),
}

#[derive(Clone, Trace, Finalize, Debug)]
pub enum Number {
    NaN,
    IaN(f64),
}

impl std::cmp::PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Number::IaN(x), Number::IaN(y)) => x == y,
            _ => false,
        }
    }
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
    fn ToBoolean(&self) -> Value {
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

impl std::ops::Sub for Value {
    type Output = Self;
    // TODO: correct impl after adding other primitives
    fn sub(self, other: Self) -> Self {
        match (&self, &other) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(match (n1, n2) {
                (Number::IaN(n1), Number::IaN(n2)) => Number::IaN(n1 - n2),
                _ => Number::NaN,
            }),
            _ => Value::Undefined,
        }
    }
}

impl std::cmp::PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => n1 == n2,
            (Value::Boolean(b1), Value::Boolean(b2)) => b1 == b2,
            (Value::Object(_), Value::Object(_)) => false,
            (Value::Undefined, Value::Null) => true,
            (Value::Null, Value::Undefined) => true,
            _ => false,
        }
    }
}

impl Value {
    pub fn from_f64(f: f64) -> Self {
        Value::Number(if f.is_nan() {
            Number::NaN
        } else {
            Number::IaN(f)
        })
    }
    pub fn from_rjsfunc(func: RJSFunc, name: &'static str) -> Self {
        Value::Object(Gc::new(GcCell::new(Object::from_rjsfunc(func, name))))
    }
    pub fn new_functionobject(ctx: &Context, code: Rc<Code>, len: usize) -> Self {
        Value::Object(Gc::new(GcCell::new(Object::new_functionobject(
            ctx, code, len,
        ))))
    }
    pub fn new_regobject() -> Self {
        Value::Object(Gc::new(GcCell::new(Object::new_regobject())))
    }
    pub fn from_str(s: &str) -> Self {
        Value::String(String::from(s))
    }
    pub fn new_arrayobject(ctx: &Context, els: Vec<Value>) -> Self {
        Value::Object(Gc::new(GcCell::new(Object::new_arrayobject(ctx, els))))
    }
}

impl Value {
    pub fn to_bool(&self) -> bool {
        match self.ToBoolean() {
            Value::Boolean(b) => b,
            _ => panic!("ToBoolean didn't return JSBool"),
        }
    }
}

impl Objectable for Value {
    fn get(&self, prop: &String) -> Value {
        match self {
            Value::Object(o) => o.borrow().get(prop),
            _ => Value::Undefined,
        }
    }
    fn put(&mut self, prop: &String, val: Value) {
        match self {
            Value::Object(o) => o.borrow_mut().put(prop, val),
            _ => {
                panic!("'put' expected object. Received: {}", val);
            }
        }
    }
    fn call(&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        match self {
            Value::Object(o) => o.borrow().call(vm, args),
            _ => Err(Value::from_str("object not callable")),
        }
    }
    fn spawn(&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        match self {
            Value::Object(o) => o.borrow().spawn(vm, args),
            _ => Err(Value::from_str("object not constructible")),
        }
    }
    fn toString(&self, vm: &mut VM) -> JSResult {
        match &self {
            Value::Object(o) => o.borrow().toString(vm),
            _ => Ok(self.to_string()),
        }
    }
    fn setPrototype(&mut self, prototype: GcBox<Object>) {
        match self {
            Value::Object(o) => o.borrow_mut().setPrototype(prototype),
            _ => panic!("Expected Object"),
        }
    }
}
