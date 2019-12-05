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
        .map(|x| {
            Cell::new(
                x.parse::<i32>()
                    .expect(format!("Failed to parse {}", x).as_str()),
            )
        })
        .collect::<Vec<_>>();
    //println!("Values: {:?}", memory);
    let result = execute_program(&memory);
    println!("result: {:?}", result);
    Ok(())
}

fn execute_program(memory: &Vec<Cell<i32>>) -> i32 {
    let mut ip: usize = 0; // Instruction pointer
    let mut memory = memory.clone();

    loop {
        match read_op_code(&mut memory, &mut ip) {
            (OpCode::Add, parameter_modes) => {
                execute_instruction3(&mut memory, &mut ip, parameter_modes, |a, b, c| {
                    c.set(a.get() + b.get());
                    None
                })
            }
            (OpCode::Mult, parameter_modes) => {
                execute_instruction3(&mut memory, &mut ip, parameter_modes, |a, b, c| {
                    c.set(a.get() * b.get());
                    None
                })
            }
            (OpCode::Input, parameter_modes) => {
                execute_instruction1(&mut memory, &mut ip, parameter_modes, |a| {
                    a.set(read_input());
                    None
                })
            }
            (OpCode::Output, parameter_modes) => {
                execute_instruction1(&mut memory, &mut ip, parameter_modes, |a| {
                    write_output(a.get());
                    None
                })
            }
            (OpCode::JumpIfTrue, parameter_modes) => {
                execute_instruction2(&mut memory, &mut ip, parameter_modes, |a, b| {
                    if a.get() != 0 {
                        Some(b.get() as usize)
                    } else {
                        None
                    }
                })
            }
            (OpCode::JumpIfFalse, parameter_modes) => {
                execute_instruction2(&mut memory, &mut ip, parameter_modes, |a, b| {
                    if a.get() == 0 {
                        Some(b.get() as usize)
                    } else {
                        None
                    }
                })
            }
            (OpCode::LessThan, parameter_modes) => {
                execute_instruction3(&mut memory, &mut ip, parameter_modes, |a, b, c| {
                    c.set(if a.get() < b.get() { 1 } else { 0 });
                    None
                })
            }
            (OpCode::Equals, parameter_modes) => {
                execute_instruction3(&mut memory, &mut ip, parameter_modes, |a, b, c| {
                    c.set(if a.get() == b.get() { 1 } else { 0 });
                    None
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
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
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
        5 => OpCode::JumpIfTrue,
        6 => OpCode::JumpIfFalse,
        7 => OpCode::LessThan,
        8 => OpCode::Equals,
        99 => OpCode::Exit,
        x => panic!("Unknown op code: {}", x),
    };

    *ip += 1;
    (op_code, parameter_modes)
}

fn execute_instruction1(
    memory: &mut Vec<Cell<i32>>,
    ip: &mut usize,
    parameter_modes: u32,
    operation: fn(Parameter) -> Option<usize>,
) -> () {
    let mut param_modes = parameter_modes;
    let x = get_parameter(memory, ip, &mut param_modes);
    operation(x).map(|ptr| *ip = ptr);
}

fn execute_instruction2(
    memory: &mut Vec<Cell<i32>>,
    ip: &mut usize,
    parameter_modes: u32,
    operation: fn(Parameter, Parameter) -> Option<usize>,
) -> () {
    let mut param_modes = parameter_modes;
    let x = get_parameter(memory, ip, &mut param_modes);
    let y = get_parameter(memory, ip, &mut param_modes);
    operation(x, y).map(|ptr| *ip = ptr);
}

fn execute_instruction3(
    memory: &mut Vec<Cell<i32>>,
    ip: &mut usize,
    parameter_modes: u32,
    operation: fn(Parameter, Parameter, Parameter) -> Option<usize>,
) -> () {
    let mut param_modes = parameter_modes;
    let x = get_parameter(memory, ip, &mut param_modes);
    let y = get_parameter(memory, ip, &mut param_modes);
    let z = get_parameter(memory, ip, &mut param_modes);
    operation(x, y, z).map(|ptr| *ip = ptr);
}

fn get_parameter(memory: &Vec<Cell<i32>>, ip: &mut usize, parameter_modes: &mut u32) -> Parameter {
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

enum Parameter {
    ImmediateValue(i32),
    CellReference(*const Cell<i32>),
}

impl<'a> Parameter {
    fn get(&self) -> i32 {
        match self {
            Parameter::CellReference(cell) => unsafe { cell.as_ref().unwrap().get() },
            Parameter::ImmediateValue(value) => *value,
        }
    }

    fn set(&self, value: i32) -> () {
        match self {
            Parameter::CellReference(cell) => unsafe { cell.as_ref().unwrap().set(value) },
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
    5
}

fn write_output(value: i32) {
    println!("{}", value);
}
