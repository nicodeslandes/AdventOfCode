use std::cell::Cell;
use std::env;
use std::fs::File;
use std::io::Read;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");

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

        let mut max_output = 0;

        for i in 0..5 {
            for j in 0..5 {
                if j != i {
                    for k in 0..5 {
                        if k != i && k != j {
                            for l in 0 ..5 {
                                if l != i &&l != j &&l != k {
                                    for m in 0 .. 5 {
                                        if m != i && m != j && m != k && m != l {
                                            let phase_settings = vec!(i, j, k, l, m);
                                            println!("Trying out {:?}", phase_settings);
                                            let output = run_amplifiers(&memory, phase_settings);
                                            if output > max_output {
                                                max_output = output;
                                            }
                                            //run_amplifier(&memory, i, input: i32)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

    println!("Max output: {}", max_output);
    Ok(())
}

fn run_amplifiers(instructions: &Vec<Cell<i32>>, phase_settings: Vec<i32>) -> i32 {
    let mut current_input = 0;
    for phase_setting in phase_settings {
        current_input = run_amplifier(instructions, phase_setting, current_input);
    }

    current_input
}

fn run_amplifier(instructions: &Vec<Cell<i32>>, phase_setting: i32, input: i32) -> i32 {
    // Make a copy of the program
    let memory = instructions.clone();
    let output = execute_program(&memory, vec!(phase_setting, input));
    if output.len() == 0 {
        panic!("No output was produced by the program!");
    }

    if output.len() > 1 {
        println!("Warning! Unexpected output detected: {:?}", output);
    }

    output[0]
}

fn execute_program(memory: &Vec<Cell<i32>>, input: Vec<i32>) -> Vec<i32> {
    let ip: Cell<usize> = Cell::new(0); // Instruction pointer
    let mut memory = memory.clone();
    let mut current_input_index = 0;
    let mut output = vec!();

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
                    if current_input_index >= input.len() {
                        panic!("Attempted to read past input array");
                    }
                    a.set(input[current_input_index]);
                    current_input_index += 1;
                })
            }
            (OpCode::Output, parameter_modes) => {
                execute_instruction1(&mut memory, &ip, parameter_modes, |a| {
                    output.push(a.get());
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

    output
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
    mut operation: impl FnMut(Parameter) -> (),
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
