use super::*;
use gc::{Finalize, Trace};

#[derive(Trace, Finalize, Debug)]
pub struct RegObject {
    dict: JSDict,
    proto: Option<GcBox<Object>>,
    // constructor: Option<GcBox<function::FunctionObject>>,
}

impl RegObject {
    pub fn new() -> Self {
        RegObject {
            dict: JSDict::new(),
            proto: None,
        }
    }
}

impl Objectable for RegObject {
    fn get(&self, prop: &String) -> Value {
        if prop.eq("prototype") {
            match &self.proto {
                None => Value::Null,
                Some(proto) => Value::Object(proto.clone()),
            }
        } else {
            match self.dict.get(prop) {
                Some(v) => v.clone(),
                None => match &self.proto {
                    None => Value::Undefined,
                    Some(proto) => proto.borrow().get(prop),
                },
            }
        }
    }
    fn put(&mut self, prop: &String, val: Value) {
        self.dict.insert(prop.to_string(), val);
    }
}
