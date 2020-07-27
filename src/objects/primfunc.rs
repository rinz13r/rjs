use super::*;
use crate::vm::value::Value;

#[derive(Trace, Finalize)]
pub struct PrimFunction {
    #[unsafe_ignore_trace]
    pub func: RJSFunc,
    pub name: &'static str,
    pub prototype: GcObject,
}

pub fn new_gcobject(
    ctx: &Context,
    name: &'static str,
    func: RJSFunc,
    prototype: GcObject,
) -> GcObject {
    let mut obj = Object {
        __proto__: Some(ctx.Function_prototype.clone()),
        payload: ObjectPayload::PrimFunction(PrimFunction {
            func,
            name,
            prototype: prototype.clone(),
        }),
        props: JSDict::new(),
    };
    obj.props
        .insert("prototype".to_string(), Property::new(prototype.into()));
    Gc::new(GcCell::new(obj))
}

impl std::fmt::Debug for PrimFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "builtin function: {}", self.name)
    }
}

impl PrimFunction {
    // pub fn new(func: RJSFunc, name: &'static str) -> Self {
    //     Self {
    //         func,
    //         name,
    //     }
    // }
}

impl PrimFunction {
    pub fn Call(&self, vm: &mut VM, args: &[Value]) -> JSResult {
        let func = self.func;
        func(vm, args)
    }
    fn toString(&self, _vm: &mut VM) -> JSResult {
        Ok(Value::String(format!("[builtin {}]", self.name)))
    }
}
