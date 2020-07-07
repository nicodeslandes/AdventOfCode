use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::result::Result;

type MainResult<T> = Result<T, Box<dyn ::std::error::Error>>;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Pos(usize, usize);

#[derive(Debug)]
enum Content {
    Wall,
    Passage,
    Portal(String),
}

fn main() -> MainResult<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");
    let file = File::open(file_name)?;

    let mut character_grid: HashMap<Pos, char> = HashMap::new();
    let mut grid: HashMap<Pos, Content> = HashMap::new();

    let mut reader = BufReader::new(&file);
    let mut y = 0;
    loop {
        let mut line = String::new();
        let read = reader.read_line(&mut line)?;
        if read == 0 {
            break;
        }

        for (x, ch) in line.chars().enumerate() {
            let pos = Pos(x, y);
            character_grid.insert(pos, ch);
        }

        y += 1;
    }

    let x_max = character_grid.keys().map(|Pos(x, _)| *x).max().unwrap();
    let y_max = character_grid.keys().map(|Pos(_, y)| *y).max().unwrap();

    let read_portal_name = |pos1: Pos, pos2: Pos| {
        let chars = vec![character_grid[&pos1], character_grid[&pos2]];
        String::from_iter(chars.into_iter())
    };

    let mut portals: Vec<(String, Pos)> = vec![];

    let mut gen_portal = |pos: Pos, pos1: Pos, pos2: Pos| {
        let name = read_portal_name(pos1, pos2);
        portals.push((name.clone(), pos));
        Content::Portal(name)
    };

    for y in 2..y_max - 1 {
        for x in 2..x_max - 1 {
            let pos = Pos(x, y);
            let content = match character_grid.get(&pos) {
                Some('.') => {
                    if x == 2 {
                        gen_portal(pos, Pos(x - 2, y), Pos(x - 1, y))
                    } else if x == x_max - 1 {
                        gen_portal(pos, Pos(x + 1, y), Pos(x + 2, y))
                    } else if y == 2 {
                        gen_portal(pos, Pos(x, y - 2), Pos(x, y - 1))
                    } else if y == y_max - 1 {
                        gen_portal(pos, Pos(x, y + 1), Pos(x, y + 2))
                    } else {
                        Content::Passage
                    }
                }
                Some('#') => Content::Wall,
                _ => continue,
            };

            grid.insert(pos, content);
        }
    }

    println!("Grid: {:?}", grid);

    let current = *grid
        .iter()
        .find(|(_, v)| match v {
            Content::Portal(s) => s == "AA",
            _ => false,
        })
        .unwrap()
        .0;

    println!("Start position: {:?}", current);

    let mut state: Vec<Vec<State>> = vec![];
    for x in 0..x_max {
        state.push(vec![State::None; y_max + 1]);
        for y in 0..y_max {
            state[x][y] = match grid.get(&Pos(x, y)) {
                Some(Content::Wall) => State::Wall,
                _ => State::None,
            }
        }
    }

    for y in 0..y_max {
        for x in 0..x_max {
            if current == Pos(x, y) {
                print!("x");
            } else {
                match state[x][y] {
                    State::Wall => print!("#"),
                    State::Visited(n) => print!("{}", n % 10),
                    _ => print!(" "),
                }
            }
        }
        println!();
    }

    // loop {
    //     match grid.get(&current){
    //         Some(Content::Passage) {
    //             for
    //         }
    //     }
    // }

    Ok(())
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
