mod vm;
use vm::context::Context;
use vm::vm::VM;

mod codegen;
use codegen::codegen::gen_code;

mod objects;

use std::env;
use std::fs;

#[test]
fn test_new() {
    let filename = String::from("examples/new.js");
    let js = match fs::read_to_string(filename) {
        Ok(js) => js,
        Err(msg) => panic!("{}", msg.to_string()),
    };
    let ctx = Context::new();
    let code = gen_code(js, &ctx);
    let mut vm = VM::new(code, &ctx);
    vm.run();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.len() {
        2 => args[1].clone(),
        _ => String::from("examples/member.js"),
    };
    let js = match fs::read_to_string(filename) {
        Ok(js) => js,
        Err(msg) => panic!("{}", msg.to_string()),
    };
    let ctx = Context::new();
    let code = gen_code(js, &ctx);
    dbg!(&code.instrs);
    let mut vm = VM::new(code, &ctx);
    vm.run();
}
