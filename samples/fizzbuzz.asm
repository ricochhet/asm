; output
; buzz
; 19
; fizz
; 17
; 16
; fizzbuzz
; 14
; 13
; fizz
; 11
; buzz
; fizz
; 8
; 7
; fizz
; buzz
; 4
; fizz
; 2
; 1

proc loop
    ; store how many fizz-buzz iterations we want do
    pushint 21
    mov 0 -1
    jmp loop

    label loop
        ; load from mov 0 (20)
        ld 0
        ; decrement the value we just loaded
        decr
        ; overwrite the stored value (x--)
        mov 0 -1
        je break
        ; pop the decremented value
        pop

        ; =======
        ; if-elif-else
        ; if x % 15 == 0
            
            ; load the decremented value
            ld 0
            ; push the value we want to perform the modulo op with
            pushint 15
            mod
                ; fizzbuzz
            je mod15

        ; if x % 3 == 0

            ; load the decremented value
            ld 0
            ; push the value we want to perform the modulo op with
            pushint 3
            mod
                ; fizz
            je mod3 

        ; if x % 5 == 0

            ; load the decremented value
            ld 0
            ; push the value we want to perform the modulo op with
            pushint 5
            mod
                ; buzz
            je mod5

        ; =========

        ld 0
        prntln
        pop
        jne loop

        ret

    label break
        ; cleanup
        clsreg
        dlcreg
        clsstk
        dlcstk
        ret

    label mod15
        ; pop modulo if stack is not empty
        pop
        prntstr fizzbuzz
        jmp loop
        ret
    
    label mod3
        ; pop modulo if stack is not empty
        pop
        prntstr fizz
        jmp loop
        ret

    label mod5
        ; pop modulo if stack is not empty
        pop
        prntstr buzz
        jmp loop
        ret
end

call loop