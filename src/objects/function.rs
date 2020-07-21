use super::*;
use crate::vm::code::Code;
use crate::vm::context::Context;
use std::rc::Rc;

#[derive(Trace, Finalize, Debug)]
pub struct FunctionObject {
    proto: GcBox<proto::ProtoObject>,
    name: String,
    #[unsafe_ignore_trace]
    code: Rc<Code>, // refcnt is enough
    length: usize,
    dict: JSDict,
    prototype: GcBox<Object>,
}

impl FunctionObject {
    pub fn new(ctx: &Context, code: Rc<Code>, length: usize, name: &String) -> Self {
        // TODO: Add Function proto
        Self::init(Self {
            code,
            length,
            name: name.clone(),
            proto: ctx.prim_proto.clone(),
            dict: JSDict::new(),
            prototype: Gc::new(GcCell::new(Object::new_regobject())),
        })
    }
    fn init(mut obj: Self) -> Self {
        obj.dict.insert(
            "prototype".to_string(),
            Value::Object(obj.prototype.clone()),
        );
        obj
    }
}

impl Objectable for FunctionObject {
    fn call(&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        vm.call_code(self.code.clone(), self.length, args)
    }
    fn get(&self, prop: &String) -> Value {
        match self.dict.get(prop) {
            Some(v) => v.clone(),
            None => Value::Undefined,
        }
    }
    fn put(&mut self, prop: &String, val: Value) {
        self.dict.insert(prop.clone(), val);
    }
    fn spawn(&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        vm.push_this();
        match self.call(vm, args) {
            Ok(_) => (),
            Err(msg) => return Err(msg),
        }
        let mut obj = vm.pop_this();
        obj.setPrototype(self.prototype.clone());
        Ok(obj)
    }
    fn setPrototype(&mut self, prototype: GcBox<Object>) {}
}
