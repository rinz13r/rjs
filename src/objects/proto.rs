use gc::{Gc, GcCell, Finalize, Trace};
use crate::vm::value::Value;
use super::*;

#[derive(Trace, Finalize, Debug)]
pub struct ProtoObject {
    pub parent: Option<Gc<GcCell<Object>>>,
    pub dict: JSDict,
}

struct Operations {
    to_string: fn (GcBox<ProtoObject>) -> Value,
}

impl Objectable for ProtoObject {
    fn get (&self, prop: &String) -> Value {
        Value::Undefined
    }
    fn put (&mut self, prop: &String, val: Value) {}
    fn call (&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        Err ("Object not callable")
    }

}
