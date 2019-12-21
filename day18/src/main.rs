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

#[allow(dead_code)]
#[derive(Debug)]
enum Content {
    Wall,
    Key(char),
    Door(char),
    Passage,
}

type Grid = HashMap<Pos, Content>;

#[allow(dead_code)]
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
                        Some(Pos(x + 1, y))
                    }
                    Direction::Bottom => {
                        self.next_direction = Some(Direction::Left);
                        Some(Pos(x, y + 1))
                    }
                    Direction::Left => {
                        self.next_direction = None;
                        Some(Pos(x - 1, y))
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
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
    let mut grid: Grid = Grid::new();

    let mut y = 0;
    let mut current_pos = Pos(0, 0);
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
                    grid.insert(pos, Content::Wall);
                }
                '.' => {
                    grid.insert(pos, Content::Passage);
                }
                '@' => {
                    current_pos = pos;
                    grid.insert(pos, Content::Passage);
                }
                x if x.is_lowercase() => {
                    grid.insert(pos, Content::Key(x));
                }
                x if x.is_uppercase() => {
                    grid.insert(pos, Content::Door(x));
                }
                _ => (),
            }
        }

        y += 1;
    }

    //println!("Walls: {:?}", walls);
    //println!("State: {:?}", state);
    display_grid(&grid, |s| match s {
        Some(Content::Wall) => String::from("#"),
        Some(Content::Key(v)) => format!("{}", v),
        Some(Content::Door(v)) => format!("{}", v),
        Some(Content::Passage) => ".".to_string(),
        _ => " ".to_string(),
    });

    loop {
        println!("Current pos: {:?}", current_pos);
        let next_moves = get_neighbouring_positions(current_pos).filter(|p| {
            let s = grid.get(p);
            match s {
                Some(Content::Wall) | None => false,
                _ => true,
            }
        });

        let moves: Vec<_> = next_moves.collect();
        println!("Moves: {:?}", moves);
        break;
    }

    Ok(())
}

fn get_accessible_keys(grid: &HashMap<Pos, State>, pos: Pos) -> Vec<(char, u32)> {
    return vec![];
}

fn get_neighbouring_positions(pos: Pos) -> NextMoveIterator {
    NextMoveIterator::new(pos)
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
