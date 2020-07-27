#![allow(non_snake_case)]

use crate::objects::*;

use gc::{Gc, GcCell};

pub struct Context {
    pub Object_prototype: GcObject,
    pub Object_function: GcObject,
    pub Function_prototype: GcObject,
    pub Function_function: GcObject,
    pub Number_prototype: GcObject,
    pub Number_function: GcObject,
}

impl Context {
    pub fn new() -> Self {
        let Object_prototype = Self::build_Object_prototype();
        let Function_prototype = Self::build_Function_prototype(Object_prototype.clone());
        let Function_function = Self::build_Function_function(Function_prototype.clone());
        let Object_function =
            Self::build_Object_function(Function_prototype.clone(), Object_prototype.clone());
        let Number_prototype = Self::build_Number_prototype(Object_prototype.clone());
        let Number_function =
            Self::build_Number_function(Function_prototype.clone(), Number_prototype.clone());
        let mut ctx = Context {
            Object_prototype,
            Object_function,
            Function_prototype,
            Function_function,
            Number_prototype,
            Number_function,
        };
        ctx.init_Number_prototype();
        ctx
    }

    // Object.prototype
    fn build_Object_prototype() -> GcObject {
        // let Object_prototype = proto::ProtoObject {
        //     parent: None,
        //     dict: JSDict::new(),
        // };
        // TODO: Add properties to Object.prototype
        Gc::new(GcCell::new(Object {
            __proto__: None,
            payload: ObjectPayload::Regular,
            props: JSDict::new(),
        }))
    }
    // Function.prototype
    fn build_Function_prototype(Object_prototype: GcObject) -> GcObject {
        let Function_prototype = Object {
            __proto__: Some(Object_prototype),
            props: JSDict::new(),
            payload: ObjectPayload::Regular,
        };
        Gc::new(GcCell::new(Function_prototype))
    }
    // function Function ()
    fn build_Function_function(Function_prototype: GcObject) -> GcObject {
        let Function_object = Object {
            __proto__: Some(Function_prototype.clone()),
            props: JSDict::new(),
            payload: ObjectPayload::PrimitiveFunction(function::PrimitiveFunction {
                name: "Function",
                prototype: Function_prototype,
                func: function::Function_constructor,
                constructor: function::Function_constructor,
                length: 0,
            }),
        };
        Gc::new(GcCell::new(Function_object))
    }
    // function Object ()
    fn build_Object_function(Function_prototype: GcObject, Object_prototype: GcObject) -> GcObject {
        let Object = Object {
            __proto__: Some(Function_prototype),
            payload: ObjectPayload::PrimitiveFunction(function::PrimitiveFunction {
                name: "Object",
                func: object::function,
                constructor: object::constructor,
                prototype: Object_prototype,
                length: 0,
            }),
            props: JSDict::new(),
        };
        Gc::new(GcCell::new(Object))
    }

    fn build_Number_prototype(Object_prototype: GcObject) -> GcObject {
        let Number_prototype = Object {
            __proto__: Some(Object_prototype),
            payload: ObjectPayload::Regular,
            props: JSDict::new(),
        };
        Gc::new(GcCell::new(Number_prototype))
    }

    fn build_Number_function(Function_prototype: GcObject, Number_prototype: GcObject) -> GcObject {
        let Number_function = Object {
            __proto__: Some(Function_prototype),
            payload: ObjectPayload::PrimitiveFunction(function::PrimitiveFunction {
                name: "Number",
                func: number::function,
                constructor: number::constructor,
                length: 0,
                prototype: Number_prototype,
            }),
            props: JSDict::new(),
        };
        Gc::new(GcCell::new(Number_function))
    }
}

impl Context {
    fn init_Number_prototype(&mut self) {
        fn insert_prop(prototype: &mut GcObject, key: String, value: Value) {
            prototype
                .borrow_mut()
                .props
                .insert(key, Property::new(value));
        }
        let valueOf = self.new_PrimitiveFunction("valueOf", number::valueOf, number::valueOf, 0);
        let toString = self.new_PrimitiveFunction("toString", number::toString, number::toString, 0);
        insert_prop(&mut self.Number_prototype, "valueOf".to_string(), valueOf);
        insert_prop(&mut self.Number_prototype, "toString".to_string(), toString);
    }
}
use super::code::Code;
use super::value::Value;
use std::rc::Rc;

impl Context {
    pub fn new_Number(&self, value: f64) -> Value {
        let n = Object {
            __proto__: Some(self.Number_prototype.clone()),
            payload: ObjectPayload::Number(number::Number::new(value)),
            props: JSDict::new(),
        };
        Value::Object(Gc::new(GcCell::new(n)))
    }
    pub fn new_Object(&self, __proto__: Option<GcObject>) -> Value {
        let object = Object {
            __proto__,
            payload: ObjectPayload::Regular,
            props: JSDict::new(),
        };
        Value::Object(Gc::new(GcCell::new(object)))
    }
    pub fn new_Function(&self, name: String, code: Rc<Code>, length: usize) -> Value {
        let prototype = self
            .new_Object(self.Object_prototype.clone().into())
            .unwrap_object();
        let mut object = Object {
            __proto__: self.Function_prototype.clone().into(),
            payload: ObjectPayload::Function(function::Function::new(
                code,
                name,
                length,
                prototype.clone(),
            )),
            props: JSDict::new(),
        };
        fn init_Function_object(object: &mut Object, prototype: GcObject) {
            object
                .props
                .insert("prototype".to_string(), Property::new(prototype.into()));
        }
        init_Function_object(&mut object, prototype);
        Value::Object(Gc::new(GcCell::new(object)))
    }
    pub fn new_PrimitiveFunction(
        &self,
        name: &'static str,
        func: RJSFunc,
        constructor: RJSFunc,
        length: usize,
    ) -> Value {
        let prototype = self
            .new_Object(self.Object_prototype.clone().into())
            .unwrap_object();
        let mut object = Object {
            __proto__: self.Function_prototype.clone().into(),
            payload: ObjectPayload::PrimitiveFunction(function::PrimitiveFunction::new(
                name,
                func,
                constructor,
                length,
                prototype.clone(),
            )),
            props: JSDict::new(),
        };
        object
            .props
            .insert("prototype".to_string(), Property::new(prototype.into()));
        object.into()
    }
}
