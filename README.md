# asm-vm

note that the current included test scripts may be outdated

## data structures & architecture
The virtual machine is split into 3 different data structures to handle various parts of the program.

The primary data structure is a stack, in this case, a `Vec`. The vector contains a list of `StackValue`s which hold important information about each "working" / current value.

A `StackValue` contains an `isize`: value, and a `boolean`: hashed. The isize will contain either integers or a hashed string value, which is indicated by the `hashed` boolean.

The secondary data structure is a hash table, in this case, a `HashMap`, which takes a key `isize` and a value `String`. 

`PushInt` will push a value directly into the stack and mark hashed as `false`. `PushStr` will push an `FNV1A` hashed string into the stack, and mark hashed as `true`. Hashed values will get put into the `hash table` with the FNV1A hash as the key, and raw string as the value. 

A similar design is applied to registers, a register will first get defined using the `Mov x y` instruction, the difference being the register identifier(x) will be directly put into a separate register table. `y` contains a reference to a `StackValue`.

## instructions
All instructions are currently case-sensitive (subject to change).

- `PushInt x` push an integer(x) to the top of the stack.
- `PushFloat x` push a float(x) to the top of the stack.
- `PushStr x` push a string(x) to the top of the stack.
- `Pop` pop the top item from the stack.
- `Add` `AddF` pops the top two items from the stack, adds them, and pushes the result.
- `Sub` `SubF` pops the top two items from the stack, subtracts them, and pushes the result.
- `Mul` `MulF` pops the top two items from the stack, multiplies them, and pushes the result.
- `Div` `DivF` pops the top two items from the stack, divides them, and pushes the result.
- `Incr` increments the top item of the stack by one.
- `Decr` decrements the top item of the stack by one.
- `Mov x y` moves y index into x register. y is a position in the stack.
    - maximum registers(x) is currently the isize max `9223372036854775807`, although you will likely run out memory before hitting this point. 
- `Ld x` pushes register x to the top of the stack.
- `DmpHash x` deletes x from the hash table. x is a key.
- `DmpReg x` deletes x from the register table. x is a key.
- `Jump x` jump to a defined label(x).
- `Cmp x` compares the top two items items, and jumps to label(x) if truthy.
- `Incl x` pops the top two items from the stack. checks if the second to last item in the stack contains the top-most item, jumps to label(x) if truthy.
- `JE x` `JFE` peeks the top-most value, and jumps to label(x) if it is equal to `0`
- `JNE x` `JFNE` peeks the top-most value, and jumps to label(x) if it is not equal to `0`
- `JGT x` `JFGT` peeks the top-most value, and jumps to label(x) if it is greater than`0`
- `JLT x` `JFLT` peeks the top-most value, and jumps to label(x) if it is less than `0`
- `JGE x` `JFGE` peeks the top-most value, and jumps to label(x) if it is greater than or equal to `0`
- `JLE x` `JFLE` peeks the top-most value, and jumps to label(x) if it is less than or equal to `0`
- `Get x` gets an index(x) in the stack, and pushes the item at the index in stack to the top.
- `Set x` sets an index(x) relative to the last item in the call stack to the top of the call stack.
- `GetArg x` gets an index(x) in the stack, and pushes the item at the index in stack to the top.
- `SetArg x` sets an index(x) relative to the last item in the call stack to the top of the call stack.
- `Print` prints the top-most value in the stack
- `PrintC` prints the top-most value in the stack as a char
- `PrintStack` prints everything in the stack (primarily for debugging)
- `PrintRegisters` prints everything in the register table (primarily for debugging)
- `Call x` calls a defined process(x) (`Proc`)
- `Ret` returns / exits the current context / "closure"
- `Proc x` `End` define a high order label(x), which most be closed via the `End` keyword.
- `label x` define a label(x), which can be jumped to based on jump instructions.
    - Truthy instructions will fall through to the label if `Ret` is not declared before the labels, whether they evaluated to true or not.
- `-- x` defines a code comment(x), multiline is not supported.