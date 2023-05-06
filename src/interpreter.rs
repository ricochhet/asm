use crate::instructions::Instruction;
use crate::stack::*;
use std::collections::HashMap;

pub fn compile(buffer: String) {
    let line_splits = buffer.split('\n')
                            .map(|s| s.split_whitespace().collect::<Vec<_>>())
                            .filter(|s| !matches!(s.as_slice(), [] | ["--" | ";", ..]))
                            .collect::<Vec<_>>();

    let labels: Labels = line_splits.iter().enumerate().filter_map(|(i, s)| find_label(i, s.as_slice())).collect();
    let procedures: Procedures = find_procedures(line_splits.as_slice());
    let instructions: Vec<Instruction> = line_splits.iter().map(|s| parse_instruction(s.as_slice(), &labels, &procedures)).collect();

    run(&instructions[..]);
}

fn run(program: Program<'_>) {
    use Instruction::*;

    let mut stack: Stack = Stack { values: Vec::new(),
                                   hashmap: HashMap::new(),
                                   registers: HashMap::new() };
    let mut pointer: Pointer = 0;
    let mut call_stack = CallStack::new();

    while let Some(instruction) = program.get(pointer) {
        pointer += 1;

        match instruction {
            Noop => {}
            PushInt(d) => {
                stack.push_as_value(*d);
            }
            PushFloat(d) => {
                stack.push_hashed_float(*d);
            }
            PushStr(d) => stack.push_hashed_string(d),
            Pop => {
                stack.pop();
            }
            Dup => {
                let a = stack.peek();

                if a.hashed {
                    stack.push_as_hashed(a.value);
                } else {
                    stack.push_as_value(a.value);
                }
            }
            Swap => {
                let (a, b) = (stack.pop(), stack.pop());

                if a.hashed {
                    stack.push_as_hashed(a.value);
                } else {
                    stack.push_as_value(a.value);
                }

                if b.hashed {
                    stack.push_as_hashed(b.value);
                } else {
                    stack.push_as_value(b.value);
                }
            }
            ClsStk => stack.clear_stack(),
            DlcStk => {
                stack.shrink_stack();
            }
            Add => {
                let (a, b) = (stack.pop(), stack.pop());

                if !a.hashed && !b.hashed {
                    stack.push_as_value(a.value + b.value)
                }
            }
            AddF => {
                let (a, b) = (stack.pop(), stack.pop());

                if a.hashed && b.hashed {
                    if let (Some(ValueType::Float(a)), Some(ValueType::Float(b))) = (stack.hashmap.get(&a.value), stack.hashmap.get(&b.value)) {
                        stack.push_hashed_float(a + b);
                    }
                }
            }
            Sub => {
                let (a, b) = (stack.pop(), stack.pop());

                if !a.hashed && !b.hashed {
                    stack.push_as_value(b.value - a.value)
                }
            }
            SubF => {
                let (a, b) = (stack.pop(), stack.pop());

                if a.hashed && b.hashed {
                    if let (Some(ValueType::Float(a)), Some(ValueType::Float(b))) = (stack.hashmap.get(&a.value), stack.hashmap.get(&b.value)) {
                        stack.push_hashed_float(b - a);
                    }
                }
            }
            Mul => {
                let (a, b) = (stack.pop(), stack.pop());

                if !a.hashed && !b.hashed {
                    stack.push_as_value(a.value * b.value)
                }
            }
            MulF => {
                let (a, b) = (stack.pop(), stack.pop());

                if a.hashed && b.hashed {
                    if let (Some(ValueType::Float(a)), Some(ValueType::Float(b))) = (stack.hashmap.get(&a.value), stack.hashmap.get(&b.value)) {
                        stack.push_hashed_float(a * b);
                    }
                }
            }
            Div => {
                let (a, b) = (stack.pop(), stack.pop());

                if !a.hashed && !b.hashed {
                    stack.push_as_value(b.value / a.value)
                }
            }
            DivF => {
                let (a, b) = (stack.pop(), stack.pop());

                if a.hashed && b.hashed {
                    if let (Some(ValueType::Float(a)), Some(ValueType::Float(b))) = (stack.hashmap.get(&a.value), stack.hashmap.get(&b.value)) {
                        stack.push_hashed_float(b / a);
                    }
                }
            }
            Mod => {
                let (a, b) = (stack.pop(), stack.pop());

                if !a.hashed && !b.hashed {
                    stack.push_as_value(b.value % a.value);
                }
            }
            ModF => {
                let (a, b) = (stack.pop(), stack.pop());

                if a.hashed && b.hashed {
                    if let (Some(ValueType::Float(a)), Some(ValueType::Float(b))) = (stack.hashmap.get(&a.value), stack.hashmap.get(&b.value)) {
                        stack.push_hashed_float(b % a);
                    }
                }
            }
            Cmp(p) => {
                let (a, b) = (stack.pop(), stack.pop());

                if a.hashed && b.hashed {
                    if b.value == a.value {
                        stack.push_as_hashed(b.value);
                        pointer = *p;
                    }
                } else if !a.hashed && !b.hashed && b.value == a.value {
                    stack.push_as_value(b.value);
                    pointer = *p;
                }
            }
            Incr => stack.peek_mut().value += 1,
            Decr => stack.peek_mut().value -= 1,
            Mov(d, p) => {
                if p.is_negative() {
                    let ind = p.abs();
                    let pos = stack.len() - ind as usize;
                    let val = stack.get(pos);
                    stack.push_register(*d, *val);
                } else {
                    let a = *stack.get(*p as usize + call_stack.last().map_or(0, |s| s.stack_offset));
                    stack.push_register(*d, a);
                }
            }
            Ld(d) => {
                if let Some(register) = stack.registers.get(d) {
                    if register.hashed {
                        stack.push_as_hashed(register.value);
                    } else if !register.hashed {
                        stack.push_as_value(register.value);
                    }
                }
            }
            DmpHash(p) => {
                if p.is_negative() {
                    let ind = p.abs();
                    let pos = stack.len() - ind as usize;
                    let val = stack.get(pos);

                    if val.hashed {
                        stack.delete_hash(val.value);
                    }
                } else {
                    let a = *stack.get(*p as usize + call_stack.last().map_or(0, |s| s.stack_offset));

                    if a.hashed {
                        stack.delete_hash(a.value);
                    }
                }
            }
            ClsHash => {
                stack.clear_hashmap();
            }
            DlcHash => stack.shrink_hashmap(),
            DmpReg(p) => {
                stack.delete_register(*p);
            }
            ClsReg => stack.clear_registers(),
            DlcReg => stack.shrink_registers(),
            Jmp(p) => pointer = *p,
            IntHas(p) => {
                let (a, b) = (stack.pop(), stack.pop());

                if !a.hashed && !b.hashed {
                    let (str1, str2) = (&a.value.to_string(), &b.value.to_string());

                    if str2.contains(str1) {
                        stack.push_as_value(b.value);
                        pointer = *p;
                    }
                }
            }
            StrHas(p) => {
                let (a, b) = (stack.pop(), stack.pop());

                if a.hashed && b.hashed {
                    if let (Some(ValueType::String(str1)), Some(ValueType::String(str2))) = (stack.hashmap.get(&a.value), stack.hashmap.get(&b.value)) {
                        if str2.contains(str1) {
                            stack.push_as_hashed(b.value);
                            pointer = *p;
                        }
                    }
                }
            }
            FltHas(p) => {
                let (a, b) = (stack.pop(), stack.pop());

                if a.hashed && b.hashed {
                    if let (Some(ValueType::Float(str1)), Some(ValueType::Float(str2))) = (stack.hashmap.get(&a.value), stack.hashmap.get(&b.value)) {
                        if str2.to_string().contains(&str1.to_string()) {
                            stack.push_as_hashed(b.value);
                            pointer = *p;
                        }
                    }
                }
            }
            JE(p) => {
                if stack.peek().value == 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            JFE(p) => {
                let a = stack.peek();

                if a.hashed {
                    if let Some(ValueType::Float(v)) = stack.hashmap.get(&a.value) {
                        if *v == 0.0_f32 {
                            stack.pop();
                            pointer = *p;
                        }
                    }
                }
            }
            JNE(p) => {
                if stack.peek().value != 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            JFNE(p) => {
                let a = stack.peek();

                if a.hashed {
                    if let Some(ValueType::Float(v)) = stack.hashmap.get(&a.value) {
                        if *v != 0.0_f32 {
                            stack.pop();
                            pointer = *p;
                        }
                    }
                }
            }
            JGT(p) => {
                if stack.peek().value > 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            JFGT(p) => {
                let a = stack.peek();

                if a.hashed {
                    if let Some(ValueType::Float(v)) = stack.hashmap.get(&a.value) {
                        if *v > 0.0_f32 {
                            stack.pop();
                            pointer = *p;
                        }
                    }
                }
            }
            JLT(p) => {
                if stack.peek().value < 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            JFLT(p) => {
                let a = stack.peek();

                if a.hashed {
                    if let Some(ValueType::Float(v)) = stack.hashmap.get(&a.value) {
                        if *v < 0.0_f32 {
                            stack.pop();
                            pointer = *p;
                        }
                    }
                }
            }
            JGE(p) => {
                if stack.peek().value >= 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            JFGE(p) => {
                let a = stack.peek();

                if a.hashed {
                    if let Some(ValueType::Float(v)) = stack.hashmap.get(&a.value) {
                        if *v >= 0.0_f32 {
                            stack.pop();
                            pointer = *p;
                        }
                    }
                }
            }
            JLE(p) => {
                if stack.peek().value <= 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            JFLE(p) => {
                let a = stack.peek();

                if a.hashed {
                    if let Some(ValueType::Float(v)) = stack.hashmap.get(&a.value) {
                        if *v <= 0.0_f32 {
                            stack.pop();
                            pointer = *p;
                        }
                    }
                }
            }
            Get(i) => {
                let a = *stack.get(*i + call_stack.last().map_or(0, |s| s.stack_offset));
                if a.hashed {
                    stack.push_as_hashed(a.value);
                } else if !a.hashed {
                    stack.push_as_value(a.value);
                }
            }
            Set(i) => {
                let a = *i + call_stack.last().map_or(0, |s| s.stack_offset);
                *stack.get_mut(a) = stack.peek();
            }
            GetArg(i) => {
                let a = *stack.get(call_stack.last().unwrap().stack_offset - 1 - *i);

                if a.hashed {
                    stack.push_as_hashed(a.value);
                } else if !a.hashed {
                    stack.push_as_value(a.value);
                }
            }
            SetArg(i) => {
                let offset_i = call_stack.last().unwrap().stack_offset - 1 - *i;
                let new_val = stack.peek();

                *stack.get_mut(offset_i) = new_val;
            }
            Prnt => print!("{}", stack.peek().value),
            PrntStr(d) => println!("{}", d),
            Prntln => println!("{}", stack.peek().value),
            PrntC => print!("{}", stack.peek().value as u8 as char),
            PrntCln => println!("{}", stack.peek().value as u8 as char),
            PrntStk => {
                stack.print();
            }
            PrntReg => {
                stack.print_registers();
            }
            Call(p) => {
                call_stack.push(StackFrame { stack_offset: stack.len(),
                                             ip: pointer });
                pointer = *p;
            }
            Ret => pointer = call_stack.pop().unwrap().ip,
        }
    }
}

fn parse_instruction(s: &[&str], labels: &Labels, procedures: &Procedures) -> Instruction {
    use Instruction::*;

    match s {
        ["pushint" | "pint", x] => PushInt(x.parse::<isize>().unwrap()),
        ["pushfloat" | "pflt", x] => PushFloat(x.parse::<f32>().unwrap()),
        ["pushstr" | "pstr", x] => PushStr(x.parse::<String>().unwrap()),
        ["pop"] => Pop,
        ["dup"] => Dup,
        ["swap"] => Swap,
        ["clsstk"] => ClsStk, // clear table
        ["dlcstk"] => DlcStk, // shrink_to_fit / dealloc table
        ["add"] => Add,       // int
        ["addf"] => AddF,     // float
        ["sub"] => Sub,       // int
        ["subf"] => SubF,     // float
        ["mul"] => Mul,       // int
        ["mulf"] => MulF,     // float
        ["div"] => Div,       // int
        ["divf"] => DivF,     // float
        ["mod"] => Mod,       // int
        ["modf"] => ModF,     // float
        ["incr"] => Incr,
        ["decr"] => Decr,
        ["mov", d, p] => Mov(d.parse::<isize>().unwrap(), p.parse::<isize>().unwrap()),
        ["ld", d] => Ld(d.parse::<isize>().unwrap()),
        ["dmphash", p] => DmpHash(p.parse::<isize>().unwrap()), // remove key
        ["clshash"] => ClsHash,                                 // clear table
        ["dlchash"] => DlcHash,                                 // shrink_to_fit / dealloc table
        ["dmpreg", p] => DmpReg(p.parse::<isize>().unwrap()),   // remove key
        ["clsreg"] => ClsReg,                                   // clear table
        ["dlcreg"] => DlcReg,                                   // shrink_to_fit / dealloc table
        ["jmp", l] => Jmp(*labels.get(l).unwrap()),
        ["cmp", l] => Cmp(*labels.get(l).unwrap()),
        ["inthas", l] => IntHas(*labels.get(l).unwrap()),
        ["strhas", l] => StrHas(*labels.get(l).unwrap()),
        ["flthas", l] => FltHas(*labels.get(l).unwrap()),
        ["je", l] => JE(*labels.get(l).unwrap()),     // int
        ["jfe", l] => JFE(*labels.get(l).unwrap()),   // float
        ["jne", l] => JNE(*labels.get(l).unwrap()),   // int
        ["jfne", l] => JFNE(*labels.get(l).unwrap()), // float
        ["jge", l] => JGE(*labels.get(l).unwrap()),   // int
        ["jfge", l] => JFGE(*labels.get(l).unwrap()), // float
        ["jle", l] => JLE(*labels.get(l).unwrap()),   // int
        ["jfle", l] => JFLE(*labels.get(l).unwrap()), // float
        ["jgt", l] => JGT(*labels.get(l).unwrap()),   // int
        ["jfgt", l] => JFGT(*labels.get(l).unwrap()), // float
        ["jlt", l] => JLT(*labels.get(l).unwrap()),   // int
        ["jflt", l] => JFLT(*labels.get(l).unwrap()), // float
        ["get", p] => Get(p.parse::<Pointer>().unwrap()),
        ["set", p] => Set(p.parse::<Pointer>().unwrap()),
        ["getarg", p] => GetArg(p.parse::<Pointer>().unwrap()),
        ["setarg", p] => SetArg(p.parse::<Pointer>().unwrap()),
        ["prnt"] => Prnt,
        ["prntstr", d] => PrntStr(d.parse::<String>().unwrap()),
        ["prntln"] => Prntln,
        ["prntc"] => PrntC,
        ["prntcln"] => PrntCln,
        ["prntstk"] => PrntStk,
        ["prntreg"] => PrntReg,
        ["proc", proc] => Jmp(procedures.get(proc).unwrap().1),
        ["call", proc] => Call(procedures.get(proc).unwrap().0 + 1),
        ["ret"] => Ret,
        ["label", ..] | ["end"] => Noop,
        l => panic!("Invalid instruction: {:?}", l),
    }
}

fn find_label<'a>(i: Pointer, s: &'a [&'a str]) -> Option<Label> {
    if let ["label", l] = s {
        Some((l, i))
    } else {
        None
    }
}

fn find_procedures<'a>(lines: &'a [Vec<&str>]) -> Procedures<'a> {
    let mut ip = 0;
    let mut res = Procedures::new();

    while ip < lines.len() {
        if let ["proc", proc_name] = lines[ip].as_slice() {
            let start_ip = ip;
            while lines[ip] != ["end"] {
                ip += 1;
            }
            res.insert(proc_name, (start_ip, ip + 1));
        } else {
            ip += 1;
        }
    }

    res
}
