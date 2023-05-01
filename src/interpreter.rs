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
                                   hashmap: HashMap::new() };
    let mut pointer: Pointer = 0;
    let mut call_stack = CallStack::new();

    while let Some(instruction) = program.get(pointer) {
        pointer += 1;

        match instruction {
            Noop => {}
            PushInt(d) => {
                stack.push(*d);
            }
            PushStr(d) => stack.push_hashed(d),
            Pop => {
                stack.pop();
            }
            Add => {
                let (a, b) = (stack.pop(), stack.pop());

                if !a.hashed && !b.hashed {
                    stack.push(a.value + b.value)
                }
            }
            Sub => {
                let (a, b) = (stack.pop(), stack.pop());

                if !a.hashed && !b.hashed {
                    stack.push(b.value - a.value)
                }
            }
            Mul => {
                let (a, b) = (stack.pop(), stack.pop());

                if !a.hashed && !b.hashed {
                    stack.push(a.value * b.value)
                }
            }
            Div => {
                let (a, b) = (stack.pop(), stack.pop());

                if !a.hashed && !b.hashed {
                    stack.push(b.value / a.value)
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
                        stack.push(b.value);
                        pointer = *p;
                    }
                }
            }
            Incr => stack.peek_mut().value += 1,
            Decr => stack.peek_mut().value -= 1,
            Jump(p) => pointer = *p,
            Incl(p) => {
                let (a, b) = (stack.pop(), stack.pop());

                if a.hashed && b.hashed {
                    if let (Some(str1), Some(str2)) = (stack.hashmap.get(&a.value), stack.hashmap.get(&b.value)) {
                        if str2.contains(str1) {
                            stack.push_as_hashed(b.value);
                            pointer = *p;
                        }
                    }
                } else if !a.hashed && !b.hashed {
                    let (str1, str2) = (&a.value.to_string(), &b.value.to_string());

                    if str2.contains(str1) {
                        stack.push(b.value);
                        pointer = *p;
                    }
                }
            }
            JE(p) => {
                if stack.peek().value == 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            JNE(p) => {
                if stack.peek().value != 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            JGT(p) => {
                if stack.peek().value > 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            JLT(p) => {
                if stack.peek().value < 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            JGE(p) => {
                if stack.peek().value >= 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            JLE(p) => {
                if stack.peek().value <= 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            Get(i) => {
                let a = *stack.get(*i + call_stack.last().map_or(0, |s| s.stack_offset));
                if a.hashed {
                    stack.push_as_hashed(a.value);
                } else if !a.hashed {
                    stack.push(a.value);
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
                    stack.push(a.value);
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
        ["PushStr", x] => PushStr(x.parse::<String>().unwrap()),
        ["Pop"] => Pop,
        ["Add"] => Add,
        ["Sub"] => Sub,
        ["Mul"] => Mul,
        ["Div"] => Div,
        ["Incr"] => Incr,
        ["Decr"] => Decr,
        ["Jump", l] => Jump(*labels.get(l).unwrap()),
        ["Cmp", l] => Cmp(*labels.get(l).unwrap()),
        ["Incl", l] => Incl(*labels.get(l).unwrap()),
        ["JE", l] => JE(*labels.get(l).unwrap()),
        ["JNE", l] => JNE(*labels.get(l).unwrap()),
        ["JGE", l] => JGE(*labels.get(l).unwrap()),
        ["JLE", l] => JLE(*labels.get(l).unwrap()),
        ["JGT", l] => JGT(*labels.get(l).unwrap()),
        ["JLT", l] => JLT(*labels.get(l).unwrap()),
        ["Get", p] => Get(p.parse::<Pointer>().unwrap()),
        ["Set", p] => Set(p.parse::<Pointer>().unwrap()),
        ["GetArg", p] => GetArg(p.parse::<Pointer>().unwrap()),
        ["SetArg", p] => SetArg(p.parse::<Pointer>().unwrap()),
        ["Print"] => Print,
        ["PrintC"] => PrintC,
        ["PrintStack"] => PrintStack,
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
