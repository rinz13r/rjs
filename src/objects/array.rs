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
    pub fn new (ctx: &Context) -> Self {
        ArrayObject {
            proto: ctx.array_proto.clone (),
            vec: Vec::new ()
        }
    }
}

use std::str::FromStr;

impl Objectable for ArrayObject {
    fn get (&self, prop: &String) -> Value {
        let idx = usize::from_str (prop);
        match idx {
            Ok(idx) => {
                if idx < self.vec.len () {
                    self.vec[idx].clone ()
                } else {
                    Value::Undefined
                }
            },
            Err (_) => Value::Undefined
        }
    }
    fn put (&mut self, prop: &String, val: Value) {
        let idx = usize::from_str (prop);
        match idx {
            Ok(idx) => {
                if idx < self.vec.len () {
                    self.vec[idx] = val;
                }
            },
            Err (_) => ()
        };
    }
}
