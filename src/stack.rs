use crate::instructions::Instruction;
use std::collections::{BTreeMap, HashMap};

pub fn hash(s: String) -> isize {
    let mut hash = 2166136261;

    for b in s.bytes() {
        hash ^= b as u32;
        hash = hash.wrapping_mul(16777619);
    }

    hash as isize
}

pub type Pointer = usize;
pub type Program<'a> = &'a [Instruction];
pub type Label<'a> = (&'a str, Pointer);
pub type Labels<'a> = BTreeMap<&'a str, Pointer>;
pub type Procedures<'a> = BTreeMap<&'a str, (Pointer, Pointer)>;

pub struct StackFrame {
    pub stack_offset: Pointer,
    pub ip: Pointer,
}

pub type CallStack = Vec<StackFrame>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StackValue {
    pub value: isize,
    pub hashed: bool,
}

pub struct Stack {
    pub values: Vec<StackValue>,
    pub hashmap: HashMap<isize, String>,
    pub registers: HashMap<isize, isize>,
}

impl Stack {
    pub fn len(&mut self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&mut self) -> bool {
        self.values.is_empty()
    }

    pub fn print(&mut self) {
        for value in &self.values {
            let hash_str = if value.hashed {
                format!(" --> Hash: ({})", self.hashmap.get(&value.value).unwrap_or(&String::new()))
            } else {
                String::new()
            };

            println!("Stack: {}", value.value.to_string() + &hash_str);
        }
    }

    pub fn print_registers(&mut self) {
        for value in &self.registers {
            println!("Register: {:?}", self.registers.get(value.0))
        }
    }

    pub fn push(&mut self, v: isize) {
        self.values.push(StackValue { value: v, hashed: false });
    }

    pub fn push_as_hashed(&mut self, v: isize) {
        self.values.push(StackValue { value: v, hashed: true });
    }

    pub fn push_hashed(&mut self, v: &str) {
        let h = hash(v.to_string());
        self.values.push(StackValue { value: h, hashed: true });
        self.hashmap.insert(h, v.to_string());
    }

    pub fn push_register(&mut self, r: isize, v: isize) {
        self.registers.insert(r, v);
    }

    pub fn pop(&mut self) -> StackValue {
        self.values.pop().expect("popped an empty stack")
    }

    pub fn peek(&mut self) -> StackValue {
        *self.values.last().expect("peeked an empty stack")
    }

    pub fn peek_mut(&mut self) -> &mut StackValue {
        self.values.last_mut().expect("peeked an empty stack")
    }

    pub fn get(&self, i: usize) -> &StackValue {
        self.values.get(i).expect("accessed a nonexistent stack index")
    }

    pub fn get_mut(&mut self, i: usize) -> &mut StackValue {
        self.values.get_mut(i).expect("mutably accessed a nonexistent stack index")
    }
}
