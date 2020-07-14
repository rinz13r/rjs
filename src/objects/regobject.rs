use super::*;
use gc::{Finalize, Trace};

#[derive(Trace, Finalize)]
pub struct RegObject {
    dict: JSDict,
    proto: Option<GcBox<proto::ProtoObject>>,
}

impl Objectable for RegObject {
    fn get(&self, prop: &String) -> Value {
        match self.dict.get(prop) {
            Some(v) => v.clone(),
            None => match &self.proto {
                None => Value::Undefined,
                Some(proto) => proto.borrow().get(prop),
            },
        }
    }
    fn put(&mut self, prop: &String, val: Value) {
        self.dict.insert(prop.to_string(), val);
    }
}
