use crate::{stack::{Stack, TableValue}, interpreter::hash};
use std::collections::HashMap;

pub fn call_extern(p: &str, s: &mut Stack, t: &mut HashMap<isize, TableValue>) {
    match p {
        "prtvfs" => {
            let val = t.get(&s.peek());
            print!("{:?}\n", val)
        }
        "prtvfh" => {
            let val = t.get(&hash(s.peek().to_string()));
            print!("{:?}\n", val)
        }
        &_ => todo!(),
    }
}
