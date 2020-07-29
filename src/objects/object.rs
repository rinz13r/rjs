use super::*;

#[derive(Trace, Finalize, Debug)]
pub struct Regular;

impl Regular {
    pub fn valueOf(&self, obj: &Object, vm: &mut VM) -> JSResult {
        if let Some(ref proto) = obj.__proto__ {
            if let Value::Object(ref valueOf) = proto.borrow().Get(&"valueOf".to_string()) {
                return valueOf.borrow().Call(vm, &[]);
            }
        }
        panic!("Fatal Error: valueOf not found");
    }
    pub fn toString(&self, gcobj: &GcObject, vm: &mut VM) -> JSResult {
        if let Some(ref proto) = gcobj.borrow().__proto__ {
            if let Value::Object(ref toString) = proto.borrow().Get(&"toString".to_string()) {
                vm.push_this(gcobj.into());
                let res = toString.borrow().Call(vm, &[]);
                let _ = vm.pop_this();
                return res;
            }
        }
        panic!("Fatal Error: toString not found");
    }
}

// JS Primitives
pub fn constructor(_vm: &mut VM, _args: &[Value]) -> JSResult {
    Err("Not impl".into())
}
pub fn function(vm: &mut VM, args: &[Value]) -> JSResult {
    match args.len() {
        0 => Ok(vm
            .ctx
            .new_Object(vm.ctx.Object_prototype.clone().into())
            .into()),
        _ => match args[0] {
            Value::Null | Value::Undefined => Ok(vm
                .ctx
                .new_Object(vm.ctx.Object_prototype.clone().into())
                .into()),
            _ => Ok(args[0].ToObject(vm.ctx)),
        },
    }
}

pub fn toString(vm: &mut VM, _args: &[Value]) -> JSResult {
    if let Value::Object(_) = vm.get_this() {
        return Ok("[object Object]".into());
    }
    panic!("Fatal Error: Expected Object");
}

pub fn valueOf(vm: &mut VM, _args: &[Value]) -> JSResult {
    Ok(vm.get_this().clone())
}
