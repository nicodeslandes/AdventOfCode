use std::cell::Cell;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
type Memory = HashMap<usize, Cell<i64>>;

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");

    let mut instructions = String::new();
    File::open(file_name)?
        .read_to_string(&mut instructions)
        .expect("Failed to read input file");

    let memory: Memory = instructions
        .split(",")
        .map(|x| {
            Cell::new(
                x.parse::<i64>()
                    .expect(format!("Failed to parse {}", x).as_str()),
            )
        })
        .enumerate()
        .collect();

    let mut context = ExecutionContext::new(&memory, &vec![1]);
    execute_program(&mut context);

    Ok(())
}

// fn run_amplifiers(instructions: &Memory, phase_settings: Vec<i64>) -> i64 {
//     let mut current_input = 0;
//     let mut contexts: Vec<_> = phase_settings
//         .iter()
//         .map(|s| ExecutionContext::new(instructions, &vec![*s]))
//         .collect();

//     // Initialize the 1st amplifier input
//     while contexts.iter().any(|c| !c.ended) {
//         for i in 0..contexts.len() {
//             if contexts[i].ended {
//                 continue;
//             }
//             // println!("Running amplifier {}", i);
//             // println!("===================",);
//             let context = &mut contexts[i];
//             // println!(
//             //     "Memory:\n{:?}",
//             //     context.memory.iter().map(|x| x.get()).collect::<Vec<_>>()
//             // );

//             let result = run_amplifier(context, current_input);
//             // println!(
//             //     "Memory:\n{:?}",
//             //     context.memory.iter().map(|x| x.get()).collect::<Vec<_>>()
//             // );

//             current_input = context.output.remove(0);
//             // println!("Extracting output: {}", current_input);

//             match result {
//                 ExecutionResult::MoreInputNeeded => {}
//                 ExecutionResult::Exit => {
//                     // println!("Amplifier {} ended", i);
//                     break;
//                 }
//             }
//         }
//     }

//     current_input
// }

// fn run_amplifier(context: &mut ExecutionContext, input: i64) -> ExecutionResult {
//     context.input.push(input);
//     execute_program(context)
// }

struct ExecutionContext {
    ip: Cell<usize>,
    memory: Memory,
    input: Vec<i64>,
    output: Vec<i64>,
    ended: bool,
}

impl ExecutionContext {
    fn new(memory: &Memory, input: &Vec<i64>) -> ExecutionContext {
        ExecutionContext {
            ip: Cell::new(0),
            memory: memory.clone(),
            input: input.clone(),
            output: vec![],
            ended: false,
        }
    }
}

enum ExecutionResult {
    MoreInputNeeded,
    Exit,
}

fn execute_program(context: &mut ExecutionContext) -> ExecutionResult {
    // println!("Executing program; ip: {}", context.ip.get());
    loop {
        match read_op_code(&context.memory, &context.ip) {
            (OpCode::Add, parameter_modes) => execute_instruction3(
                &context.memory,
                &context.ip,
                parameter_modes,
                |a: Parameter, b: Parameter, c: Parameter| {
                    c.set(a.get() + b.get());
                },
            ),
            (OpCode::Mult, parameter_modes) => {
                execute_instruction3(&context.memory, &context.ip, parameter_modes, |a, b, c| {
                    c.set(a.get() * b.get());
                })
            }
            (OpCode::Input, parameter_modes) => {
                if context.input.is_empty() {
                    // println!(
                    //     "Halting program due to input read; ip: {}",
                    //     context.ip.get()
                    // );
                    // Revert the reading of the op-code, so we can read it again when the
                    // thread is resumed
                    context.ip.set(context.ip.get() - 1);
                    return ExecutionResult::MoreInputNeeded;
                }

                let input_value = context.input.remove(0);
                // println!("Reading input {}", input_value);
                execute_instruction1(&context.memory, &context.ip, parameter_modes, |a| {
                    a.set(input_value);
                });
            }
            (OpCode::Output, parameter_modes) => {
                let mut output = 0;
                execute_instruction1(&context.memory, &context.ip, parameter_modes, |a| {
                    output = a.get();
                });
                // println!("Outputting {}", output);
                context.output.push(output);
            }
            (OpCode::JumpIfTrue, parameter_modes) => {
                execute_instruction2(&context.memory, &context.ip, parameter_modes, |a, b| {
                    if a.get() != 0 {
                        jump_to(&context.ip, b.get());
                    }
                })
            }
            (OpCode::JumpIfFalse, parameter_modes) => {
                execute_instruction2(&context.memory, &context.ip, parameter_modes, |a, b| {
                    if a.get() == 0 {
                        jump_to(&context.ip, b.get());
                    }
                })
            }
            (OpCode::LessThan, parameter_modes) => {
                execute_instruction3(&context.memory, &context.ip, parameter_modes, |a, b, c| {
                    c.set(if a.get() < b.get() { 1 } else { 0 });
                })
            }
            (OpCode::Equals, parameter_modes) => {
                execute_instruction3(&context.memory, &context.ip, parameter_modes, |a, b, c| {
                    c.set(if a.get() == b.get() { 1 } else { 0 });
                })
            }
            (OpCode::Exit, _) => {
                context.ended = true;
                return ExecutionResult::Exit;
            }
        }

        // println!("Values: {:?}", memory);
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

fn jump_to(ip: &Cell<usize>, address: i64) {
    ip.set(address as usize);
}

fn read_op_code(memory: &Memory, ip: &Cell<usize>) -> (OpCode, u32) {
    let value = memory[&ip.get()].get();
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
    memory: &Memory,
    ip: &Cell<usize>,
    parameter_modes: u32,
    mut operation: impl FnMut(Parameter) -> (),
) -> () {
    let mut param_modes = parameter_modes;
    let x = get_parameter(memory, &ip, &mut param_modes);
    operation(x);
}

fn execute_instruction2(
    memory: &Memory,
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
    memory: &Memory,
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

fn get_parameter(memory: &Memory, ip: &Cell<usize>, parameter_modes: &mut u32) -> Parameter {
    // Get the parameter mode for this parameter
    let parameter_mode = match *parameter_modes % 10 {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        x => panic!(format!("Incorrect parameter mode: {}", x)),
    };
    *parameter_modes /= 10;

    let parameter_value = memory[&ip.get()].get();
    ip.set(ip.get() + 1);
    match parameter_mode {
        ParameterMode::Position => Parameter::CellReference(&memory[&(parameter_value as usize)]),
        ParameterMode::Immediate => Parameter::ImmediateValue(parameter_value),
    }
}

enum Parameter {
    ImmediateValue(i64),
    CellReference(*const Cell<i64>),
}

impl<'a> Parameter {
    fn get(&self) -> i64 {
        match self {
            Parameter::CellReference(cell) => unsafe { cell.as_ref().unwrap().get() },
            Parameter::ImmediateValue(value) => *value,
        }
    }

    fn set(&self, value: i64) -> () {
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
