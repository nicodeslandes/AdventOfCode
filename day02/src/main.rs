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
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    //println!("Values: {:?}", memory);

    for a in 1..100 {
        for b in 1..100 {
            let result = execute_program(&memory, a, b);
            if result == 19690720 {
                println!("Found it! a = {}, b = {}", a, b);
                break;
            }
        }
    }

    Ok(())
}

fn execute_program(memory: &Vec<usize>, arg1: usize, arg2: usize) -> usize {
    let mut ip: usize = 0; // Instruction pointer
    let mut memory = memory.clone();
    // Enter parameters
    memory[1] = arg1;
    memory[2] = arg2;

    loop {
        match read_op_code(&mut memory, &mut ip) {
            OpCode::Add => execute_instruction(&mut memory, &mut ip, |a, b| a + b),
            OpCode::Mult => execute_instruction(&mut memory, &mut ip, |a, b| a * b),
            OpCode::Exit => break,
        }

        //println!("Values: {:?}", memory);
    }

    memory[0]
}

enum OpCode {
    Add,
    Mult,
    Exit,
}

fn read_op_code(memory: &mut Vec<usize>, ip: &mut usize) -> OpCode {
    let op_code = match memory[*ip] {
        1 => OpCode::Add,
        2 => OpCode::Mult,
        99 => OpCode::Exit,
        _ => panic!("Unknown op code!"),
    };

    *ip += 1;
    op_code
}
fn execute_instruction(
    memory: &mut Vec<usize>,
    ip: &mut usize,
    operation: fn(usize, usize) -> usize,
) -> () {
    let x = memory[memory[*ip]];
    *ip += 1;

    let y = memory[memory[*ip]];
    *ip += 1;

    let index = memory[*ip];
    memory[index] = operation(x, y);
    *ip += 1;
}
