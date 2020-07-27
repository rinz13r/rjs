#![allow(non_snake_case)]

use crate::objects::*;
use crate::vm::context::Context;
use crate::vm::vm::VM;
use gc::{Finalize, Gc, GcCell, Trace};

#[derive(Trace, Finalize, Clone, Debug)]
pub enum Value {
    Null,
    Undefined,
    Number(f64),
    Boolean(bool),
    String(String),
    Object(GcObject),
}

impl Value {
    fn ToBoolean(&self) -> Value {
        match self {
            Value::Undefined | Value::Null => Value::Boolean(false),
            Value::Boolean(_) => self.clone(),
            Value::Number(n) => match n {
                n if n.is_nan() => Value::Boolean(false),
                n if n == &0. => Value::Boolean(false),
                _ => Value::Boolean(true),
            },
            Value::String(s) => Value::Boolean(s.len() > 0),
            Value::Object(_) => Value::Boolean(true),
        }
    }
    pub fn ToNumber(&self, vm: &mut VM) -> JSResult {
        Ok(match self {
            Value::Undefined => Value::Number(f64::NAN),
            Value::Null => Value::Number(0.),
            Value::Number(_) => self.clone(),
            Value::Boolean(b) => Value::Number(if *b { 1. } else { 0. }),
            // TODO: impl the spec
            Value::String(s) => Value::Number(s.parse::<f64>().unwrap_or_default()),
            Value::Object(o) => o.borrow().DefaultValue(PreferredType::Number.into(), vm)?,
        })
    }

    pub fn ToObject(&self, ctx: &Context) -> Value {
        self.as_object(ctx).into()
    }

    pub fn ToPrimitive(&self, vm: &mut VM) -> JSResult {
        match self {
            Value::Object(o) => o.borrow().DefaultValue(None, vm),
            _ => Ok(self.clone()),
        }
    }
    pub fn ToString(&self, vm: &mut VM) -> JSResult {
        match self {
            Value::Undefined => Ok("undefined".into()),
            Value::Null => Ok("null".into()),
            Value::Boolean(b) => Ok(b.to_string().into()),
            Value::Number(n) => Ok(n.to_string().into()),
            Value::String(s) => Ok(s.clone().into()),
            Value::Object(o) => {
                vm.push_this(self.clone());
                let ret = o.borrow().DefaultValue(PreferredType::String.into(), vm);
                vm.pop_this();
                ret
            }
        }
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
                    n if n.is_nan() => String::from("NaN"),
                    _ => n.to_string(),
                },
                Value::Boolean(b) => b.to_string(),
                Value::String(s) => s.clone(),
                // TODO: impl Object A/C spec
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
            (Value::Number(n1), Value::Number(n2)) => (n1 + n2).into(),
            _ => Value::Undefined,
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Self;
    // TODO: correct impl after adding other primitives
    fn sub(self, other: Self) -> Self {
        match (&self, &other) {
            (Value::Number(n1), Value::Number(n2)) => match (n1.is_nan(), n2.is_nan()) {
                (false, false) => Value::Number(n1 - n2),
                _ => Value::Number(f64::NAN),
            },
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
    pub fn as_object(&self, ctx: &Context) -> GcObject {
        match self {
            Value::Object(o) => o.clone(),
            Value::Number(n) => ctx.new_Number(*n).unwrap_object(),
            Value::String(s) => ctx.new_String(s.clone()).unwrap_object(),
            _ => panic!("Not an object"),
        }
    }
    pub fn unwrap_object(&self) -> GcObject {
        match self {
            Value::Object(o) => o.clone(),
            _ => panic!("Fatal Error: Not an object!"),
        }
    }
    pub fn unwrap_string(&self) -> &String {
        match self {
            Value::String(o) => o,
            _ => panic!("Fatal Error: Not a String!"),
        }
    }
}

impl std::default::Default for Value {
    fn default() -> Self {
        Self::Undefined
    }
}

impl From<f64> for Value {
    fn from(val: f64) -> Self {
        Value::Number(val)
    }
}

impl From<String> for Value {
    fn from(val: String) -> Self {
        Self::String(val)
    }
}

impl From<&str> for Value {
    fn from(val: &str) -> Self {
        Self::String(val.into())
    }
}

impl From<bool> for Value {
    fn from(val: bool) -> Self {
        Self::Boolean(val)
    }
}

impl From<u64> for Value {
    fn from(val: u64) -> Self {
        Value::Number((val as f64).into())
    }
}

impl From<GcObject> for Value {
    fn from(val: GcObject) -> Self {
        Value::Object(val)
    }
}

impl From<&GcObject> for Value {
    fn from(val: &GcObject) -> Self {
        Value::Object(val.clone())
    }
}

impl From<Object> for Value {
    fn from(val: Object) -> Self {
        Value::Object(Gc::new(GcCell::new(val)))
    }
}
impl Into<bool> for Value {
    fn into(self) -> bool {
        match self.ToBoolean() {
            Value::Boolean(b) => b,
            _ => panic!("ToBoolean didn't return JSBool"),
        }
    }
}

impl Value {
    fn is_String(&self) -> bool {
        if let Value::String(_) = self {
            true
        } else {
            false
        }
    }
}

// Operators
impl Value {
    pub fn bin_add(&self, rhs: Self, vm: &mut VM) -> JSResult {
        let v1 = self.ToPrimitive(vm)?;
        let v2 = rhs.ToPrimitive(vm)?;
        if !(v1.is_String() || v2.is_String()) {
            let v1 = v1.ToNumber(vm)?;
            let v2 = v2.ToNumber(vm)?;
            Ok(v1 + v2)
        } else {
            let v1 = v1.ToString(vm)?;
            let v2 = v2.ToString(vm)?;
            Ok(v1 + v2)
        }
    }
    pub fn bin_sub(&self, rhs: Self, vm: &mut VM) -> JSResult {
        let v1 = self.ToNumber(vm)?;
        let v2 = rhs.ToNumber(vm)?;
        Ok(v1 - v2)
    }
}
