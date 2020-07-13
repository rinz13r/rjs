mod vm;
use vm::vm::VM;
use vm::context::Context;

mod codegen;
use codegen::codegen::gen_code;

mod objects;

use std::fs;

fn main() {
    let filename = "examples/print.js";
    let js = fs::read_to_string(filename).unwrap();
    let ctx = Context::new ();
    let code = gen_code(js, &ctx);
    let mut vm = VM::new(code, &ctx);
    vm.run();
}
