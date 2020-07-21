use super::value::Value;
use super::vm::VM;
use crate::objects::array::ArrayObject;
use crate::objects::*;

use gc::{Gc, GcCell};

pub struct Context {
    pub mother_proto: GcBox<proto::ProtoObject>,
    pub array_proto: GcBox<Object>,
    pub prim_proto: GcBox<proto::ProtoObject>,
}

struct Mother;
impl Mother {
    fn toString(vm: &mut VM, args: &Vec<Value>) -> JSResult {
        Ok(Value::String(String::from("[object Object]")))
    }
    fn valueOf(vm: &mut VM, args: &Vec<Value>) -> JSResult {
        match args.len() {
            1 => Ok(args[0].clone()),
            _ => Err(Value::from_str("Expected 1 argument")),
        }
    }
}

impl Context {
    pub fn new() -> Self {
        let prim_proto = proto::ProtoObject {
            parent: None,
            dict: JSDict::new(),
        };
        Context {
            mother_proto: Self::get_mother(),
            array_proto: Self::build_array_proto(),
            prim_proto: Gc::new(GcCell::new(prim_proto)),
        }
    }

    fn get_mother() -> GcBox<proto::ProtoObject> {
        let mut mother_proto = proto::ProtoObject {
            parent: None,
            dict: JSDict::new(),
        };
        let to_string = Value::from_rjsfunc(Mother::toString, "toString");
        let value_of = Value::from_rjsfunc(Mother::valueOf, "valueOf");
        mother_proto
            .dict
            .insert(String::from("toString"), to_string);
        mother_proto.dict.insert(String::from("valueOf"), value_of);
        Gc::new(GcCell::new(mother_proto))
    }

    fn build_array_proto() -> GcBox<Object> {
        let mut array_proto = proto::ProtoObject {
            parent: None,
            dict: JSDict::new(),
        };
        let push_func = Value::from_rjsfunc(ArrayObject::js_push, "push");
        array_proto.dict.insert("push".to_string(), push_func);
        Gc::new(GcCell::new(Object::ProtoObject(array_proto)))
    }
}
