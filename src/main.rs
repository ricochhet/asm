use std::collections::{BTreeMap, HashMap};
use std::io::Read;

fn hash(s: String) -> isize {
    let mut hash = 2166136261;

    for b in s.bytes() {
        hash ^= b as u32;
        hash = hash.wrapping_mul(16777619);
    }

    hash as isize
}

type Pointer = usize;
type Program<'a> = &'a [Instruction];
type Label<'a> = (&'a str, Pointer);
type Labels<'a> = BTreeMap<&'a str, Pointer>;
type Procedures<'a> = BTreeMap<&'a str, (Pointer, Pointer)>;

struct StackFrame {
    pub stack_offset: Pointer,
    pub ip: Pointer,
}

type CallStack = Vec<StackFrame>;

#[derive(Debug)]
enum TableValue {
    Integer(isize),
    String(String)
}

struct Stack(Vec<isize>);

impl Stack {
    fn push(&mut self, v: isize) {
        self.0.push(v);
    }

    fn pop(&mut self) -> isize {
        self.0.pop().expect("popped an empty stack")
    }

    fn peek(&mut self) -> isize {
        *self.0.last().expect("peeked an empty stack")
    }

    fn peek_mut(&mut self) -> &mut isize {
        self.0.last_mut().expect("peeked an empty stack")
    }

    fn get(&self, i: usize) -> &isize {
        self.0.get(i).expect("accessed a nonexistent stack index")
    }

    fn get_mut(&mut self, i: usize) -> &mut isize {
        self.0
            .get_mut(i)
            .expect("mutably accessed a nonexistent stack index")
    }
}

#[derive(Debug)]
enum Instruction {
    PushInt(isize),
    PushStr(String),
    Pop,
    Add,
    Sub,
    Incr,
    Decr,
    Mul,
    Div,
    Cmp(Pointer),
    Strhas(Pointer),
    Jump(Pointer),
    JE(Pointer),
    JNE(Pointer),
    JGT(Pointer),
    JLT(Pointer),
    JGE(Pointer),
    JLE(Pointer),
    Get(Pointer),
    Set(Pointer),
    GetArg(Pointer),
    SetArg(Pointer),
    Noop,
    Print,
    PrintC,
    PrintStack,
    Call(Pointer),
    Extern(String),
    Ret,
}

fn interpret<'a>(program: Program<'a>) {
    use Instruction::*;

    let mut stack: Stack = Stack(Vec::new());
    let mut pointer: Pointer = 0;
    let mut call_stack = CallStack::new();
    let mut table: HashMap<isize, TableValue> = HashMap::new();

    while let Some(instruction) = program.get(pointer) {
        pointer += 1;

        match instruction {
            Noop => {}
            PushInt(d) => {
                let h = hash(d.to_string());
                table.insert(h, TableValue::Integer(*d));
                stack.push(*d)
            },
            PushStr(d) => {
                let h = hash(d.to_string());
                table.insert(h, TableValue::String(d.clone()));
                stack.push(h);
            },
            Pop => {
                stack.pop();
            }
            Add => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(a + b)
            }
            Sub => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(b - a)
            }
            Mul => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(a * b) 
            }
            Div => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(b / a)
            }
            Cmp(p) => {
                let (a, b) = (stack.pop(), stack.pop());

                if b == a {
                    stack.push(b);
                    pointer = *p;
                }
            },
            Incr => *stack.peek_mut() += 1,
            Decr => *stack.peek_mut() -= 1,
            Jump(p) => pointer = *p,
            Strhas(p) => {
                let (a, b) = (stack.pop(), stack.pop());
                let (value1, value2) = (table.get(&a), table.get(&b));

                if let (Some(TableValue::String(str1)), Some(TableValue::String(str2))) = (value1, value2) {
                    if str2.contains(str1) {
                        stack.push(b);
                        pointer = *p;
                    }
                }                
            },
            JE(p) => {
                if stack.peek() == 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            JNE(p) => {
                if stack.peek() != 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            JGT(p) => {
                if stack.peek() > 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            JLT(p) => {
                if stack.peek() < 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            JGE(p) => {
                if stack.peek() >= 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            JLE(p) => {
                if stack.peek() <= 0 {
                    stack.pop();
                    pointer = *p;
                }
            }
            Get(i) => stack.push(*stack.get(*i + call_stack.last().map_or(0, |s| s.stack_offset))),
            Set(i) => {
                *stack
                    .0
                    .get_mut(*i + call_stack.last().map_or(0, |s| s.stack_offset))
                    .unwrap() = stack.peek()
            }
            GetArg(i) => stack.push(
                *stack
                    .0
                    .get(call_stack.last().unwrap().stack_offset - 1 - *i)
                    .unwrap(),
            ),
            SetArg(i) => {
                let offset_i = call_stack.last().unwrap().stack_offset - 1 - *i;
                let new_val = stack.peek();
                *stack.get_mut(offset_i) = new_val;
            }
            Print => print!("{}", stack.peek()),
            PrintC => print!("{}", stack.peek() as u8 as char),
            PrintStack => println!("{:?}", stack.0),
            Call(p) => {
                call_stack.push(StackFrame {
                    stack_offset: stack.0.len(),
                    ip: pointer,
                });
                pointer = *p;
            },
            Extern(d) => {
                // call_extern(d)
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
        ["Strhas", l] => Strhas(*labels.get(l).unwrap()),
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

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut f = std::fs::File::open(&args[1])?;

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let line_splits = buffer
        .split('\n')
        .map(|s| s.split_whitespace().collect::<Vec<_>>())
        .filter(|s| !matches!(s.as_slice(), [] | ["--", ..]))
        .collect::<Vec<_>>();

    let labels: Labels = line_splits
        .iter()
        .enumerate()
        .filter_map(|(i, s)| find_label(i, s.as_slice()))
        .collect();

    let procedures: Procedures = find_procedures(line_splits.as_slice());

    let instructions: Vec<Instruction> = line_splits
        .iter()
        .map(|s| parse_instruction(s.as_slice(), &labels, &procedures))
        .collect();

    interpret(&instructions[..]);

    Ok(())
}