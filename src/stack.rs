use crate::instructions::Instruction;
use std::collections::BTreeMap;

pub type Pointer = usize;
pub type Program<'a> = &'a [Instruction];
pub type Label<'a> = (&'a str, Pointer);
pub type Labels<'a> = BTreeMap<&'a str, Pointer>;
pub type Procedures<'a> = BTreeMap<&'a str, (Pointer, Pointer)>;

#[derive(Debug)]
pub enum TableValue {
    Integer(isize),
    String(String),
}

pub struct StackFrame {
    pub stack_offset: Pointer,
    pub ip: Pointer,
}

pub type CallStack = Vec<StackFrame>;
pub struct Stack(pub Vec<isize>);

impl Stack {
    pub fn push(&mut self, v: isize) {
        self.0.push(v);
    }

    pub fn pop(&mut self) -> isize {
        self.0.pop().expect("popped an empty stack")
    }

    pub fn peek(&mut self) -> isize {
        *self.0.last().expect("peeked an empty stack")
    }

    pub fn peek_mut(&mut self) -> &mut isize {
        self.0.last_mut().expect("peeked an empty stack")
    }

    pub fn get(&self, i: usize) -> &isize {
        self.0.get(i).expect("accessed a nonexistent stack index")
    }

    pub fn get_mut(&mut self, i: usize) -> &mut isize {
        self.0.get_mut(i).expect("mutably accessed a nonexistent stack index")
    }
}
