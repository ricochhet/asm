Proc fib
    Decr
    JLE retOne

    -- [n - 1 |]
    GetArg 0
    -- [n - 1 | n - 1]
    Call fib
    -- [n - 1 | fib(n - 1)]
    GetArg 0
    Decr
    -- [n - 1 | fib(n - 1), n - 2]
    Call fib
    Add
    -- [n - 1 | fib(n - 1) + fib(n - 2)]
    SetArg 0
    Pop
    Ret

    label retOne
        Push 1
        Ret
End

Push 35
Call fib
Print

Push 10
PrintC
