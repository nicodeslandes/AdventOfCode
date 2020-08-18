use std::cell::Cell;
use std::env;
use std::fs::File;
use std::io::Read;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");
    let input: i32 = env::args()
        .nth(2)
        .expect("Missing input")
        .parse()
        .expect("Invalid input: enter a number");

    let mut instructions = String::new();
    File::open(file_name)?
        .read_to_string(&mut instructions)
        .expect("Failed to read input file");

    let memory = instructions
        .split(",")
        .map(|x| {
            Cell::new(
                x.parse::<i32>()
                    .expect(format!("Failed to parse {}", x).as_str()),
            )
        })
        .collect::<Vec<_>>();

    //println!("Values: {:?}", memory);
    execute_program(&memory, input);
    Ok(())
}

fn execute_program(memory: &Vec<Cell<i32>>, input: i32) {
    let ip: Cell<usize> = Cell::new(0); // Instruction pointer
    let mut memory = memory.clone();

    loop {
        match read_op_code(&mut memory, &ip) {
            (OpCode::Add, parameter_modes) => execute_instruction3(
                &mut memory,
                &ip,
                parameter_modes,
                |a: Parameter, b: Parameter, c: Parameter| {
                    c.set(a.get() + b.get());
                },
            ),
            (OpCode::Mult, parameter_modes) => {
                execute_instruction3(&mut memory, &ip, parameter_modes, |a, b, c| {
                    c.set(a.get() * b.get());
                })
            }
            (OpCode::Input, parameter_modes) => {
                execute_instruction1(&mut memory, &ip, parameter_modes, |a| {
                    a.set(input);
                })
            }
            (OpCode::Output, parameter_modes) => {
                execute_instruction1(&mut memory, &ip, parameter_modes, |a| {
                    write_output(a.get());
                })
            }
            (OpCode::JumpIfTrue, parameter_modes) => {
                execute_instruction2(&mut memory, &ip, parameter_modes, |a, b| {
                    if a.get() != 0 {
                        jump_to(&ip, b.get());
                    }
                })
            }
            (OpCode::JumpIfFalse, parameter_modes) => {
                execute_instruction2(&mut memory, &ip, parameter_modes, |a, b| {
                    if a.get() == 0 {
                        jump_to(&ip, b.get());
                    }
                })
            }
            (OpCode::LessThan, parameter_modes) => {
                execute_instruction3(&mut memory, &ip, parameter_modes, |a, b, c| {
                    c.set(if a.get() < b.get() { 1 } else { 0 });
                })
            }
            (OpCode::Equals, parameter_modes) => {
                execute_instruction3(&mut memory, &ip, parameter_modes, |a, b, c| {
                    c.set(if a.get() == b.get() { 1 } else { 0 });
                })
            }
            (OpCode::Exit, _) => break,
        }

        //println!("Values: {:?}", memory);
    }
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

fn jump_to(ip: &Cell<usize>, address: i32) {
    ip.set(address as usize);
}

fn read_op_code(memory: &mut Vec<Cell<i32>>, ip: &Cell<usize>) -> (OpCode, u32) {
    let value = memory[ip.get()].get();
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

    ip.set(ip.get() + 1);
    (op_code, parameter_modes)
}

fn execute_instruction1(
    memory: &mut Vec<Cell<i32>>,
    ip: &Cell<usize>,
    parameter_modes: u32,
    operation: impl Fn(Parameter) -> (),
) -> () {
    let mut param_modes = parameter_modes;
    let x = get_parameter(memory, &ip, &mut param_modes);
    operation(x);
}

fn execute_instruction2(
    memory: &mut Vec<Cell<i32>>,
    ip: &Cell<usize>,
    parameter_modes: u32,
    operation: impl Fn(Parameter, Parameter) -> (),
) -> () {
    let mut param_modes = parameter_modes;
    let x = get_parameter(memory, &ip, &mut param_modes);
    let y = get_parameter(memory, &ip, &mut param_modes);
    operation(x, y);
}

fn execute_instruction3(
    memory: &mut Vec<Cell<i32>>,
    ip: &Cell<usize>,
    parameter_modes: u32,
    operation: impl Fn(Parameter, Parameter, Parameter) -> (),
) -> () {
    let mut param_modes = parameter_modes;
    let x = get_parameter(memory, &ip, &mut param_modes);
    let y = get_parameter(memory, &ip, &mut param_modes);
    let z = get_parameter(memory, &ip, &mut param_modes);
    operation(x, y, z);
}

fn get_parameter(
    memory: &Vec<Cell<i32>>,
    ip: &Cell<usize>,
    parameter_modes: &mut u32,
) -> Parameter {
    // Get the parameter mode for this parameter
    let parameter_mode = match *parameter_modes % 10 {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        x => panic!(format!("Incorrect parameter mode: {}", x)),
    };
    *parameter_modes /= 10;

    let parameter_value = memory[ip.get()].get();
    ip.set(ip.get() + 1);
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

fn write_output(value: i32) {
    println!("{}", value);
}
