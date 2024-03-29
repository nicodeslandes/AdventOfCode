use crossterm::cursor;
use crossterm::{queue, style, terminal};
use log::debug;
use simplelog::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::stdout;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::time::Duration;
use std::{env, thread};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
type Grid = HashSet<Pos>;

fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;
    let file_name = env::args().nth(1).expect("Enter a file name");
    let (mut grid, instructions) = parse_lines(&file_name)?;

    debug!("Grid: {:?}", grid);
    debug!("Instructions: {:?}", instructions);
    apply_fold(&mut grid, instructions[0]);
    let part1 = grid.len();

    for &instruction in instructions.iter().skip(1) {
        apply_fold(&mut grid, instruction);
    }

    print_grid(&grid)?;
    let part2 = 0;
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn print_grid(grid: &Grid) -> Result<()> {
    let mut stdout = stdout();
    let mut max_y = 0;
    queue!(stdout, terminal::Clear(terminal::ClearType::All))?;
    for &pos in grid {
        queue!(stdout, cursor::MoveTo(pos.x, pos.y), style::Print("#"))?;
        stdout.flush()?;
        thread::sleep(Duration::from_millis(30));
        max_y = max_y.max(pos.y);
    }

    queue!(stdout, cursor::MoveTo(0, max_y + 1))?;
    Ok(())
}

fn apply_fold(grid: &mut Grid, instruction: Instruction) {
    let positions: Vec<_> = grid.iter().copied().collect();
    grid.clear();
    match instruction {
        Instruction::FoldX(fold) => {
            for p in positions {
                grid.insert(if p.x < fold {
                    p
                } else {
                    Pos {
                        x: fold - (p.x - fold),
                        y: p.y,
                    }
                });
            }
        }
        Instruction::FoldY(fold) => {
            for p in positions {
                grid.insert(if p.y < fold {
                    p
                } else {
                    Pos {
                        x: p.x,
                        y: fold - (p.y - fold),
                    }
                });
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: u16,
    y: u16,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    FoldX(u16),
    FoldY(u16),
}

fn parse_lines(file_name: &str) -> Result<(Grid, Vec<Instruction>)> {
    debug!("Reading input from {}", file_name);
    let file = File::open(file_name)?;
    let mut lines = BufReader::new(&file).lines();
    let mut positions = Grid::new();
    loop {
        match lines.next() {
            None => break,
            Some(line) => {
                let line = line.unwrap();
                if line.len() == 0 {
                    break;
                }
                if let [x, y] = line
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<_>>()[..]
                {
                    positions.insert(Pos { x: x, y: y });
                }
            }
        }
    }

    let mut instructions = vec![];
    loop {
        match lines.next() {
            None => break,
            Some(line) => {
                let line = line.unwrap();
                if line.len() == 0 {
                    break;
                }
                if let [text, val] = line.split('=').collect::<Vec<&str>>()[..] {
                    let instruction = match text {
                        "fold along x" => Instruction::FoldX(val.parse().unwrap()),
                        "fold along y" => Instruction::FoldY(val.parse().unwrap()),
                        _ => panic!("Unexpected: {}", text),
                    };
                    instructions.push(instruction);
                }
            }
        }
    }

    Ok((positions, instructions))
}
