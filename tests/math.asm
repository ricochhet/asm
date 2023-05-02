Proc math
    GetArg 0
    GetArg 1
    Mov 0 0
    Mov 1 1
    Pop
    Pop

    Ld 1
    Ld 0
    Div
    Mov 2 0
    Pop
    Pop
    Pop
    Ld 2
    Ld 1
    Mul
    DmpReg 0
    DmpReg 1
    DmpReg 2
    PrintRegisters
    PrintStack
    Ret
End

PushInt 2
PushInt 21
Call math