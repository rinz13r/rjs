use super::*;
use crate::vm::code::Code;
use std::rc::Rc;

#[derive(Trace, Finalize, Debug)]
pub struct Function {
    name: String,
    #[unsafe_ignore_trace]
    code: Rc<Code>, // refcnt is enough
    length: usize,
    prototype: GcObject,
}

#[derive(Trace, Finalize)]
pub struct PrimitiveFunction {
    pub name: &'static str,
    #[unsafe_ignore_trace]
    pub func: RJSFunc,
    #[unsafe_ignore_trace]
    pub constructor: Option<RJSFunc>,
    pub length: usize,
    pub prototype: GcObject,
}

impl Function {
    pub fn new(code: Rc<Code>, name: String, length: usize, prototype: GcObject) -> Self {
        Self {
            code,
            name,
            length,
            prototype,
        }
    }

    pub fn Call(&self, vm: &mut VM, args: &[Value]) -> JSResult {
        vm.call_code(self.code.clone(), self.length, args)
    }
    // TODO:
    pub fn Construct(&self, vm: &mut VM, args: &[Value]) -> JSResult {
        let this = vm.ctx.new_Object(Some(self.prototype.clone()));
        vm.push_this(this.clone());
        this.unwrap_object().borrow_mut().__proto__ = self.prototype.clone().into();
        match self.Call(vm, args) {
            Ok(_) => (),
            Err(msg) => return Err(msg),
        }
        Ok(vm.pop_this())
    }
}

impl PrimitiveFunction {
    pub fn new(
        name: &'static str,
        func: RJSFunc,
        constructor: Option<RJSFunc>,
        length: usize,
        prototype: GcObject,
    ) -> Self {
        Self {
            name,
            func,
            constructor,
            length,
            prototype,
        }
    }
    pub fn Call(&self, vm: &mut VM, args: &[Value]) -> JSResult {
        let func = self.func;
        func(vm, args)
    }
    // PrimitiveFunction's constructors will build the object themselves
    pub fn Construct(&self, vm: &mut VM, args: &[Value]) -> JSResult {
        if let Some(cons) = self.constructor {
            cons(vm, args)
        } else {
            Err(format!("{} is not a Constructor", self.name).into())
        }
    }
}

impl std::fmt::Debug for PrimitiveFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "builtin function: {}", self.name)
    }
}

// JS Primitives
// function Function ()
pub fn function(_vm: &mut VM, _args: &[Value]) -> JSResult {
    Err("Not impl".into())
}

pub fn constructor(_vm: &mut VM, _args: &[Value]) -> JSResult {
    Err("Not impl".into())
}
