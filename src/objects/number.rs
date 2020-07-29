use super::*;

#[derive(Trace, Finalize, Debug, Clone)]
pub struct Number {
    value: f64,
}
impl Number {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
    pub fn valueOf(&self) -> Value {
        self.value.into()
    }
    pub fn toString(&self) -> Value {
        self.value.to_string().into()
    }
}

macro_rules! extract_number {
    ($x:ident) => {
        if let Value::Object(ref $x) = $x {
            if let ObjectPayload::Number(ref n) = $x.borrow().payload {
                n.clone()
            } else {
                return Err("Expect number".into());
            }
        } else {
            panic!("Expected object")
        }
    };
}

// JS Primitives
pub fn function(vm: &mut VM, args: &[Value]) -> JSResult {
    Ok(match args.len() {
        0 => Value::Number(0.),
        _ => args[0].ToNumber(vm)?,
    })
}

pub fn constructor(vm: &mut VM, args: &[Value]) -> JSResult {
    match args.len() {
        0 => Ok(vm.ctx.new_Number(0.)),
        _ => match args[0].ToNumber(vm) {
            Err(e) => Err(e),
            Ok(v) => {
                if let Value::Number(n) = v {
                    Ok(vm.ctx.new_Number(n))
                } else {
                    Err("Fatal Error: ToNumber didn't return Value::Number".into())
                }
            }
        },
    }
}

pub fn valueOf(vm: &mut VM, _args: &[Value]) -> JSResult {
    let this = vm.get_this();
    Ok(extract_number!(this).value.into())
}

pub fn toString(vm: &mut VM, _args: &[Value]) -> JSResult {
    let this = vm.get_this();
    Ok(extract_number!(this).value.to_string().into())
}
