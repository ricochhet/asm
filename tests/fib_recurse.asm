proc fib
    decr
    jle retOne

    ; [n - 1 |]
    getarg 0
    ; [n - 1 | n - 1]
    call fib
    ; [n - 1 | fib(n - 1)]
    getarg 0
    decr
    ; [n - 1 | fib(n - 1), n - 2]
    call fib
    add
    ; [n - 1 | fib(n - 1) + fib(n - 2)]
    setarg 0
    pop
    ret

    label retOne
        pushint 1
        ret
end

pushint 34
call fib
prnt

pushint 10
prntc
