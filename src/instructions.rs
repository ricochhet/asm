use crate::stack::Pointer;

#[derive(Debug)]
pub enum Instruction {
    PushInt(isize),
    PushStr(String),
    Pop,
    Add,
    Sub,
    Incr,
    Decr,
    Mul,
    Div,
    Mov(isize, Pointer),
    Ld(isize),
    Cmp(Pointer),
    Incl(Pointer),
    Jump(Pointer),
    JE(Pointer),
    JNE(Pointer),
    JGT(Pointer),
    JLT(Pointer),
    JGE(Pointer),
    JLE(Pointer),
    Get(Pointer),
    Set(Pointer),
    GetArg(Pointer),
    SetArg(Pointer),
    Noop,
    Print,
    PrintC,
    PrintStack,
    PrintRegisters,
    Call(Pointer),
    Ret,
}
