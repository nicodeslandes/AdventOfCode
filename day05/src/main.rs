use std::cell::Cell;
use std::env;
use std::fs::File;
use std::io::Read;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");

    println!("Reading input from {}", file_name);

    let mut input = String::new();
    File::open(file_name)?
        .read_to_string(&mut input)
        .expect("Failed to read input file");

    let memory = input
        .split(",")
        .map(|x| Cell::new(x.parse::<i32>().unwrap()))
        .collect::<Vec<_>>();
    //println!("Values: {:?}", memory);
    let result = execute_program(&memory, 1, 1);
    println!("result: {:?}", result);
    Ok(())
}

fn execute_program(memory: &Vec<Cell<i32>>, arg1: i32, arg2: i32) -> i32 {
    let mut ip: usize = 0; // Instruction pointer
    let mut memory = memory.clone();
    // Enter parameters
    memory[1].set(arg1);
    memory[2].set(arg2);

    loop {
        match read_op_code(&mut memory, &mut ip) {
            OpCode::Add => {
                execute_instruction3(&mut memory, &mut ip, |a, b, c| c.set(a.get() + b.get()))
            }
            OpCode::Mult => {
                execute_instruction3(&mut memory, &mut ip, |a, b, c| c.set(a.get() * b.get()))
            }
            OpCode::Input => execute_instruction1(&mut memory, &mut ip, |a| a.set(read_input())),
            OpCode::Output => execute_instruction1(&mut memory, &mut ip, |a| write_output(a.get())),
            OpCode::Exit => break,
        }

        //println!("Values: {:?}", memory);
    }

    memory[0].get()
}

enum OpCode {
    Add,
    Mult,
    Exit,
    Input,
    Output,
}

fn read_op_code(memory: &mut Vec<Cell<i32>>, ip: &mut usize) -> OpCode {
    let op_code = match memory[*ip].get() {
        1 => OpCode::Add,
        2 => OpCode::Mult,
        3 => OpCode::Input,
        4 => OpCode::Output,
        99 => OpCode::Exit,
        x => panic!("Unknown op code: {}", x),
    };

    *ip += 1;
    op_code
}

fn execute_instruction3(
    memory: &mut Vec<Cell<i32>>,
    ip: &mut usize,
    operation: fn(&Cell<i32>, &Cell<i32>, &Cell<i32>) -> (),
) -> () {
    let x = get_parameter(&memory, ip);
    let y = get_parameter(&memory, ip);
    let z = get_parameter(&memory, ip);

    operation(x, y, z);
}

fn execute_instruction1(
    memory: &mut Vec<Cell<i32>>,
    ip: &mut usize,
    operation: fn(&Cell<i32>) -> (),
) -> () {
    let x = get_parameter(&memory, ip);
    operation(x);
}

fn get_parameter<'a>(memory: &'a Vec<Cell<i32>>, ip: &mut usize) -> &'a Cell<i32> {
    let x_addr = memory[*ip].get() as usize;
    *ip += 1;
    &memory[x_addr]
}

fn read_input() -> i32 {
    1
}

fn write_output(value: i32) {
    print!("{}", value);
}
