Proc printStr
    -- [..., i | ]
    GetArg 1
    -- [..., last_char, i | last_char ]
    PrintC
    Pop
    -- [..., last_char, i | ]
    SetArg 1
    -- [..., i - 1, i| ]
    Pop
    -- [..., i | ]
    PushInt 1
    -- [..., i | 1 ]
    Sub
    -- [..., i - 1 ]

    GetArg 1
    JE finish
    JNE continue

    label finish
        Ret

    label continue 
        Call printStr
        Ret
End

-- \n
PushInt 10

-- d
PushInt 100

-- l
PushInt 108

-- r
PushInt 114

-- o
PushInt 111

-- W
PushInt 87

-- space
PushInt 32

-- o
PushInt 111

-- l
PushInt 108

-- l
PushInt 108

-- e
PushInt 101

-- H
PushInt 72

-- string length
PushInt 12

Call printStr
PrintStack