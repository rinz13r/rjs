use crate::vm::code::Code;
use std::rc::Rc;
use super::*;
use crate::vm::context::Context;

#[derive(Trace, Finalize, Debug)]
pub struct FunctionObject {
    proto: GcBox<proto::ProtoObject>,
    name: String,
    #[unsafe_ignore_trace]
    code: Rc<Code>, // refcnt is enough
    length: usize
}

impl FunctionObject {
    pub fn new (ctx: &Context, code: Rc<Code>, length: usize, name: &String) -> Self {
        // TODO: Add Function proto
        Self {code, length, name: name.clone (), proto: ctx.array_proto.clone ()}
    }
}

impl Objectable for FunctionObject {
    fn call (&self, vm: &mut VM, args: &Vec<Value>) -> JSResult {
        vm.call_code (self.code.clone (), self.length, args)
    }
    fn get (&self, prop: &String) -> Value {Value::Undefined}
}
