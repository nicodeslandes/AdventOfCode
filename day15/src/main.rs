use crate::memory::Memory;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

mod memory;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");

    let mut instructions = String::new();
    File::open(file_name)?
        .read_to_string(&mut instructions)
        .expect("Failed to read input file");

    let memory = Memory::parse(&instructions);

    let mut context = ExecutionContext::new(&memory);
    context.grid.insert((0, 0), CellStatus::Origin);

    let mut next_move = Move::North;
    let mut current_position: (i32, i32) = (0, 0);
    let mut loop_count = 0;

    loop {
        context.next_input = Some(match next_move {
            Move::North => 1,
            Move::South => 2,
            Move::West => 3,
            Move::East => 4,
        });
        let execution_result = execute_program(&mut context);
        //println!("Result: {:?}", context.result);
        let target_position = apply_move(current_position, next_move);

        let found_new_cell = !context.grid.contains_key(&target_position);
        let should_update_cell_status = |ctx: &ExecutionContext, position: (i32, i32)| {
            match get_cell_status(ctx, position) {
                CellStatus::Origin | CellStatus::Wall | CellStatus::VisitedAll => None,
                _ => {
                    // How many visited-all or Wall positions do we have around the position
                    let dead_end_count = get_positions_around(position)
                        .iter()
                        .filter(|pos| {
                            let status = get_cell_status(&ctx, **pos);
                            //println!("Status for {:?}: {:?}", pos, status);
                            status == CellStatus::Origin
                                || status == CellStatus::VisitedAll
                                || status == CellStatus::Wall
                                || status == CellStatus::Oxygen
                        })
                        .count();

                    //println!("Dead-ends for {:?}: {}", position, dead_end_count);
                    //draw_grid(&ctx.grid, Some(current_position));
                    if get_cell_status(ctx, position) != CellStatus::Origin {
                        let new_status = if dead_end_count >= 3 {
                            CellStatus::VisitedAll
                        } else {
                            CellStatus::Visited
                        };
                        Some(new_status)
                    } else {
                        None
                    }
                }
            }
        };

        match context.result {
            MoveResult::Moved => {
                if let Some(status) = should_update_cell_status(&context, target_position) {
                    context.grid.insert(target_position, status);
                }
                current_position = target_position;
            }
            MoveResult::HitWall => {
                context.grid.insert(target_position, CellStatus::Wall);
                if let Some(status) = should_update_cell_status(&context, current_position) {
                    context.grid.insert(current_position, status);
                }
            }
            MoveResult::FoundOxygen => {
                context.grid.insert(target_position, CellStatus::Oxygen);
                current_position = target_position;
                draw_grid(&context.grid, None);
                display_oxygen_location(&context, current_position);
                break;
            }
        };

        let search_for_next_move = || {
            let unknown_neighbor_move = get_all_moves().into_iter().find(|m| {
                let pos = apply_move(current_position, *m);
                get_cell_status(&context, pos) == CellStatus::Unknown
            });

            // Find an unknown neighbour first
            if unknown_neighbor_move.is_some() {
                return unknown_neighbor_move;
            }

            // Find a neighbour that is not in a final state
            let non_final_neighbor_move = get_all_moves().into_iter().find(|m| {
                let pos = apply_move(current_position, *m);
                // println!(
                //     "Trying out moving {:?} to {:?}: {:?}",
                //     next_move,
                //     new_pos,
                //     get_cell_status(&context, new_pos)
                // );
                match get_cell_status(&context, pos) {
                    CellStatus::Origin | CellStatus::Wall | CellStatus::VisitedAll => false,
                    _ => true,
                }
            });
            non_final_neighbor_move
        };

        let next_move_search_result = search_for_next_move();
        match next_move_search_result {
            Some(m) => next_move = m,
            _ => break,
        }

        loop_count += 1;
        if found_new_cell || loop_count % 1_000_000 == 0 {
            //draw_grid(&context.grid, Some(current_position));
        }

        if let ExecutionResult::Exit = execution_result {
            break;
        }
    }

    Ok(())
}

fn get_all_moves() -> Vec<Move> {
    vec![Move::North, Move::West, Move::South, Move::East]
}

