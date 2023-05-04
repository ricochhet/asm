# asm-vm

**The current included test scripts may be outdated.**

## about

This is essentially a virtual machine for a custom "assembly-like" instruction set. This is mostly just a toy project that I want to use for scripting in the future. I wanted to keep it free of "magic" in the sense that the instructions you see are what the instruction does. This also means "memory management" is entirely up to the user. This means every register and hashed value is not removed from their respective table when you remove it from the stack (See `dmphash` and `dmpreg`). 

Eventually I may add more `simd`-like instructions, primarily adding extended `mov` type instructions for mathematical operations. I will likely make indexing at a specific point in the stack impossible for this registers, as it would get cluttered and become inefficient. As an example, an instruction like `movadd` would perform the exact same operation as `add` but allow you to store it in the specified register.  

## data structures & architecture
The virtual machine is split into 3 different data structures to handle various parts of the program.

The primary data structure is a stack, in this case, a `Vec`. The vector contains a list of `StackValue`s which hold important information about each "working" / current value.

A `StackValue` contains an `isize`: value, and a `boolean`: hashed. The isize will contain either integers or a hashed value, which is indicated by the `hashed` boolean.

The secondary data structure is a hash table, in this case, a `HashMap`, which takes a key `isize` and a value `String`. 

`pushint` will push a value directly into the stack and mark hashed as `false`. `pushstr` will push an `FNV1A` hashed string into the stack, and mark hashed as `true`. Hashed values will get put into the `hash table` with the FNV1A hash as the key, and raw string as the value. 

A similar design is applied to registers, a register will first get defined using the `mov x y` instruction, the difference being the register identifier(x) will be directly put into a separate register table. `y` contains a reference to a `StackValue`.

*All non `int` types are hashed.*

## instructions
All instructions are currently case-sensitive (subject to change).

Instructions that have two variations follow an `int` `float` pattern. These instructions are separated because float operations require a hash table lookup for the value.

- `pushint x` push an integer(x) to the top of the stack.
- `pushfloat x` push a float(x) to the top of the stack.
- `pushstr x` push a string(x) to the top of the stack.
- `pop` pop the top item from the stack.
- `dup` duplicate the top item of the stack.
- `swap` swaps the top two items on the stack.
- `clsstk` clears the entire stack.
- `dlcstk` dealloc stack. Performs `shrink_to_fit()`. `clear()` does not deallocate memory.
- `add` `addf` pops the top two items from the stack, adds them, and pushes the result.
- `sub` `subf` pops the top two items from the stack, subtracts them, and pushes the result.
- `mul` `mulf` pops the top two items from the stack, multiplies them, and pushes the result.
- `div` `divf` pops the top two items from the stack, divides them, and pushes the result.
- `mod` `modf` pops the top two items from the stack, returns the remainder of them, and pushes the result.
- `incr` increments the top item of the stack by one.
- `decr` decrements the top item of the stack by one.
- `mov x y` moves y index into x register. y is a position in the stack (-1 is the top of the stack).
    - maximum registers(x) is currently the isize max `9223372036854775807`, although you will likely run out memory before hitting this point. 
- `ld x` pushes register x to the top of the stack.
- `dmphash x` deletes x from the hash table. x is an index in the stack (-1 is the top of the stack).
- `clshash` clears the entire hash table.
- `dlchash` dealloc hash table. Performs `shrink_to_fit()`. `clear()` does not deallocate memory.
- `dmpreg x` deletes x from the register table. x is an index in the stack (-1 is the top of the stack).
- `clsreg` clears the entire register table.
- `dlcreg` dealloc register table. Performs `shrink_to_fit()`. `clear()` does not deallocate memory.
- `jump x` jump to a defined label(x).
- `cmp x` compares the top two items items, and jumps to label(x) if truthy.
- `incl x` pops the top two items from the stack. checks if the second to last item in the stack contains the top-most item, jumps to label(x) if truthy.
- `je x` `jfe` peeks the top-most value, and jumps to label(x) if it is equal to `0`
- `jne x` `jfne` peeks the top-most value, and jumps to label(x) if it is not equal to `0`
- `jgt x` `jfgt` peeks the top-most value, and jumps to label(x) if it is greater than`0`
- `jlt x` `jflt` peeks the top-most value, and jumps to label(x) if it is less than `0`
- `jge x` `jfge` peeks the top-most value, and jumps to label(x) if it is greater than or equal to `0`
- `jle x` `jfle` peeks the top-most value, and jumps to label(x) if it is less than or equal to `0`
- `get x` gets an index(x) in the stack, and pushes the item at the index in stack to the top.
- `set x` sets an index(x) relative to the last item in the call stack to the top of the call stack.
- `getarg x` gets an index(x) in the stack, and pushes the item at the index in stack to the top.
- `setarg x` sets an index(x) relative to the last item in the call stack to the top of the call stack.
- `prnt` prints the top-most value in the stack.
- `prntln` prints the top-most value in the stack with a \n.
- `prntc` prints the top-most value in the stack as a char.
- `prntcln` prints the top-most value in the stack as a char with a \n.
- `prntstr x` prints value x as astring with a \n.
- `prntstk` prints everything in the stack (primarily for debugging).
- `prntreg` prints everything in the register table (primarily for debugging).
- `call x` calls a defined process(x) (`proc`).
- `ret` returns / exits the current context / "closure."
- `proc x` `end` define a high order label(x), which most be closed via the `end` keyword.
- `label x` define a label(x), which can be jumped to based on jump instructions.
    - Truthy instructions will fall through to the label if `ret` is not declared before the labels, whether they evaluated to true or not, unless you explicity define a seperate jump-based instruction.
- `-- x` `; x` defines a code comment(x), multiline is not supported, inline comments are not supported.