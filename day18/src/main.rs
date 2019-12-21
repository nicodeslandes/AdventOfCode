extern crate num;

use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::result::Result;

type MainResult<T> = Result<T, Box<dyn ::std::error::Error>>;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Pos(usize, usize);

#[derive(Debug)]
enum Content {
    Key(char),
    Door(char),
    Passage,
}

#[derive(Copy, Clone, Debug)]
enum State {
    Wall,
    None,
    Visited(u32),
}

struct NextMoveIterator {
    next_direction: Option<Direction>,
    origin: Pos,
}

impl NextMoveIterator {
    fn new(pos: Pos) -> NextMoveIterator {
        NextMoveIterator {
            origin: pos,
            next_direction: Some(Direction::Up),
        }
    }
}

impl Iterator for NextMoveIterator {
    type Item = Pos;

    fn next(&mut self) -> Option<Pos> {
        match &self.next_direction {
            None => None,
            Some(d) => {
                let Pos(x, y) = self.origin;
                match d {
                    Direction::Up => {
                        self.next_direction = Some(Direction::Right);
                        Some(Pos(x, y - 1))
                    }
                    Direction::Right => {
                        self.next_direction = Some(Direction::Bottom);
                        Some(Pos(x, y - 1))
                    }
                    Direction::Bottom => {
                        self.next_direction = Some(Direction::Left);
                        Some(Pos(x, y - 1))
                    }
                    Direction::Left => {
                        self.next_direction = None;
                        Some(Pos(x, y - 1))
                    }
                }
            }
        }
    }
}

enum Direction {
    Up,
    Right,
    Bottom,
    Left,
}

fn main() -> MainResult<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);

    let mut walls: HashSet<Pos> = HashSet::new();
    let mut state: HashMap<Pos, State> = HashMap::new();

    let mut y = 0;
    let mut current_pos: Pos;
    loop {
        let mut line = String::new();
        let read = reader.read_line(&mut line)?;
        if read == 0 {
            break;
        }

        for (x, ch) in line.chars().enumerate() {
            let pos = Pos(x, y);
            match ch {
                '#' => {
                    walls.insert(pos);
                    state.insert(pos, State::Wall);
                }
                '.' => {
                    state.insert(pos, State::None);
                }
                '@' => {
                    current_pos = pos;
                }
                x => (),
            }
        }

        y += 1;
    }

    println!("Walls: {:?}", walls);
    println!("State: {:?}", state);
    display_grid(&state, |s| match s {
        Some(State::Wall) => String::from("#"),
        Some(State::Visited(v)) => format!("{}", v % 10),
        None | Some(State::None) => " ".to_string(),
    });

    Ok(())
}

fn display_grid<T>(grid: &HashMap<Pos, T>, display: impl Fn(Option<&T>) -> String) {
    let x_max = grid.keys().map(|Pos(x, _)| *x).max().unwrap();
    let y_max = grid.keys().map(|Pos(_, y)| *y).max().unwrap();

    for y in 0..y_max + 1 {
        for x in 0..x_max + 1 {
            print!("{}", display(grid.get(&Pos(x, y))));
        }

        println!();
    }
}

struct Pattern {
    current_index: usize,
    repeat_len: usize,
    pattern_values: Vec<i32>,
}

impl Pattern {
    fn new(repeat_len: usize) -> Pattern {
        Pattern {
            current_index: 0,
            repeat_len,
            pattern_values: vec![0, 1, 0, -1],
        }
    }
}

impl Iterator for Pattern {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.current_index += 1;
        Some(
            self.pattern_values[(self.current_index / self.repeat_len) % self.pattern_values.len()],
        )
    }
}

fn calculate_iteration(input: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for i in 0..input.len() {
        let pattern = Pattern::new(i + 1);

        let r: i32 = input.iter().zip(pattern).map(|(v, p)| v * p).sum();
        result.push((r % 10).abs());
    }

    result
}
