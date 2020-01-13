use crate::code::{Computer, ExecutionResult::*};
use crate::memory::Memory;
use std::cell::Cell;
use std::collections::VecDeque;
use std::env;
use std::rc::Rc;

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
    let values = Rc::new(Cell::new(values));
    //let v = Rc::get_mut(values.get_mut()).unwrap();

    let input = || {
        let v = values.get_mut();
        v.pop_back()
    };

    let output = |x| {
        println!("Output: {}", x);
        let v = values.get_mut();
        v.push_front(x);
    };
    let mut computer = Computer::new(&memory, Box::new(input), Box::new(output));

    while computer.execute() != Exit {}
    Ok(())
}
