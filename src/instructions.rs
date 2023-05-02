use crate::stack::Pointer;

#[derive(Debug)]
pub enum Instruction {
    PushInt(isize),
    PushFloat(f32),
    PushStr(String),
    Pop,
    Add,  // int
    AddF, // float
    Sub,  // int
    SubF, // float
    Incr, // int
    Decr, // int
    Mul,  // int
    MulF, // float
    Div,  // int
    DivF, // float
    Mov(isize, Pointer),
    Ld(isize),
    DmpHash(isize),
    DmpReg(isize),
    Cmp(Pointer),
    InclI(Pointer),
    InclS(Pointer),
    InclF(Pointer),
    Jump(Pointer),
    JE(Pointer),   // int
    JFE(Pointer),  // float
    JNE(Pointer),  // int
    JFNE(Pointer), // float
    JGT(Pointer),  // int
    JFGT(Pointer), // float
    JLT(Pointer),  // int
    JFLT(Pointer), // float
    JGE(Pointer),  // int
    JFGE(Pointer), // float
    JLE(Pointer),  // int
    JFLE(Pointer), // float
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
