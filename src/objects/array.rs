use super::proto::ProtoObject;
use crate::vm::value::Value;

use super::*;

#[derive(Trace, Finalize, Debug)]
pub struct ArrayObject {
    proto: GcBox<Object>,
    vec: Vec<Value>,
}

use crate::vm::context::Context;
impl ArrayObject {
    pub fn new(ctx: &Context, els: Vec<Value>) -> Self {
        ArrayObject {
            proto: ctx.array_proto.clone(),
            vec: els,
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
            Err(_) => self.proto.borrow().get(prop),
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
        macro_rules! print_val {
            ($x:ident) => {
                match $x.toString(vm) {
                    Ok(o) => match &o {
                        Value::String(s) => res.push_str(s.as_str()),
                        _ => return Err(Value::from_str("toString () expected to return  String")),
                    },
                    Err(msg) => return Err(msg),
                };
            };
        }
        for v in &self.vec[..self.vec.len() - 1] {
            print_val!(v);
            res.push_str(", ")
        }
        for v in &self.vec[self.vec.len() - 1..] {
            print_val!(v);
        }
        res.push_str("]");
        Ok(Value::String(res))
    }
    fn setPrototype(&mut self, prototype: GcBox<Object>) {}
}

// methods (rust)
impl ArrayObject {
    fn push(&mut self, val: Value) -> JSResult {
        self.vec.push(val);
        Ok(Value::from_f64(self.vec.len() as f64))
    }
}

// JS methods
impl ArrayObject {
    pub fn js_push(vm: &mut VM, args: &Vec<Value>) -> JSResult {
        let el = if args.len() > 0 {
            args[0].clone()
        } else {
            return Ok(Value::Undefined);
        };
        let this = vm.get_this();
        match &this {
            Value::Object(o) => match &mut *o.borrow_mut() {
                Object::ArrayObject(o) => return o.push(el),
                _ => panic!("Push for array only"),
            },
            _ => panic!("push meant for arrays"),
        };
        Ok(Value::Undefined)
    }
}
