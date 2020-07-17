use super::*;
use crate::vm::value::Value;
use gc::{Finalize, Gc, GcCell, Trace};

#[derive(Trace, Finalize, Debug)]
pub struct ProtoObject {
    pub parent: Option<Gc<GcCell<Object>>>,
    pub dict: JSDict,
}

struct Operations {
    to_string: fn(GcBox<ProtoObject>) -> Value,
}

impl Objectable for ProtoObject {
    fn get(&self, prop: &String) -> Value {
        match self.dict.get(prop) {
            Some(v) => v.clone(),
            None => match &self.parent {
                None => Value::Undefined,
                Some(par) => par.borrow().get(prop),
            },
        }
    }
    fn put(&mut self, prop: &String, val: Value) {
        self.dict.insert(prop.clone(), val);
    }
}
