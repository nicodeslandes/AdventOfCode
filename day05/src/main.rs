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
            (OpCode::Add, parameter_modes) => {
                execute_instruction3(&mut memory, &mut ip, parameter_modes, |a, b, c| {
                    c.set(a.get() + b.get())
                })
            }
            (OpCode::Mult, parameter_modes) => {
                execute_instruction3(&mut memory, &mut ip, parameter_modes, |a, b, c| {
                    c.set(a.get() * b.get())
                })
            }
            (OpCode::Input, parameter_modes) => {
                execute_instruction1(&mut memory, &mut ip, parameter_modes, |a| {
                    a.set(read_input())
                })
            }
            (OpCode::Output, parameter_modes) => {
                execute_instruction1(&mut memory, &mut ip, parameter_modes, |a| {
                    write_output(a.get())
                })
            }
            (OpCode::Exit, _) => break,
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

fn read_op_code(memory: &mut Vec<Cell<i32>>, ip: &mut usize) -> (OpCode, u32) {
    let value = memory[*ip].get();
    let op_code_value = value % 100;
    let parameter_modes = (value / 100) as u32;

    let op_code = match op_code_value {
        1 => OpCode::Add,
        2 => OpCode::Mult,
        3 => OpCode::Input,
        4 => OpCode::Output,
        99 => OpCode::Exit,
        x => panic!("Unknown op code: {}", x),
    };

    *ip += 1;
    (op_code, parameter_modes)
}

fn execute_instruction3<'a>(
    memory: &'a mut Vec<Cell<i32>>,
    ip: &mut usize,
    parameter_modes: u32,
    operation: fn(Parameter<'a>, Parameter<'a>, Parameter<'a>) -> (),
) -> () {
    let mut param_modes = parameter_modes;
    let x = get_parameter(&memory, ip, &mut param_modes);
    let y = get_parameter(&memory, ip, &mut param_modes);
    let z = get_parameter(&memory, ip, &mut param_modes);

    operation(x, y, z);
}

fn execute_instruction1<'a>(
    memory: &'a mut Vec<Cell<i32>>,
    ip: &mut usize,
    parameter_modes: u32,
    operation: fn(Parameter<'a>) -> (),
) -> () {
    let mut param_modes = parameter_modes;
    let x = get_parameter(&memory, ip, &mut param_modes);
    operation(x);
}

fn get_parameter<'a>(
    memory: &'a Vec<Cell<i32>>,
    ip: &mut usize,
    parameter_modes: &mut u32,
) -> Parameter<'a> {
    // Get the parameter mode for this parameter
    let parameter_mode = match *parameter_modes % 10 {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        x => panic!(format!("Incorrect parameter mode: {}", x)),
    };
    *parameter_modes /= 10;

    let parameter_value = memory[*ip].get();
    *ip += 1;
    match parameter_mode {
        ParameterMode::Position => Parameter::CellReference(&memory[parameter_value as usize]),
        ParameterMode::Immediate => Parameter::ImmediateValue(parameter_value),
    }
}

enum Parameter<'a> {
    ImmediateValue(i32),
    CellReference(&'a Cell<i32>),
}

impl<'a> Parameter<'a> {
    fn get(&self) -> i32 {
        match self {
            Parameter::CellReference(cell) => cell.get(),
            Parameter::ImmediateValue(value) => *value,
        }
    }

    fn set(&self, value: i32) -> () {
        match self {
            Parameter::CellReference(cell) => cell.set(value),
            Parameter::ImmediateValue(value) => panic!(format!(
                "Attempted to write value {} to an immediate parameter",
                value
            )),
        }
    }
}

enum ParameterMode {
    Position,
    Immediate,
}

fn read_input() -> i32 {
    1
}

fn write_output(value: i32) {
    print!("{}", value);
}
