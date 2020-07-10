use crate::vm::value::Value;

#[derive(Debug)]
pub enum Instruction {
    LoadUndefined,
    LoadNull,
    PrintTop,
    BinAdd,
    LoadConst(usize),
    Call(usize),
    LoadName(usize),
    StoreName(usize),
    LoadArg(usize),
    New(usize),
}

pub struct Code {
    pub instrs: Vec<Instruction>,
    pub consts: Vec<Value>,
    pub names: Vec<String>,
}

impl Code {
    pub fn new(instrs: Vec<Instruction>, consts: Vec<Value>, names: Vec<String>) -> Self {
        Code {
            instrs,
            consts,
            names,
        }
    }
}
