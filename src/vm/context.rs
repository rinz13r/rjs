use crate::objects::*;
use gc::{Gc, GcCell};
use super::value::Value;

pub struct Context {
    pub mother_proto: GcBox<proto::ProtoObject>,
    pub array_proto: GcBox<proto::ProtoObject>,
    pub prim_proto: GcBox<proto::ProtoObject>,
}

struct Mother;
impl Mother {
    fn toString (args: &Vec<Value>) -> JSResult {
        Ok (Value::String (String::from ("[object Object]")))
    }
}

impl Context {
    pub fn new () -> Self {
        let array_proto = proto::ProtoObject {
            parent: None,
            dict: JSDict::new ()
        };
        let prim_proto = proto::ProtoObject {
            parent: None,
            dict: JSDict::new ()
        };
        Context {
            mother_proto: Self::get_mother (),
            array_proto: Gc::new (GcCell::new (array_proto)),
            prim_proto: Gc::new (GcCell::new (prim_proto)),
        }
    }


    fn get_mother () -> GcBox<proto::ProtoObject> {
        let mut mother_proto = proto::ProtoObject {
            parent: None,
            dict: JSDict::new ()
        };
        let to_string = Value::from_rjsfunc (Mother::toString, "toString");
        mother_proto.dict.insert (String::from ("toString"), to_string);
        Gc::new (GcCell::new (mother_proto))
    }
}
