use super::proto::ProtoObject;
use crate::vm::value::Value;

use super::*;

#[derive(Trace, Finalize, Debug)]
pub struct ArrayObject {
    proto: GcBox<ProtoObject>,
    vec: Vec<Value>,
}

use crate::vm::context::Context;
impl ArrayObject {
    pub fn new(ctx: &Context) -> Self {
        ArrayObject {
            proto: ctx.array_proto.clone(),
            vec: Vec::new(),
        }
    }
}

use std::str::FromStr;

impl Objectable for ArrayObject {
    fn get(&self, prop: &String) -> Value {
        let idx = usize::from_str(prop);
        match idx {
            Ok(idx) => {
                if idx < self.vec.len() {
                    self.vec[idx].clone()
                } else {
                    Value::Undefined
                }
            }
            Err(_) => Value::Undefined,
        }
    }
    fn put(&mut self, prop: &String, val: Value) {
        let idx = usize::from_str(prop);
        match idx {
            Ok(idx) => {
                if idx < self.vec.len() {
                    self.vec[idx] = val;
                }
            }
            Err(_) => (),
        };
    }
    fn toString(&self, vm: &mut VM) -> JSResult {
        let mut res = String::from("[");
        for v in &self.vec {
            match v.toString(vm) {
                Ok(o) => match &o {
                    Value::String(s) => res.push_str(s.as_str()),
                    _ => return Err(Value::from_str("toString () expected to return  String")),
                },
                Err(msg) => return Err(msg),
            };
            res.push_str(", ")
        }
        res.push_str("]");
        Ok(Value::String(res))
    }
}

// methods (rust)
impl ArrayObject {
    fn push(&mut self, val: Value) {
        self.vec.push(val);
    }
}
