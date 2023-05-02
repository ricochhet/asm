use crate::stack::Pointer;

#[derive(Debug)]
pub enum Instruction {
    PushInt(isize),
    PushFloat(f32),
    PushStr(String),
    Pop,
    ClsStk, // clear
    DlcStk, // dealloc
    Add,    // int
    AddF,   // float
    Sub,    // int
    SubF,   // float
    Incr,   // int
    Decr,   // int
    Mul,    // int
    MulF,   // float
    Div,    // int
    DivF,   // float
    Mod,    // int
    ModF,   // Float
    Mov(isize, isize),
    Ld(isize),
    DmpHash(isize),
    ClsHash, // clear
    DlcHash, // dealloc
    DmpReg(isize),
    ClsReg, // clear
    DlcReg, // dealloc
    Cmp(Pointer),
    InclI(Pointer),
    InclS(Pointer),
    InclF(Pointer),
    Jmp(Pointer),
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
    Prnt,
    PrntC,
    PrntStk,
    PrntReg,
    Call(Pointer),
    Ret,
}
