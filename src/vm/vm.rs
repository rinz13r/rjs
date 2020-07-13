use crate::vm::code::*;
use crate::vm::value::*;
use super::context::*;
use crate::objects::*;

extern crate gc;

use std::collections::HashMap;
use std::rc::Rc;

pub struct VM <'a> {
    callstack: Vec<Frame>,
    global_scope: HashMap<String, Value>,
    scopes: Vec<HashMap<String, Value>>,
    ctx: &'a Context,
    // this: GcBox<Object>,
}

struct Frame {
    datastack: Vec<Value>,
    code: Rc<Code>,
    ip: usize,
    nargs: usize,
}

impl Frame {
    fn new(code: Rc<Code>, nargs: usize) -> Self {
        Frame {
            datastack: Vec::new(),
            code,
            ip: 0,
            nargs,
        }
    }
}

fn builtin_print(arguments: &Vec<Value>) -> JSResult {
    for arg in arguments {
        print!("{} ", arg);
    }
    println!();
    Ok(Value::Undefined)
}

impl<'a> VM<'a> {
    pub fn new(code: Code, ctx: &'a Context) -> Self {
        let callstack = vec![Frame::new(Rc::from(code), 0)];
        Self::init_vm(VM {
            callstack,
            global_scope: HashMap::new(),
            scopes: Vec::new(),
            ctx
        })
    }

    fn init_vm(vm: Self) -> Self {
        let mut vm = vm;
        let print = Value::from_rjsfunc (builtin_print, "print");
        vm.global_scope.insert("print".to_string(), print);
        vm
    }

    #[inline]
    fn vec_back<T>(v: &mut Vec<T>) -> Option<&mut T> {
        match v.len() {
            0 => None,
            n => Some(&mut v[n - 1]),
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
            let ref instr = instrs[frm.ip];
            frm.ip += 1;
            match instr {
                Instruction::LoadUndefined => frm.datastack.push(Value::Undefined),
                Instruction::LoadNull => frm.datastack.push(Value::Null),
                Instruction::PrintTop => match frm.datastack.pop() {
                    None => panic!("Datastack empty"),
                    Some(v) => println!("{}", v),
                },
                Instruction::BinAdd => match (frm.datastack.pop(), frm.datastack.pop()) {
                    (Some(v1), Some(v2)) => frm.datastack.push(v1 + v2),
                    _ => panic!("stack underflow during BinOp"),
                },
                Instruction::LoadConst(idx) => frm.datastack.push(match idx {
                    n if *n < consts.len() => consts[*n].clone (),
                    _ => panic!("const cannot be indexed"),
                }),
                Instruction::Call(nargs) => {
                    let v = frm.datastack.pop ().unwrap ();
                    let mut arguments = Vec::new();
                    for _ in 0..*nargs {
                        if let Some(v) = frm.datastack.pop() {
                            arguments.push(v);
                        }
                    }
                    let res = v.call (self, &arguments);
                    if let Some(frm) = Self::vec_back(&mut self.callstack) {
                        match res {
                            Ok(val) => frm.datastack.push(val),
                            Err(msg) => panic!(msg),
                        }
                    };
                }
                Instruction::LoadName(idx) => {
                    let ref name = frm.code.names[*idx];
                    let mut found = false;
                    if let Some(scope) = Self::vec_back(&mut self.scopes) {
                        if let Some(v) = scope.get(name) {
                            frm.datastack.push(v.clone ());
                            found = true;
                        }
                    }
                    if !found {
                        if let Some(v) = self.global_scope.get(name) {
                            frm.datastack.push(v.clone ());
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
                    frm.datastack.push(frm.datastack[*idx].clone ());
                }
                Instruction::New(_nargs) => {
                    // TODO
                }
            }
        }
        Ok(Value::Undefined) // Default return value of a frame
    }

    pub fn call_code(&mut self, code: Rc<Code>, arity: usize, args: &Vec<Value>) -> JSResult {
        let mut frm = Frame::new(code, arity);
        for arg in args {
            frm.datastack.push(arg.clone ());
        }
        for _ in args.len()..arity {
            frm.datastack.push(Value::Undefined);
        }
        self.callstack.push(frm);
        self.exec_top_frame()
    }
}
