use super::*;

#[derive(Trace, Finalize, Debug, Clone)]
pub struct Number {
    value: f64,
}
impl Number {
    pub fn new(value: f64) -> Self {
        Self { value }
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
pub fn function(_vm: &mut VM, _this: Value, args: &[Value]) -> JSResult {
    Ok(match args.len() {
        0 => Value::Number(0.),
        _ => args[0].ToNumber(),
    })
}

pub fn constructor(_vm: &mut VM, _this: Value, _args: &[Value]) -> JSResult {
    Err("sdf".into())
}

pub fn valueOf(_vm: &mut VM, this: Value, _args: &[Value]) -> JSResult {
    Ok(extract_number!(this).value.into())
}

pub fn toString(_vm: &mut VM, this: Value, _args: &[Value]) -> JSResult {
    Ok(extract_number!(this).value.to_string().into())
}
