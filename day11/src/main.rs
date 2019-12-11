use std::cell::Cell;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::ops::Index;
use std::ops::IndexMut;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Clone)]
struct Memory {
    _values: HashMap<usize, Cell<i64>>,
}

impl Memory {
    fn new(values: HashMap<usize, Cell<i64>>) -> Memory {
        Memory { _values: values }
    }
}

impl Index<usize> for Memory {
    type Output = Cell<i64>;

    fn index(&self, index: usize) -> &Self::Output {
        &self._values[&index]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self._values.entry(index).or_insert(Cell::new(0))
    }
}

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

    let mut position = (0, 0);
    let mut panel: HashMap<(i32, i32), i64> = HashMap::new();
    let mut output_mode = OutputMode::Color;
    let painted_panel_count = 0;

    let mut input = || panel.get(&position).map(|x| *x).or(Some(0));
    let mut output = |x| {
        println!("Output: {}", x);
        match output_mode {
            OutputMode::Color => {
                if panel.insert(position, x).is_none() {
                    painted_panel_count += 1;
                }
            }
            _ => panic!(),
        }
    };
    let mut context = ExecutionContext::new(&memory, &mut input, &mut output);
    execute_program(&mut context);

    Ok(())
}

enum OutputMode {
    Color,
    Rotation,
}

struct ExecutionContext<'a> {
    ip: usize,
    memory: Memory,
    //input: Vec<i64>,
    //output: Vec<i64>,
    ended: bool,
    relative_base: usize,
    input: &'a mut dyn FnMut() -> Option<i64>,
    output: &'a mut dyn FnMut(i64) -> (),
}

impl<'a> ExecutionContext<'a> {
    fn new(
        memory: &Memory,
        input: &'a mut dyn FnMut() -> Option<i64>,
        output: &'a mut dyn FnMut(i64) -> (),
    ) -> ExecutionContext<'a> {
        ExecutionContext {
            ip: 0,
            memory: memory.clone(),
            input,
            output,
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
                match (*context.input)() {
                    Some(value) => {
                        // println!("Reading input {}", input_value);
                        execute_instruction1(context, parameter_modes, |a| {
                            a.set(value);
                        });
                    }
                    None => {
                        // println!(
                        //     "Halting program due to input read; ip: {}",
                        //     context.ip.get()
                        // );
                        // Revert the reading of the op-code, so we can read it again when the
                        // thread is resumed
                        context.ip -= 1;
                        return ExecutionResult::MoreInputNeeded;
                    }
                }
            }
            (OpCode::Output, parameter_modes) => {
                let mut output = 0;
                execute_instruction1(context, parameter_modes, |a| {
                    output = a.get();
                });
                //println!("{}", output);
                (*context.output)(output);
            }
            (OpCode::JumpIfTrue, parameter_modes) => {
                let mut jump_address: Option<i64> = None;
                execute_instruction2(context, parameter_modes, |a, b| {
                    if a.get() != 0 {
                        jump_address = Some(b.get());
                    }
                });

                if let Some(address) = jump_address {
                    jump_to(&mut context.ip, address);
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
                    jump_to(&mut context.ip, address);
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
                context.relative_base = (context.relative_base as i64 + adjustment) as usize;
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

fn jump_to(ip: &mut usize, address: i64) {
    *ip = address as usize;
}

fn read_op_code(context: &mut ExecutionContext) -> (OpCode, u32) {
    let value = context.memory[context.ip].get();
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

    context.ip += 1;
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

    let parameter_value = context.memory[context.ip].get();
    context.ip += 1;

    match parameter_mode {
        ParameterMode::Position => {
            Parameter::CellReference(&mut context.memory[parameter_value as usize])
        }
        ParameterMode::Immediate => Parameter::ImmediateValue(parameter_value),
        ParameterMode::Relative => {
            let address = (parameter_value + context.relative_base as i64) as usize;
            Parameter::CellReference(&mut context.memory[address])
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
