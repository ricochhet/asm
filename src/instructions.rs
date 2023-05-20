use crate::stack::Pointer;

#[non_exhaustive]
pub struct Type;

impl Type {
    pub const INT: &str = "int";
    pub const STRING: &str = "str";
    pub const FLOAT: &str = "flt";
}

// TODO: Possibly add some instructions to assist with debugging?
// These would sort of act like environment variables.
//
// I would probably want to add a distinct marker for these variables,
// Possibly using the #, @, &, or $ symbol, although I would want to determine
// use cases for the other symbols first.
//
// Debug variables would not change any instruction, but would print what the instruction is doing
// I would want to pretty-print the value in some way so it doesn't feel like a gigantic dump of print
// statements when running your program.
//
//
// TODO: Consider renaming instructions to fit a more concise convention, I don't know exactly how I want
// to do this just yet, but the amount of instructions is going to increase and get more "cluttered"
// whether to make instructions more verbose or less verbose may be instruction-dependant.
//
// Things like `add` and `addf` may get annoying if we add other types. We could possibly just do `addi` and `addf`
// Should instructions like cls* and dlc* be converted to a clear * and dealloc * type? I'm personally not a huge fan
// of having to specify an enum-like string such as "clear stack" or "dealloc hash" but it would be confusing to use
// integers to reference what we want to delete or clear as well.
//
// There's not really anything wrong with making more verbose instructions like "dealloc_hash" but I want to keep the "trend"
// of an assembly like language abbreviating everything, just because I think it looks cool and overly technical.
#[derive(Debug)]
pub enum Instruction {
    PushInt(isize),
    PushFlt(f32),
    PushStr(String),
    Pop,
    Dup,
    Swap,
    ClrStk, // clear
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
    ClrHash, // clear
    DlcHash, // dealloc
    DmpReg(isize),
    ClrReg, // clear
    DlcReg, // dealloc
    Cmp(Pointer),
    IntHas(Pointer),
    StrHas(Pointer),
    FltHas(Pointer),
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
    PrntStr(String),
    Prntln,
    PrntC,
    PrntCln,
    PrntStk,
    PrntReg,
    Call(Pointer),
    Ret,
}
