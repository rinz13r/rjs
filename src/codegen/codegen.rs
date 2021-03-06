extern crate ressa;
use resast::prelude::*;
use ressa::Parser;

use crate::vm::code::*;
use crate::vm::context::Context;
use crate::vm::value::Value;

// use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;

struct CodeGen<'a> {
    instrs: Vec<Instruction>,
    consts: Vec<Value>,
    names: Vec<String>,
    index_of_name: HashMap<String, usize>,
    index_of_param: HashMap<String, usize>,
    is_func: bool,
    in_load_prop: bool,
    ctx: &'a Context,
    this_stack_len: usize,
    in_call_expr: bool,
}

impl<'a> CodeGen<'a> {
    fn new(is_func: bool, ctx: &'a Context) -> Self {
        CodeGen {
            instrs: Vec::new(),
            consts: Vec::new(),
            names: Vec::new(),
            index_of_name: HashMap::new(),
            index_of_param: HashMap::new(),
            is_func,
            in_load_prop: false,
            ctx,
            this_stack_len: 0,
            in_call_expr: false,
        }
    }
    fn gen(src: String, ctx: &'a Context) -> Code {
        let mut parser = Parser::new(src.as_str()).expect("Failed to create parser");
        let program = parser.parse().expect("Unabl eto parse");
        let mut codegen = CodeGen::new(false, ctx);
        match program {
            Program::Script(parts) => codegen.code(parts),
            Program::Mod(_parts) => panic!("Modules not implemented"),
        };
        Code::new(codegen.instrs, codegen.consts, codegen.names)
    }
    fn code(&mut self, parts: Vec<ProgramPart>) {
        for p in parts {
            match p {
                ProgramPart::Stmt(stmt) => self.visit_stmt(stmt),
                ProgramPart::Decl(decl) => self.visit_decl(decl),
                _ => panic!("Not impl"),
            }
        }
    }
    fn visit_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Empty => (),
            Stmt::Expr(expr) => self.visit_expr(expr),
            Stmt::Return(ret) => {
                if let Some(expr) = ret {
                    self.visit_expr(expr);
                    self.instrs.push(Instruction::Return);
                } else {
                    self.instrs.push(Instruction::LoadUndefined);
                }
            }
            Stmt::If(stmt) => {
                self.visit_expr(stmt.test);

                self.instrs.push(Instruction::PopJumpIfFalse(0));
                let index1 = self.instrs.len() - 1;

                self.visit_stmt(*stmt.consequent);

                self.instrs.push(Instruction::Jump(0));
                let index2 = self.instrs.len() - 1;
                self.instrs[index1] = Instruction::PopJumpIfFalse(index2 + 1);

                if let Some(elseif) = stmt.alternate {
                    self.visit_stmt(*elseif);
                }
                let index3 = self.instrs.len() - 1;
                self.instrs[index2] = Instruction::Jump(index3 + 1);
            }
            Stmt::Block(block) => {
                for p in block.0 {
                    match p {
                        ProgramPart::Stmt(stmt) => self.visit_stmt(stmt),
                        ProgramPart::Decl(decl) => self.visit_decl(decl),
                        _ => panic!("Not impl"),
                    }
                }
            }
            _ => panic!("Unimplemented stmt, {:?}", stmt),
        }
    }
    fn visit_decl(&mut self, decl: Decl) {
        match decl {
            Decl::Var(kind, decls) => {
                if kind != VarKind::Var {
                    panic!("Only 'var' decls supported");
                }
                for decl in decls {
                    match decl {
                        VarDecl { id, init } => {
                            if let Some(init) = init {
                                self.visit_expr(init);
                            } else {
                                self.instrs.push(Instruction::LoadUndefined);
                            }
                            if let Pat::Ident(ident) = id {
                                let name = ident.name;
                                self.instrs.push(Instruction::StoreName(
                                    match self.index_of_name.get(&name.to_string()) {
                                        Some(idx) => *idx,
                                        None => {
                                            self.names.push(name.to_string());
                                            self.index_of_name
                                                .insert(name.to_string(), self.names.len() - 1);
                                            self.names.len() - 1
                                        }
                                    },
                                ));
                            } else {
                                // TODO: Better error messages and handling, not just panic!
                                panic!("unsupported construct");
                            }
                        }
                    }
                }
            }

            Decl::Func(Func {
                id, params, body, ..
            }) => {
                let id = id.expect("function statement requires a name");
                let mut codegen = Self::new(true, self.ctx);
                for i in 0..params.len() {
                    let ref param = params[i];
                    if let FuncArg::Pat(pat) = param {
                        if let Pat::Ident(ident) = pat {
                            let ref name = ident.name;
                            codegen.index_of_param.insert(name.to_string(), i);
                        }
                    }
                }
                codegen.visit_fnbody(body);
                let code = Code::new(codegen.instrs, codegen.consts, codegen.names);
                // let obj = value::Value::new_function(Rc::from(code), params.len());
                let val = self
                    .ctx
                    .new_Function(id.name.to_string(), Rc::from(code), params.len());
                self.consts.push(val);
                self.names.push(id.name.to_string());
                self.instrs
                    .push(Instruction::LoadConst(self.consts.len() - 1));
                self.instrs
                    .push(Instruction::StoreName(self.names.len() - 1));
            }

            _ => panic!("{:?} decl not supported", decl),
        }
    }
    fn visit_fnbody(&mut self, body: FuncBody) {
        self.code(body.0);
    }
    fn visit_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Lit(lit) => {
                self.instrs.push(Instruction::LoadConst(self.consts.len()));
                match lit {
                    Lit::Null => self.instrs.push(Instruction::LoadNull),
                    Lit::Number(std::borrow::Cow::Borrowed(b)) => {
                        self.consts
                            .push(b.parse::<f64>().unwrap_or_default().into());
                    }
                    Lit::Number(std::borrow::Cow::Owned(b)) => {
                        self.consts
                            .push(b.parse::<f64>().unwrap_or_default().into());
                    }
                    Lit::String(StringLit::Double(std::borrow::Cow::Owned(b))) => {
                        self.consts.push(Value::String(b.to_string()));
                    }
                    Lit::String(StringLit::Double(std::borrow::Cow::Borrowed(b))) => {
                        self.consts.push(Value::String(b.to_string()));
                    }
                    Lit::Boolean(b) => {
                        self.instrs.pop().expect("Instruction underflow");
                        self.instrs.push(Instruction::LoadBool(b));
                    }
                    _ => panic!("No support for expr: {:?} yet", lit),
                }
            }
            Expr::Binary(BinaryExpr {
                operator,
                left,
                right,
            }) => {
                self.visit_expr(*left);
                self.visit_expr(*right);
                self.instrs.push(match operator {
                    BinaryOp::Plus => Instruction::BinAdd,
                    BinaryOp::Minus => Instruction::BinSub,
                    BinaryOp::Equal => Instruction::BinEq,
                    _ => panic!("operator '{:?}' not supported yet ", operator),
                });
            }
            Expr::Call(CallExpr { callee, arguments }) => {
                let len = arguments.len();
                for arg in arguments {
                    self.visit_expr(arg);
                }
                let this_stack_len = self.this_stack_len;
                let in_call_expr = self.in_call_expr;
                self.in_call_expr = true;
                self.visit_expr(*callee);
                self.instrs.push(Instruction::Call(len));
                for _ in 0..(self.this_stack_len - this_stack_len) {
                    self.instrs.push(Instruction::PopThis);
                }
                self.in_call_expr = in_call_expr;
                self.this_stack_len = this_stack_len;
            }
            Expr::Ident(Ident { name }) => {
                if self.in_load_prop {
                    self.consts.push(Value::String(String::from(name)));
                    self.instrs
                        .push(Instruction::LoadConst(self.consts.len() - 1));
                    return;
                }
                if self.is_func {
                    match self.index_of_param.get(&name.to_string()) {
                        Some(idx) => {
                            self.instrs.push(Instruction::LoadArg(*idx));
                            return;
                        }
                        None => (),
                    }
                }
                self.instrs.push(Instruction::LoadName(
                    match self.index_of_name.get(&name.to_string()) {
                        Some(idx) => *idx,
                        None => {
                            self.names.push(name.to_string());
                            self.index_of_name
                                .insert(name.to_string(), self.names.len() - 1);
                            self.names.len() - 1
                        }
                    },
                ));
            }
            Expr::Member(MemberExpr {
                object, property, ..
            }) => {
                self.visit_expr(*object);
                if self.in_call_expr {
                    self.instrs.push(Instruction::PushThis);
                    self.this_stack_len += 1;
                    self.in_call_expr = false;
                }
                let prev = self.in_load_prop;
                self.in_load_prop = true;
                self.visit_expr(*property);
                self.instrs.push(Instruction::LoadProperty);
                self.in_load_prop = prev;
            }
            Expr::Assign(AssignExpr {
                left,
                right,
                operator,
            }) => {
                if operator != AssignOp::Equal {
                    panic!("Operator {:?} not supported", operator);
                }
                self.visit_expr(*right);
                match left {
                    AssignLeft::Expr(expr) => self.visit_expr(*expr),
                    AssignLeft::Pat(pat) => self.visit_pat(pat),
                }
                let instr = match self.instrs.pop() {
                    None => panic!("Couldn't compute instr"),
                    Some(instr) => match instr {
                        Instruction::LoadProperty => Instruction::StoreProperty,
                        _ => instr,
                    },
                };
                self.instrs.push(instr);
            }
            Expr::This => {
                self.instrs.push(Instruction::LoadThis);
            }
            Expr::New(NewExpr { callee, arguments }) => {
                let nargs = arguments.len();
                for expr in arguments {
                    self.visit_expr(expr);
                }
                self.visit_expr(*callee);
                self.instrs.push(Instruction::New(nargs));
            }
            Expr::Func(func) => {
                self.visit_func(func);
            }
            Expr::Array(els) => {
                // TODO: Empty array slots
                let mut len = 0;
                for el in els {
                    if let Some(el) = el {
                        self.visit_expr(el);
                        len += 1;
                    }
                }
                self.instrs.push(Instruction::MakeArray(len));
            }
            _ => panic!("Unimplemented {:?}", expr),
        }
    }

    fn visit_func(&mut self, func: Func) {
        let (id, params, body) = (func.id, func.params, func.body);
        // let id = match id {
        //     None => String::from ("[anonymous function]"),
        //     Some (id) => id.name.to_string ()
        // };
        // let id = id.expect("function statement requires a name");
        let mut codegen = Self::new(true, self.ctx);
        for i in 0..params.len() {
            let ref param = params[i];
            if let FuncArg::Pat(pat) = param {
                if let Pat::Ident(ident) = pat {
                    let ref name = ident.name;
                    codegen.index_of_param.insert(name.to_string(), i);
                }
            }
        }
        codegen.visit_fnbody(body);
        let code = Code::new(codegen.instrs, codegen.consts, codegen.names);
        // let obj = value::Value::new_function(Rc::from(code), params.len());
        // let val = value::Value::new_functionobject(self.ctx, Rc::from(code), params.len());
        let val = self.ctx.new_Function(
            if let Some(ref id) = id {
                id.name.to_string()
            } else {
                "".to_string()
            },
            Rc::from(code),
            params.len(),
        );
        self.consts.push(val);
        self.instrs
            .push(Instruction::LoadConst(self.consts.len() - 1));

        match id {
            Some(id) => {
                self.names.push(id.name.to_string());
                self.instrs
                    .push(Instruction::StoreName(self.names.len() - 1));
            }
            None => (),
        };
    }
    fn visit_pat(&mut self, _pat: Pat) {
        panic!("pat");
    }
}

pub fn gen_code<'a>(src: String, ctx: &'a Context) -> Code {
    CodeGen::gen(src, ctx)
}
