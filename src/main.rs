mod vm;
use vm::context::Context;
use vm::vm::VM;

mod codegen;
use codegen::codegen::gen_code;

mod objects;

use std::fs;

fn main() {
    let filename = "examples/member.js";
    let js = fs::read_to_string(filename).unwrap();
    let ctx = Context::new();
    let code = gen_code(js, &ctx);
    // dbg!(&code.instrs);
    // dbg!(&code.names);
    let mut vm = VM::new(code, &ctx);
    vm.run();
}
