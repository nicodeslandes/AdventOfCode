use crate::code::{Computer, ExecutionResult::*};
use crate::memory::Memory;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::env;

#[cfg(unix)]
extern crate ncurses;

mod code;
mod memory;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Pos(i32, i32);

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");

    let memory = Memory::load_from_file(&file_name)?;

    let values: VecDeque<i64> = VecDeque::new();
    let values = RefCell::new(values);

    let input = || {
        //println!("Reading input");
        values.borrow_mut().pop_back()
    };

    let output = |x| {
        println!("Output: {}", x);
        values.borrow_mut().push_front(x);
    };
    let mut computer = Computer::new(&memory, &input, &output);

    while computer.execute() != Exit {}
    Ok(())
}
