mod vm;
use vm::vm::VM;

mod codegen;
use codegen::codegen::gen_code;

use std::fs;

fn main() {
    let filename = "examples/print.js";
    let js = fs::read_to_string(filename).unwrap();
    let code = gen_code(js);
    let mut vm = VM::new(code);
    vm.run();
}
