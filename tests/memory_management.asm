PushStr hello
PrintStack
; Stack: 1335831723 --> Hash: (String("hello"))

PushInt 10
PrintC
Pop

Mov 0 -1
PrintRegisters
-- Register: Some(StackValue { value: 1335831723, hashed: true })

PushInt 10
PrintC
Pop

DmpHash -1
PrintStack
-- Stack: 1335831723 --> Hash: (String("None"))
; Value has been deleted from the table
