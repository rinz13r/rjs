use super::context::*;
use crate::objects::*;
use crate::vm::code::*;
use crate::vm::value::*;

use crate::objects;

extern crate gc;

use std::collections::HashMap;
use std::rc::Rc;

pub struct VM<'a> {
    callstack: Vec<Frame>,
    global_scope: HashMap<String, Value>,
    scopes: Vec<HashMap<String, Value>>,
    pub ctx: &'a Context,
    // this: GcBox<Object>,
    thises: Vec<Value>, // Execution contexts
    throw_stack: Vec<Value>,
}

struct Frame {
    datastack: Vec<Value>,
    code: Rc<Code>,
    ip: usize,
}

impl Frame {
    fn new(code: Rc<Code>) -> Self {
        Frame {
            datastack: Vec::new(),
            code,
            ip: 0,
        }
    }
}

fn builtin_print(vm: &mut VM, _this: Value, arguments: &[Value]) -> JSResult {
    for arg in arguments {
        print!("{} ", arg.to_string());
        // match arg.to_string() {
        //     Ok(s) => print!("{} ", s),
        //     Err(msg) => return Err(msg),
        // }
    }
    println!();
    Ok(Value::default())
}

impl<'a> VM<'a> {
    pub fn new(code: Code, ctx: &'a Context) -> Self {
        let callstack = vec![Frame::new(Rc::from(code))];
        Self::init_vm(VM {
            callstack,
            global_scope: HashMap::new(),
            scopes: Vec::new(),
            ctx,
            thises: vec![Value::default()],
            throw_stack: Vec::new(),
        })
    }

    fn init_vm(vm: Self) -> Self {
        let mut vm = vm;
        vm.global_scope
            .insert("Object".to_string(), vm.ctx.Object_function.clone().into());
        vm.global_scope.insert(
            "Function".to_string(),
            vm.ctx.Function_function.clone().into(),
        );
        vm.global_scope.insert(
            "print".to_string(),
            vm.ctx
                .new_PrimitiveFunction("print", builtin_print, builtin_print, 0)
                .into(),
        );
        vm
    }

