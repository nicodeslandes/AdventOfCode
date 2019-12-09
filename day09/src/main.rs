use std::cell::Cell;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
//type Memory = HashMap<usize, Cell<i64>>;

#[derive(Clone)]
struct Memory {
    _values: HashMap<usize, Cell<i64>>,
}

impl Memory {
    fn new(values: HashMap<usize, Cell<i64>>) -> Memory {
        Memory { _values: values }
    }

    fn get(&mut self, address: usize) -> i64 {
        match self._values.get_mut(&address) {
            Some(v) => v.get(),
            None => {
                self._values.insert(address, Cell::new(0));
                0
            }
        }
    }

    fn get_cell(&mut self, address: usize) -> &Cell<i64> {
        match self._values.get_mut(&address) {
            Some(v) => v,
            None => {
                self._values.insert(address, Cell::new(0));
                self._values.get(&address).unwrap()
            }
        }
    }
    fn set(&mut self, address: usize, value: i64) {
        match self._values.get_mut(&address) {
            Some(v) => v.set(value),
            None => {
                self._values.insert(address, Cell::new(value));
            }
        }
    }
}

// impl Index<usize> for Memory {
//     type Output = Cell<i64>;

//     fn index(&self, index: usize) -> &Self::Output {
//         &self._values[&index]
//     }
// }

// impl IndexMut<usize> for Memory {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         let value = self._values.get_mut(&index);
//         match value {
//             Some(v) => v,
//             None => {
//                 self._values.insert(index, Cell::new(0));
//                 let value = self._values.get_mut(&index).unwrap();
//                 &mut value.clone()
//             }
//         }
//     }
// }

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");

    let mut instructions = String::new();
    File::open(file_name)?
        .read_to_string(&mut instructions)
        .expect("Failed to read input file");

    let memory: HashMap<usize, Cell<i64>> = instructions
        .split(",")
        .map(|x| {
            Cell::new(
                x.parse::<i64>()
                    .expect(format!("Failed to parse {}", x).as_str()),
            )
        })
        .enumerate()
        .collect();
    let memory = Memory::new(memory);

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
    relative_base: usize,
}

impl ExecutionContext {
    fn new(memory: &Memory, input: &Vec<i64>) -> ExecutionContext {
        ExecutionContext {
            ip: Cell::new(0),
            memory: memory.clone(),
            input: input.clone(),
            output: vec![],
            ended: false,
            relative_base: 0,
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
        match read_op_code(context) {
            (OpCode::Add, parameter_modes) => execute_instruction3(
                context,
                parameter_modes,
                |a: Parameter, b: Parameter, c: Parameter| {
                    c.set(a.get() + b.get());
                },
            ),
            (OpCode::Mult, parameter_modes) => {
                execute_instruction3(context, parameter_modes, |a, b, c| {
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
                execute_instruction1(context, parameter_modes, |a| {
                    a.set(input_value);
                });
            }
            (OpCode::Output, parameter_modes) => {
                let mut output = 0;
                execute_instruction1(context, parameter_modes, |a| {
                    output = a.get();
                });
                // println!("Outputting {}", output);
                context.output.push(output);
            }
            (OpCode::JumpIfTrue, parameter_modes) => {
                let mut jump_address: Option<i64> = None;
                execute_instruction2(context, parameter_modes, |a, b| {
                    if a.get() != 0 {
                        jump_address = Some(b.get());
                    }
                });

                if let Some(address) = jump_address {
                    jump_to(&context.ip, address);
                }
            }
            (OpCode::JumpIfFalse, parameter_modes) => {
                let mut jump_address: Option<i64> = None;
                execute_instruction2(context, parameter_modes, |a, b| {
                    if a.get() == 0 {
                        jump_address = Some(b.get());
                    }
                });

                if let Some(address) = jump_address {
                    jump_to(&context.ip, address);
                }
            }
            (OpCode::LessThan, parameter_modes) => {
                execute_instruction3(context, parameter_modes, |a, b, c| {
                    c.set(if a.get() < b.get() { 1 } else { 0 });
                })
            }
            (OpCode::Equals, parameter_modes) => {
                execute_instruction3(context, parameter_modes, |a, b, c| {
                    c.set(if a.get() == b.get() { 1 } else { 0 });
                })
            }
            (OpCode::AdjustRelativeBase, parameter_modes) => {
                let mut adjustment: i64 = 0;
                execute_instruction1(context, parameter_modes, |a| {
                    adjustment = a.get();
                });
                context.relative_base = context.relative_base + adjustment as usize;
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
    AdjustRelativeBase,
}

fn jump_to(ip: &Cell<usize>, address: i64) {
    ip.set(address as usize);
}

fn read_op_code(context: &mut ExecutionContext) -> (OpCode, u32) {
    let value = context.memory.get(context.ip.get());
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
        9 => OpCode::AdjustRelativeBase,
        99 => OpCode::Exit,
        x => panic!("Unknown op code: {}", x),
    };

    context.ip.set(context.ip.get() + 1);
    (op_code, parameter_modes)
}

fn execute_instruction1(
    context: &mut ExecutionContext,
    parameter_modes: u32,
    mut operation: impl FnMut(Parameter) -> (),
) -> () {
    let mut param_modes = parameter_modes;
    let x = get_parameter(context, &mut param_modes);
    operation(x);
}

fn execute_instruction2(
    context: &mut ExecutionContext,
    parameter_modes: u32,
    mut operation: impl FnMut(Parameter, Parameter) -> (),
) -> () {
    let mut param_modes = parameter_modes;
    let x = get_parameter(context, &mut param_modes);
    let y = get_parameter(context, &mut param_modes);
    operation(x, y);
}

fn execute_instruction3(
    context: &mut ExecutionContext,
    parameter_modes: u32,
    operation: impl Fn(Parameter, Parameter, Parameter) -> (),
) -> () {
    let mut param_modes = parameter_modes;
    let x = get_parameter(context, &mut param_modes);
    let y = get_parameter(context, &mut param_modes);
    let z = get_parameter(context, &mut param_modes);
    operation(x, y, z);
}

fn get_parameter(context: &mut ExecutionContext, parameter_modes: &mut u32) -> Parameter {
    // Get the parameter mode for this parameter
    let parameter_mode = match *parameter_modes % 10 {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        2 => ParameterMode::Relative,
        x => panic!(format!("Incorrect parameter mode: {}", x)),
    };
    *parameter_modes /= 10;

    let ip = &context.ip;
    let parameter_value = context.memory.get(ip.get());
    ip.set(ip.get() + 1);

    match parameter_mode {
        ParameterMode::Position => {
            Parameter::CellReference(context.memory.get_cell(parameter_value as usize))
        }
        ParameterMode::Immediate => Parameter::ImmediateValue(parameter_value),
        ParameterMode::Relative => {
            let address = (parameter_value + context.relative_base as i64) as usize;
            Parameter::CellReference(context.memory.get_cell(address))
        }
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
    Relative,
}
