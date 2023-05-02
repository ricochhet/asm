PushInt 54321
PushInt 12345
PushInt 99999
Mov 0 -1
Pop
Pop
Pop

PushStr hello
Mov 1 -1
Pop

PrintRegisters

Ld 0
Ld 1
PrintStack