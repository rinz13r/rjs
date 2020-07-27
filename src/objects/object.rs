use super::*;

// JS Primitives
pub fn constructor(_vm: &mut VM, _this: Value, _args: &[Value]) -> JSResult {
    Err("Not impl".into())
}
pub fn function(vm: &mut VM, _this: Value, args: &[Value]) -> JSResult {
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