fn display_oxygen_location(context: &ExecutionContext, position: (i32, i32)) {
    let visited_count = context
        .grid
        .values()
        .filter(|s| **s == CellStatus::Visited)
        .count();
    println!(
        "Required movements: {}; current position: {:?}; state: {:?}",
        visited_count + 1,
        position,
        get_cell_status(&context, position)
    );
}

fn get_cell_status(context: &ExecutionContext, position: (i32, i32)) -> CellStatus {
    *context.grid.get(&position).unwrap_or(&CellStatus::Unknown)
}

fn apply_move(position: (i32, i32), m: Move) -> (i32, i32) {
    let (x, y) = position;
    match m {
        Move::North => (x, y + 1),
        Move::South => (x, y - 1),
        Move::West => (x - 1, y),
        Move::East => (x + 1, y),
    }
}
fn get_positions_around(position: (i32, i32)) -> Vec<(i32, i32)> {
    let (x, y) = position;
    vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
}

fn draw_grid(grid: &HashMap<(i32, i32), CellStatus>, current: Option<(i32, i32)>) {
    let x_min = *grid.keys().map(|(x, _)| x).min().unwrap();
    let x_max = *grid.keys().map(|(x, _)| x).max().unwrap();
    let y_min = *grid.keys().map(|(_, y)| y).min().unwrap();
    let y_max = *grid.keys().map(|(_, y)| y).max().unwrap();
    //println!("Panel size: {}x{}", x_max, y_max);
    for y in y_min..y_max + 1 {
        for x in x_min..x_max + 1 {
            let reverse_y = y_max + (y_min - y);
            if let Some(c) = current {
                if (x, reverse_y) == c {
                    print!("X");
                    continue;
                }
            }
            let status = grid.get(&(x, reverse_y)).unwrap_or(&CellStatus::Unknown);
            let c = match status {
                CellStatus::Origin => 'O',
                CellStatus::Unknown => '?',
                CellStatus::Wall => '█',
                CellStatus::Visited => ' ',
                CellStatus::VisitedAll => '░',
                CellStatus::Oxygen => 'O',
            };
            print!("{}", c);
        }
        println!();
    }

    println!();
}

#[derive(Clone, Copy, Debug)]
enum Move {
    North,
    South,
    West,
    East,
}

#[derive(Clone)]
struct ExecutionContext {
    ip: usize,
    memory: Memory,
    ended: bool,
    relative_base: usize,
    grid: HashMap<(i32, i32), CellStatus>,
    next_input: Option<i64>,
    output: Vec<i32>,
    result: MoveResult,
}

impl ExecutionContext {
    fn new(memory: &Memory) -> ExecutionContext {
        ExecutionContext {
            ip: 0,
            memory: memory.clone(),
            ended: false,
            relative_base: 0,
            grid: HashMap::new(),
            output: vec![],
            next_input: Some(0),
            result: MoveResult::Moved,
        }
    }

    fn read_input(&mut self) -> Option<i64> {
        //println!("Reading input: {:?}", self.next_input);
        let res = self.next_input;
        self.next_input = None;
        res
    }

