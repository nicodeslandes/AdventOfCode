use crate::code::ExecutionResult::Exit;
use crate::code::*;
use crate::memory::Memory;
use crate::switch::Switch;
use std::cell::RefCell;
use std::collections::HashSet;
use std::env;
use std::rc::Rc;

#[cfg(unix)]
extern crate ncurses;

mod code;
mod memory;
mod switch;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Pos(i32, i32);

// struct NetworkedComputer<'a> {
//     address: i64,
//     computer: Computer<'a>
// }

// impl<'a> NetworkedComputer<'a> {
//     pub fn new(address: i64, memory: &Memory) -> NetworkedComputer<'a> {
//         let values = RefCell::new(VecDeque::from(vec![address]));

//         let input = move || {
//             //println!("Reading input");
//             values.borrow_mut().pop_back()
//         };
//         let output = move |x| {
//             println!("Output: {}", x);
//             values.borrow_mut().push_front(x);
//         };

//         NetworkedComputer {
//             address,
//             computer: Computer::new(memory, &input, &output)
//         }
//     }

//     fn read_input() -> i64 {
//         0
//     }

//     fn write_input(x: i64) -> () {}
// }

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");

    const COMPUTER_COUNT: usize = 50;
    let switch = Rc::new(RefCell::new(Switch::new(COMPUTER_COUNT)));
    let memory = Memory::load_from_file(&file_name)?;

    let mut computers: Vec<Computer> = vec![];
    for i in 0..COMPUTER_COUNT {
        let r1 = switch.clone();
        let r2 = switch.clone();
        switch.borrow().write(i, i as i64);
        computers.push(Computer::new(
            i,
            &memory,
            Box::new(move || {
                let s = r1.borrow();
                s.read(i as usize)
            }),
            Box::new(move |addr, data| {
                let s = r2.borrow();
                s.write(addr as usize, data);
            }),
        ));
    }

    let mut completed: HashSet<usize> = HashSet::new();
    while completed.len() < COMPUTER_COUNT {
        for computer in computers.iter_mut() {
            if !completed.contains(&computer.id()) {
                println!("Computer {} is running...", computer.id());
                if computer.execute() == Exit {
                    println!("Computer {} has exited", computer.id());
                    completed.insert(computer.id());
                }
            }
        }
    }
    Ok(())
}
