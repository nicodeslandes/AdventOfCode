use crate::code::ExecutionResult::Exit;
use crate::code::*;
use crate::memory::Memory;
use crate::switch::Packet;
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

enum InputStatus {
    WaitingForFirstRead,
    Idle,
    ReadingPacket(Packet),
}

enum OutputStatus {
    Idle,
    WritingPacket(i64),
}

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");

    const COMPUTER_COUNT: usize = 50;
    let switch = Rc::new(RefCell::new(Switch::new(COMPUTER_COUNT)));
    let memory = Memory::load_from_file(&file_name)?;

    let mut computers: Vec<Computer> = vec![];
    for i in 0..COMPUTER_COUNT {
        let r1 = switch.clone();
        let r2 = switch.clone();
        let input_status = RefCell::new(InputStatus::WaitingForFirstRead);
        let output_status = RefCell::new(OutputStatus::Idle);

        computers.push(Computer::new(
            i,
            memory.clone(),
            Box::new(move || {
                let mut status = input_status.borrow_mut();
                match *status {
                    InputStatus::WaitingForFirstRead => {
                        *status = InputStatus::Idle;
                        Some(i as i64)
                    }
                    InputStatus::Idle => match r1.borrow().read(i as usize) {
                        Some(packet) => {
                            *status = InputStatus::ReadingPacket(packet);
                            Some(packet.x)
                        }
                        None => None,
                    },
                    InputStatus::ReadingPacket(packet) => {
                        *status = InputStatus::Idle;
                        Some(packet.y)
                    }
                }
            }),
            Box::new(move |addr, data| {
                let mut status = output_status.borrow_mut();
                match *status {
                    OutputStatus::Idle => {
                        *status = OutputStatus::WritingPacket(data);
                    }
                    OutputStatus::WritingPacket(x) => {
                        *status = OutputStatus::Idle;
                        let packet = Packet::new(x, data);
                        r2.borrow().write(addr as usize, packet);
                    }
                }
            }),
        ));
    }

    let mut completed: HashSet<usize> = HashSet::new();
    let mut previous_nat_packet: Option<Packet> = None;

    while completed.len() < COMPUTER_COUNT {
        let switch_activity = switch.borrow().get_activity();
        let switch_was_quiet = switch.borrow().is_quiet();

        // TODO: CHEATING!!!
        // We should detect instead that all computers have been attempting to read without writing anything
        for _ in 0..1000 {
            for computer in computers.iter_mut() {
                if !completed.contains(&computer.id()) {
                    //println!("Computer {} is running...", computer.id());
                    if computer.execute_single_instruction() == Exit {
                        println!("Computer {} has exited", computer.id());
                        completed.insert(computer.id());
                    }
                }
            }
        }

        let switch = switch.borrow();
        if switch_activity == switch.get_activity() && switch_was_quiet && switch.is_quiet() {
            // No activity detected
            // println!("Writing nat packet");
            if let Some(packet) = switch.pop_nat_packet() {
                println!("Writing NAT Packet {}", packet);
                switch.write(0, packet);

                if let Some(p) = previous_nat_packet {
                    if p.y == packet.y {
                        println!("Found it!! Y = {}", p.y);
                        break;
                    }
                }

                previous_nat_packet = Some(packet);
            }
        }
    }
    Ok(())
}