    fn write_output(&mut self, value: i64) {
        //println!("Writing output: {}", value);
        self.output.push(value as i32);
        self.result = match self.output[0] {
            0 => MoveResult::HitWall,
            1 => MoveResult::Moved,
            2 => MoveResult::FoundOxygen,
            x => panic!(format!("Invalid result: {}", x)),
        };

        self.output.clear();
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum MoveResult {
    HitWall,
    Moved,
    FoundOxygen,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum CellStatus {
    Origin,
    Unknown,
    Visited,
    VisitedAll,
    Wall,
    Oxygen,
}

enum ExecutionResult {
    MoreInputNeeded,
    Exit,
}

fn execute_program(context: &mut ExecutionContext) -> ExecutionResult {
    // println!("Executing program; ip: {}", context.ip.get());
    loop {
        match read_op_code(context) {
            (OpCode::Add, parameter_modes) => {
                let (a, b, c) = extract_parameters3(context, parameter_modes);
                c.set(a.get(context) + b.get(context), context);
            }
            (OpCode::Mult, parameter_modes) => {
                let (a, b, c) = extract_parameters3(context, parameter_modes);
                c.set(a.get(context) * b.get(context), context);
            }
            (OpCode::Input, parameter_modes) => {
                match context.read_input() {
                    Some(value) => {
                        // println!("Reading input {}", value);
                        let a = extract_parameter(context, parameter_modes);
                        a.set(value, context);
                    }
                    None => {
                        // println!("Halting program due to input read; ip: {}", context.ip);
                        // Revert the reading of the op-code, so we can read it again when the
                        // thread is resumed
                        context.ip -= 1;
                        return ExecutionResult::MoreInputNeeded;
                    }
                }
            }
            (OpCode::Output, parameter_modes) => {
                let a = extract_parameter(context, parameter_modes);
                let output = a.get(&context);
                //println!("Output: {}", output);
                context.write_output(output);
            }
            (OpCode::JumpIfTrue, parameter_modes) => {
                let mut jump_address: Option<i64> = None;
                let (a, b) = extract_parameters2(context, parameter_modes);
                if a.get(&context) != 0 {
                    jump_address = Some(b.get(&context));
                }

                if let Some(address) = jump_address {
                    jump_to(&mut context.ip, address);
                }
            }
            (OpCode::JumpIfFalse, parameter_modes) => {
                let mut jump_address: Option<i64> = None;
                let (a, b) = extract_parameters2(context, parameter_modes);
                if a.get(&context) == 0 {
                    jump_address = Some(b.get(&context));
                }

                if let Some(address) = jump_address {
                    jump_to(&mut context.ip, address);
                }
            }
            (OpCode::LessThan, parameter_modes) => {
                let (a, b, c) = extract_parameters3(context, parameter_modes);
                let value = if a.get(&context) < b.get(&context) {
                    1
                } else {
                    0
                };
                c.set(value, context);
            }
            (OpCode::Equals, parameter_modes) => {
                let (a, b, c) = extract_parameters3(context, parameter_modes);
                let value = if a.get(&context) == b.get(&context) {
                    1
                } else {
                    0
                };
                c.set(value, context);
            }
            (OpCode::AdjustRelativeBase, parameter_modes) => {
                let a = extract_parameter(context, parameter_modes);
                let adjustment = a.get(&context);
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
    let value = context.memory[context.ip];
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
        x => panic!("Unknown op code: {}; ip: {}", x, context.ip),
    };

    context.ip += 1;
    (op_code, parameter_modes)
}

fn extract_parameter(context: &mut ExecutionContext, parameter_modes: u32) -> Parameter {
    let mut param_modes = parameter_modes;
    get_parameter(context, &mut param_modes)
}

fn extract_parameters2(
    context: &mut ExecutionContext,
    parameter_modes: u32,
) -> (Parameter, Parameter) {
    let mut param_modes = parameter_modes;
    let x = get_parameter(context, &mut param_modes);
    let y = get_parameter(context, &mut param_modes);
    (x, y)
}

fn extract_parameters3(
    context: &mut ExecutionContext,
    parameter_modes: u32,
) -> (Parameter, Parameter, Parameter) {
    let mut param_modes = parameter_modes;
    let x = get_parameter(context, &mut param_modes);
    let y = get_parameter(context, &mut param_modes);
    let z = get_parameter(context, &mut param_modes);
    (x, y, z)
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

    let parameter_value = context.memory[context.ip];
    context.ip += 1;

    match parameter_mode {
        ParameterMode::Position => Parameter::Reference(parameter_value as usize),
        ParameterMode::Immediate => Parameter::ImmediateValue(parameter_value),
        ParameterMode::Relative => {
            let address = (parameter_value + context.relative_base as i64) as usize;
            Parameter::Reference(address)
        }
    }
}

enum Parameter {
    ImmediateValue(i64),
    Reference(usize),
}

impl<'a> Parameter {
    fn get(&self, context: &ExecutionContext) -> i64 {
        match self {
            Parameter::Reference(address) => context.memory[*address],
            Parameter::ImmediateValue(value) => *value,
        }
    }

    fn set(&self, value: i64, context: &mut ExecutionContext) -> () {
        match self {
            Parameter::Reference(address) => context.memory[*address] = value,
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
