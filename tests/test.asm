Proc helloworld
    PushStr value1/value
    PushStr value1/

    Strhas n1
    Ret

    label n1
        PushInt 69696969
        Ret
End

Call helloworld
PrintStack