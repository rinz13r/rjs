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
    pub fn toString(&self, vm: &mut VM) -> JSResult {
        let obj = vm.get_this().unwrap_object();
        if let Some(ref proto) = obj.borrow().__proto__ {
            if let Value::Object(ref toString) = proto.borrow().Get(&"toString".to_string()) {
                return toString.borrow().Call(vm, &[]);
            }
        }
        panic!("Fatal Error: toString not found");
    }
}
