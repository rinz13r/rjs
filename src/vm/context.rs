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
    pub String_prototype: GcObject,
    pub String_function: GcObject,
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
        let String_prototype = Self::build_String_prototype(Object_prototype.clone());
        let String_function =
            Self::build_String_function(Function_prototype.clone(), String_prototype.clone());
        let mut ctx = Context {
            Object_prototype,
            Object_function,
            Function_prototype,
            Function_function,
            Number_prototype,
            Number_function,
            String_prototype,
            String_function,
        };
        ctx.init_Object_prototype();
        ctx.init_Number_prototype();
        ctx.init_String_prototype();
        ctx
    }

    // Object.prototype
    fn build_Object_prototype() -> GcObject {
        // TODO: Add properties to Object.prototype
        Gc::new(GcCell::new(Object {
            __proto__: None,
            payload: ObjectPayload::Regular(object::Regular),
            props: JSDict::new(),
        }))
    }
    // Function.prototype
    fn build_Function_prototype(Object_prototype: GcObject) -> GcObject {
        let Function_prototype = Object {
            __proto__: Some(Object_prototype),
            props: JSDict::new(),
            payload: ObjectPayload::Regular(object::Regular),
        };
        Gc::new(GcCell::new(Function_prototype))
    }
    // function Function ()
    fn build_Function_function(Function_prototype: GcObject) -> GcObject {
        let Function_object = Object {
            __proto__: Some(Function_prototype.clone()),
            props: JSDict::new(),
            payload: ObjectPayload::Function(function::Function::new_primitive(
                "Function".into(),
                function::function,
                function::constructor,
                0,
                Function_prototype,
            )),
        };
        Gc::new(GcCell::new(Function_object))
    }
    // function Object ()
    fn build_Object_function(Function_prototype: GcObject, Object_prototype: GcObject) -> GcObject {
        let mut Object = Object {
            __proto__: Some(Function_prototype),
            payload: ObjectPayload::Function(function::Function::new_primitive(
                "Object".into(),
                object::function,
                object::constructor,
                0,
                Object_prototype.clone(),
            )),
            props: JSDict::new(),
        };
        Object.props.insert(
            "prototype".to_string(),
            Property::new(Object_prototype.into()),
        );
        Gc::new(GcCell::new(Object))
    }

    fn build_Number_prototype(Object_prototype: GcObject) -> GcObject {
        let Number_prototype = Object {
            __proto__: Some(Object_prototype),
            payload: ObjectPayload::Regular(object::Regular),
            props: JSDict::new(),
        };
        Gc::new(GcCell::new(Number_prototype))
    }

    fn build_Number_function(Function_prototype: GcObject, Number_prototype: GcObject) -> GcObject {
        Self::new_PrimitiveFunction(
            Function_prototype,
            Number_prototype,
            "Number",
            number::function,
            number::constructor,
            0,
        )
        .unwrap_object()
    }
    fn build_String_prototype(Object_prototype: GcObject) -> GcObject {
        let String_prototype = Object {
            __proto__: Some(Object_prototype),
            payload: ObjectPayload::Regular(object::Regular),
            props: JSDict::new(),
        };
        Gc::new(GcCell::new(String_prototype))
    }

    fn build_String_function(Function_prototype: GcObject, String_prototype: GcObject) -> GcObject {
        let String_function = Object {
            __proto__: Some(Function_prototype),
            payload: ObjectPayload::Function(function::Function::new_primitive(
                "Number".into(),
                string::function,
                string::constructor,
                0,
                String_prototype,
            )),
            props: JSDict::new(),
        };
        Gc::new(GcCell::new(String_function))
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
        let valueOf = self.new_BuiltinFunction("valueOf", number::valueOf, 0);
        let toString = self.new_BuiltinFunction("toString", number::toString, 0);
        insert_prop(&mut self.Number_prototype, "valueOf".to_string(), valueOf);
        insert_prop(&mut self.Number_prototype, "toString".to_string(), toString);
    }
    fn init_Object_prototype(&mut self) {
        fn insert_prop(prototype: &mut GcObject, key: String, value: Value) {
            prototype
                .borrow_mut()
                .props
                .insert(key, Property::new(value));
        }
        let valueOf = self.new_BuiltinFunction("valueOf", object::valueOf, 0);
        let toString = self.new_BuiltinFunction("toString", object::toString, 0);
        insert_prop(&mut self.Object_prototype, "valueOf".to_string(), valueOf);
        insert_prop(&mut self.Object_prototype, "toString".to_string(), toString);
    }
    fn init_String_prototype(&mut self) {
        fn insert_prop(prototype: &mut GcObject, key: String, value: Value) {
            prototype
                .borrow_mut()
                .props
                .insert(key, Property::new(value));
        }
        let valueOf = self.new_BuiltinFunction("valueOf", string::valueOf, 0);
        let toString = self.new_BuiltinFunction("toString", string::toString, 0);
        insert_prop(&mut self.String_prototype, "valueOf".to_string(), valueOf);
        insert_prop(&mut self.String_prototype, "toString".to_string(), toString);
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
            payload: ObjectPayload::Regular(object::Regular),
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
            payload: ObjectPayload::Function(function::Function::new_userdefined(
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
        Function_prototype: GcObject,
        prototype: GcObject,
        name: &'static str,
        func: RJSFunc,
        constructor: RJSFunc,
        length: usize,
    ) -> Value {
        // let prototype = self
        //     .new_Object(self.Object_prototype.clone().into())
        //     .unwrap_object();
        let mut object = Object {
            __proto__: Function_prototype.clone().into(),
            payload: ObjectPayload::Function(function::Function::new_primitive(
                name.into(),
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

    pub fn new_BuiltinFunction(&self, name: &'static str, func: RJSFunc, length: usize) -> Value {
        let object = Object {
            __proto__: self.Function_prototype.clone().into(),
            payload: ObjectPayload::Function(function::Function::new_builtin(
                name.into(),
                func,
                length,
            )),
            props: JSDict::new(),
        };
        object.into()
    }

    pub fn new_String(&self, value: String) -> Value {
        let object = Object {
            __proto__: self.String_prototype.clone().into(),
            payload: ObjectPayload::String(string::String::new(value)),
            props: JSDict::new(),
        };
        object.into()
    }
}
