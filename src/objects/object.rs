use super::*;

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