    #[inline]
    fn vec_back<T>(v: &mut Vec<T>) -> Option<&mut T> {
        match v.len() {
            0 => None,
            n => Some(&mut v[n - 1]),
        }
    }
    #[inline(always)]
    fn vec_back_ref<T>(v: &Vec<T>) -> Option<&T> {
        match v.len() {
            0 => None,
            n => Some(&v[n - 1]),
        }
    }
    // TODO: return Result to indicate success or uncaught expcetion
    pub fn run(&mut self) {
        while self.callstack.len() > 0 {
            self.exec_top_frame();
        }
    }
    pub fn exec_top_frame(&mut self) -> JSResult {
        while let Some(frm) = Self::vec_back(&mut self.callstack) {
            let ref instrs = frm.code.instrs;
            if frm.ip >= instrs.len() {
                self.callstack.pop();
                return Ok(Value::Undefined);
            }
            let ref consts = frm.code.consts;
            let ref names = frm.code.names;
            let ref instr = instrs[frm.ip];
            frm.ip += 1;
            match instr {
                Instruction::LoadUndefined => frm.datastack.push(Value::Undefined),
                Instruction::LoadNull => frm.datastack.push(Value::Null),
                Instruction::LoadBool(b) => {
                    frm.datastack.push(Value::Boolean(*b));
                }
                Instruction::PrintTop => match frm.datastack.pop() {
                    None => panic!("Datastack empty"),
                    Some(v) => println!("{}", v),
                },
                Instruction::BinAdd => match (frm.datastack.pop(), frm.datastack.pop()) {
                    (Some(v1), Some(v2)) => frm.datastack.push(v1 + v2),
                    _ => panic!("stack underflow during BinOp"),
                },
                Instruction::BinSub => match (frm.datastack.pop(), frm.datastack.pop()) {
                    (Some(v1), Some(v2)) => frm.datastack.push(v2 - v1),
                    _ => panic!("stack underflow during BinOp"),
                },
                Instruction::BinEq => match (frm.datastack.pop(), frm.datastack.pop()) {
                    (Some(v1), Some(v2)) => frm.datastack.push(Value::Boolean(v1 == v2)),
                    _ => panic!("stack underflow during BinOp"),
                },
                Instruction::LoadConst(idx) => frm.datastack.push(match idx {
                    n if *n < consts.len() => consts[*n].clone(),
                    _ => panic!("const cannot be indexed"),
                }),
                Instruction::Call(nargs) => {
                    let v = frm.datastack.pop().unwrap();
                    let arguments = Vec::from(&frm.datastack[frm.datastack.len() - nargs..]);
                    for _ in 0..*nargs {
                        frm.datastack.pop().expect("datastack underflow");
                    }
                    let res = v.as_object(self.ctx).borrow().Call(
                        self,
                        self.get_this().clone(),
                        &arguments[..],
                    );
                    if let Some(frm) = Self::vec_back(&mut self.callstack) {
                        match &res {
                            Ok(val) => frm.datastack.push(val.clone()),
                            Err(_) => return res,
                        };
                    };
                }
                Instruction::LoadName(idx) => {
                    let ref name = names[*idx];
                    let mut found = false;
                    if let Some(scope) = Self::vec_back(&mut self.scopes) {
                        if let Some(v) = scope.get(name) {
                            frm.datastack.push(v.clone());
                            found = true;
                        }
                    }
                    if !found {
                        if let Some(v) = self.global_scope.get(name) {
                            frm.datastack.push(v.clone());
                        } else {
                            panic!("NameError: '{}' not found", name);
                        }
                    }
                }
                Instruction::StoreName(idx) => {
                    let ref name = frm.code.names[*idx];
                    let v = frm.datastack.pop().unwrap();
                    if let Some(scope) = Self::vec_back(&mut self.scopes) {
                        scope.insert(name.clone(), v);
                    } else {
                        self.global_scope.insert(name.clone(), v);
                    }
                }
                Instruction::LoadArg(idx) => {
                    frm.datastack.push(frm.datastack[*idx].clone());
                }
                Instruction::New(nargs) => {
                    let f = frm.datastack.pop().expect("datastack underflow");
                    let args = Vec::from(&frm.datastack[frm.datastack.len() - nargs..]);
                    for _ in 0..*nargs {
                        frm.datastack.pop().expect("data stack underflow");
                        // args.push (frm.datastack.pop().expect ("data stack underflow").clone ());
                    }
                    let res = f.as_object(self.ctx).borrow().Construct(self, &args);
                    if let Some(frm) = Self::vec_back(&mut self.callstack) {
                        match &res {
                            Ok(val) => frm.datastack.push(val.clone()),
                            Err(_) => return res,
                        }
                    };
                }
                Instruction::LoadProperty => {
                    let prop = frm
                        .datastack
                        .pop()
                        .expect("data stack underflow")
                        .to_string();
                    let v: Value = frm
                        .datastack
                        .pop()
                        .expect("data stack underflow")
                        .as_object(self.ctx)
                        .into();
                    self.thises.push(v.clone());
                    frm.datastack.push(v.unwrap_object().borrow().Get(&prop));
                    self.thises.pop();
                }
                Instruction::StoreProperty => {
                    let prop = frm
                        .datastack
                        .pop()
                        .expect("data stack underflow")
                        .to_string();
                    let lhs = frm.datastack.pop().expect("data stack underflow");
                    let rhs = frm.datastack.pop().expect("data stack underflow");
                    lhs.as_object(self.ctx).borrow_mut().Put(prop, rhs);
                }
                Instruction::LoadThis => {
                    frm.datastack.push(match Self::vec_back_ref(&self.thises) {
                        Some(v) => v.clone(),
                        None => panic!("failed to load this"),
                    });
                }
                Instruction::Return => {
                    let v = frm.datastack.pop().expect("datastack underflow");
                    self.callstack.pop();
                    return Ok(v);
                }
                Instruction::Throw => {
                    let v = frm.datastack.pop().expect("datastack underflow");
                    self.throw_stack.push(v.clone());
                    return Err(v);
                }
                Instruction::PushThis => {
                    let v = Self::vec_back_ref(&frm.datastack)
                        .expect("data stack underflow")
                        .as_object(self.ctx)
                        .into();
                    self.thises.push(v);
                }
                Instruction::PopThis => {
                    self.thises.pop().expect("'this' underflow");
                }
                Instruction::PopJumpIfFalse(delta) => {
                    let condition = frm.datastack.pop().expect("datastack underflow");
                    let predicate: bool = condition.into();
                    if !predicate {
                        frm.ip = *delta;
                    }
                }
                Instruction::Jump(delta) => {
                    frm.ip = *delta;
                }
                Instruction::MakeArray(_len) => {
                    panic!("Unimplemented")
                    // let els = Vec::from(frm.datastack[frm.datastack.len() - len..].to_vec());
                    // frm.datastack.drain(frm.datastack.len() - len..);
                    // frm.datastack.push(Value::new_arrayobject(self.ctx, els));
                }
            }
        }
        Ok(Value::Undefined) // Default return value of a frame
    }

    pub fn call_code(&mut self, code: Rc<Code>, arity: usize, args: &[Value]) -> JSResult {
        let mut frm = Frame::new(code);
        for arg in args {
            frm.datastack.push(arg.clone());
        }
        for _ in args.len()..arity {
            frm.datastack.push(Value::Undefined);
        }
        self.callstack.push(frm);
        let res = self.exec_top_frame();
        res
    }
    pub fn push_this(&mut self, this: Value) {
        self.thises.push(this);
    }
    pub fn pop_this(&mut self) -> Value {
        match self.thises.pop() {
            Some(v) => v.clone(),
            None => panic!("this underflow"),
        }
    }
    pub fn get_this(&self) -> &Value {
        match Self::vec_back_ref(&self.thises) {
            Some(v) => v,
            None => panic!("Reference to 'this' does not exist"),
        }
    }
}
