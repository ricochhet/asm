use crate::instructions::Instruction;
use crate::stack::*;
use std::collections::HashMap;

pub fn compile(buffer: String) {
    let line_splits = buffer.split('\n')
                            .map(|s| s.split_whitespace().collect::<Vec<_>>())
                            .filter(|s| !matches!(s.as_slice(), [] | ["--", ..]))
                            .collect::<Vec<_>>();

    let labels: Labels = line_splits.iter().enumerate().filter_map(|(i, s)| find_label(i, s.as_slice())).collect();
    let procedures: Procedures = find_procedures(line_splits.as_slice());
    let instructions: Vec<Instruction> = line_splits.iter().map(|s| parse_instruction(s.as_slice(), &labels, &procedures)).collect();

    run(&instructions[..]);
}

fn run<'a>(program: Program<'a>) {
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
            Cmp(p) => {
                let (a, b) = (stack.pop(), stack.pop());

                if a.hashed && b.hashed {
                    if b.value == a.value {
                        stack.push_as_hashed(b.value);
                        pointer = *p;
                    }
                } else if !a.hashed && !b.hashed {
                    if b.value == a.value {
                        stack.push_as_value(b.value);
                        pointer = *p;
                    }
                }
            }
            Incr => stack.peek_mut().value += 1,
            Decr => stack.peek_mut().value -= 1,
            Mov(d, p) => {
                let a = *stack.get(*p + call_stack.last().map_or(0, |s| s.stack_offset));
                stack.push_register(*d, a);
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
                stack.delete_hash_value(*p);
            }
            DmpReg(p) => {
                stack.delete_register(*p);
            }
            Jump(p) => pointer = *p,
            InclI(p) => {
                let (a, b) = (stack.pop(), stack.pop());

                if !a.hashed && !b.hashed {
                    let (str1, str2) = (&a.value.to_string(), &b.value.to_string());

                    if str2.contains(str1) {
                        stack.push_as_value(b.value);
                        pointer = *p;
                    }
                }
            }
            InclS(p) => {
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
            InclF(p) => {
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
            Print => print!("{}", stack.peek().value),
            PrintC => print!("{}", stack.peek().value as u8 as char),
            PrintStack => {
                stack.print();
            }
            PrintRegisters => {
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
        ["PushInt", x] => PushInt(x.parse::<isize>().unwrap()),
        ["PushFloat", x] => PushFloat(x.parse::<f32>().unwrap()),
        ["PushStr", x] => PushStr(x.parse::<String>().unwrap()),
        ["Pop"] => Pop,
        ["Add"] => Add,   // int
        ["AddF"] => AddF, // float
        ["Sub"] => Sub,   // int
        ["SubF"] => SubF, // float
        ["Mul"] => Mul,   // int
        ["MulF"] => MulF, // float
        ["Div"] => Div,   // int
        ["DivF"] => DivF, // float
        ["Incr"] => Incr,
        ["Decr"] => Decr,
        ["Mov", d, p] => Mov(d.parse::<isize>().unwrap(), p.parse::<usize>().unwrap()),
        ["Ld", d] => Ld(d.parse::<isize>().unwrap()),
        ["DmpHash", p] => DmpHash(p.parse::<isize>().unwrap()),
        ["DmpReg", p] => DmpReg(p.parse::<isize>().unwrap()),
        ["Jump", l] => Jump(*labels.get(l).unwrap()),
        ["Cmp", l] => Cmp(*labels.get(l).unwrap()),
        ["Incl", l] => InclI(*labels.get(l).unwrap()),
        ["JE", l] => JE(*labels.get(l).unwrap()),     // int
        ["JFE", l] => JFE(*labels.get(l).unwrap()),   // float
        ["JNE", l] => JNE(*labels.get(l).unwrap()),   // int
        ["JFNE", l] => JFNE(*labels.get(l).unwrap()), // float
        ["JGE", l] => JGE(*labels.get(l).unwrap()),   // int
        ["JFGE", l] => JFGE(*labels.get(l).unwrap()), // float
        ["JLE", l] => JLE(*labels.get(l).unwrap()),   // int
        ["JFLE", l] => JFLE(*labels.get(l).unwrap()), // float
        ["JGT", l] => JGT(*labels.get(l).unwrap()),   // int
        ["JFGT", l] => JFGT(*labels.get(l).unwrap()), // float
        ["JLT", l] => JLT(*labels.get(l).unwrap()),   // int
        ["JFLT", l] => JFLT(*labels.get(l).unwrap()), // float
        ["Get", p] => Get(p.parse::<Pointer>().unwrap()),
        ["Set", p] => Set(p.parse::<Pointer>().unwrap()),
        ["GetArg", p] => GetArg(p.parse::<Pointer>().unwrap()),
        ["SetArg", p] => SetArg(p.parse::<Pointer>().unwrap()),
        ["Print"] => Print,
        ["PrintC"] => PrintC,
        ["PrintStack"] => PrintStack,
        ["PrintRegisters"] => PrintRegisters,
        ["Proc", proc] => Jump(procedures.get(proc).unwrap().1),
        ["Call", proc] => Call(procedures.get(proc).unwrap().0 + 1),
        ["Ret"] => Ret,
        ["label", ..] | ["End"] => Noop,
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
        if let ["Proc", proc_name] = lines[ip].as_slice() {
            let start_ip = ip;
            while lines[ip] != &["End"] {
                ip += 1;
            }
            res.insert(proc_name, (start_ip, ip + 1));
        } else {
            ip += 1;
        }
    }

    res
}
