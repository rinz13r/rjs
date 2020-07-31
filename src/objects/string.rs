use super::*;

type RStr = std::string::String;

#[derive(Trace, Finalize, Debug)]
pub struct String {
    value: RStr,
}

impl String {
    pub fn new(value: RStr) -> Self {
        Self { value }
    }
    pub fn valueOf(&self) -> Value {
        self.value.clone().into()
    }
    pub fn toString(&self) -> Value {
        self.value.clone().into()
    }
}

pub fn function(vm: &mut VM, args: &[Value]) -> JSResult {
    if args.len() == 0 {
        Ok("".into())
    } else {
        args[0].ToString(vm)
    }
}

pub fn constructor(vm: &mut VM, args: &[Value]) -> JSResult {
    if args.len() == 0 {
        Ok(vm.ctx.new_String("".to_string()))
    } else {
        let to_string = args[0].ToString(vm);
        if to_string.is_ok() {
            Ok(to_string.unwrap().unwrap_string().clone().into())
        } else {
            to_string
        }
    }
}

use crate::js_impl;
use crate::vm::context::Context;

js_impl! {
    #[prop(name=toString, length=1)]
    fn toString(vm: &mut VM, _args: &[Value]) -> JSResult {
        let obj = vm.get_this().as_object(vm.ctx);
        let ref payload = obj.borrow().payload;
        match payload {
            ObjectPayload::String(s) => Ok(s.valueOf()),
            _ => Err("RuntimeError: `this` is not a String object".into()),
        }
    },

    #[prop(name=valueOf, length=1)]
    fn valueOf(vm: &mut VM, _args: &[Value]) -> JSResult {
        let obj = vm.get_this().as_object(vm.ctx);
        let ref payload = obj.borrow().payload;
        match payload {
            ObjectPayload::String(s) => Ok(s.valueOf()),
            _ => Err("RuntimeError: `this` is not a String object".into()),
        }
    },

    #[prop(name=toLowerCase, length=1)]
    fn toLowerCase(vm: &mut VM, _args: &[Value]) -> JSResult {
        let obj = vm.get_this().as_object(vm.ctx);
        let ref payload = obj.borrow().payload;
        match payload {
            ObjectPayload::String(String{ref value}) => Ok(value.to_lowercase ().into()),
            _ => Err("RuntimeError: `this` is not a String object".into()),
        }
    },

    #[prop(name=toUpperCase, length=1)]
    fn toUpperCase(vm: &mut VM, _args: &[Value]) -> JSResult {
        let obj = vm.get_this().as_object(vm.ctx);
        let ref payload = obj.borrow().payload;
        match payload {
            ObjectPayload::String(String{ref value}) => Ok(value.to_uppercase ().into()),
            _ => Err("RuntimeError: `this` is not a String object".into()),
        }
    }
}
