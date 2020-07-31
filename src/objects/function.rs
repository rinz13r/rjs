use super::*;
use crate::vm::code::Code;
use std::rc::Rc;

#[derive(Trace, Finalize)]
struct ConstructorMetaData {
    #[unsafe_ignore_trace]
    constructor: RJSFunc,
    prototype: GcObject,
}

impl std::fmt::Debug for ConstructorMetaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "constructor_metadata")
    }
}

#[derive(Trace, Finalize, Debug)]
pub struct Function {
    name: String,
    length: usize,
    payload: FunctionPayload,
}

#[derive(Trace, Finalize)]
enum FunctionPayload {
    UserDefined(UserFunctionData),
    Primitive(PrimitiveFunctionData),
}

#[derive(Trace, Finalize, Debug)]
struct UserFunctionData {
    #[unsafe_ignore_trace]
    code: Rc<Code>,
    prototype: GcObject,
}

#[derive(Trace, Finalize)]
struct PrimitiveFunctionData {
    #[unsafe_ignore_trace]
    func: RJSFunc,
    constructor_metadata: Option<ConstructorMetaData>,
}

impl std::fmt::Debug for FunctionPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function data")
    }
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
    pub fn new_userdefined(
        code: Rc<Code>,
        name: String,
        length: usize,
        prototype: GcObject,
    ) -> Self {
        Self {
            name,
            length,
            payload: FunctionPayload::UserDefined(UserFunctionData { code, prototype }),
        }
    }

    pub fn Call(&self, vm: &mut VM, args: &[Value]) -> JSResult {
        match &self.payload {
            FunctionPayload::Primitive(PrimitiveFunctionData { func, .. }) => func(vm, args),
            FunctionPayload::UserDefined(UserFunctionData { code, .. }) => {
                vm.call_code(code.clone(), self.length, args)
            }
        }
    }
    // TODO:
    pub fn Construct(&self, vm: &mut VM, args: &[Value]) -> JSResult {
        match &self.payload {
            FunctionPayload::Primitive(PrimitiveFunctionData {
                constructor_metadata,
                ..
            }) => {
                if let Some(constructor_metadata) = constructor_metadata {
                    let cons = constructor_metadata.constructor;
                    cons(vm, args)
                } else {
                    Err("Object is not constructible".into())
                }
            }
            FunctionPayload::UserDefined(UserFunctionData { prototype, .. }) => {
                let this = vm.ctx.new_Object(Some(prototype.clone()));
                vm.push_this(this.clone());
                this.unwrap_object().borrow_mut().__proto__ = prototype.clone().into();
                self.Call(vm, args)?;
                Ok(vm.pop_this())
            }
        }
    }
}

impl Function {
    pub fn new_primitive(
        name: String,
        func: RJSFunc,
        constructor: RJSFunc,
        length: usize,
        prototype: GcObject,
    ) -> Self {
        Self {
            name,
            payload: FunctionPayload::Primitive(PrimitiveFunctionData {
                func,
                constructor_metadata: Some(ConstructorMetaData {
                    constructor,
                    prototype,
                }),
            }),
            length,
        }
    }
    pub fn new_builtin(name: String, func: RJSFunc, length: usize) -> Self {
        Self {
            name,
            payload: FunctionPayload::Primitive(PrimitiveFunctionData {
                func,
                constructor_metadata: None,
            }),
            length,
        }
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
