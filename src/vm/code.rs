use crate::vm::value::Value;

#[derive(Debug)]
pub enum Instruction {
    LoadUndefined,
    LoadNull,
    LoadBool(bool),
    PrintTop,
    BinAdd,
    BinSub,
    BinEq,
    LoadConst(usize),
    Call(usize),
    LoadName(usize),
    StoreName(usize),
    LoadArg(usize),
    New(usize),
    LoadProperty,
    StoreProperty,
    LoadThis,
    Return,
    Throw,
    PushThis,
    PopThis,
    PopJumpIfFalse(usize),
    Jump(usize),
    MakeArray(usize),
}

#[derive(Debug)]
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
