use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::result::Result;
use std::{env, thread, time::Duration};

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

type Grid<T> = HashMap<Pos, T>;
type ContentGrid = Grid<Content>;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
enum State {
    Blocked,
    None,
    Visited(u32),
    VisitedAll(u32),
}

impl State {
    fn is_blocked(&self) -> bool {
        match *self {
            State::Blocked => true,
            _ => false,
        }
    }

    fn get_distance(&self) -> Option<u32> {
        match *self {
            State::Visited(d) | State::VisitedAll(d) => Some(d),
            _ => None,
        }
    }
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
        loop {
            match &self.next_direction {
                None => break None,
                Some(d) => {
                    let Pos(x, y) = self.origin;
                    match d {
                        Direction::Up => {
                            self.next_direction = Some(Direction::Right);
                            if y > 0 {
                                break Some(Pos(x, y - 1));
                            }
                        }
                        Direction::Right => {
                            self.next_direction = Some(Direction::Bottom);
                            break Some(Pos(x + 1, y));
                        }
                        Direction::Bottom => {
                            self.next_direction = Some(Direction::Left);
                            break Some(Pos(x, y + 1));
                        }
                        Direction::Left => {
                            self.next_direction = None;
                            if x > 0 {
                                break Some(Pos(x - 1, y));
                            }
                        }
                    };
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
    let mut grid: ContentGrid = ContentGrid::new();

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
    display_grid(&grid, current_pos, |s| match s {
        Some(Content::Wall) => String::from("#"),
        Some(Content::Key(v)) => format!("{}", v),
        Some(Content::Door(v)) => format!("{}", v),
        Some(Content::Passage) => ".".to_string(),
        _ => " ".to_string(),
    });

    let not_wall_position = |p: &Pos| {
        let s = grid.get(p);
        match s {
            Some(Content::Wall) | None => false,
            _ => true,
        }
    };

    for _ in 0..2 {
        println!("Current pos: {:?}", current_pos);
        let next_moves = get_neighbouring_positions(current_pos).filter(not_wall_position);

        let moves: Vec<_> = next_moves.collect();
        println!("Moves: {:?}", moves);

        let keys: HashSet<Key> = HashSet::new();
        get_reachable_keys(&grid, keys, current_pos);
    }

    Ok(())
}

type Key = char;

fn get_reachable_keys(grid: &ContentGrid, keys: HashSet<Key>, pos: Pos) -> HashMap<Key, i32> {
    let mut result: HashMap<Key, i32> = HashMap::new();
    visit_all_from(grid, keys, pos, |key: Key, distance: i32| {
        result.insert(key, distance);
    });

    result
}

struct Cursor {
    position: Pos,
    distance: u32,
}

fn visit_all_from(
    grid: &ContentGrid,
    keys: HashSet<Key>,
    from_pos: Pos,
    on_key_reached: impl FnMut(Key, i32) -> (),
) {
    let mut position = from_pos;
    let mut distance = 0;

    let mut state: Grid<State> = grid
        .iter()
        .map(|(k, v)| {
            (
                *k,
                match v {
                    Content::Wall => State::Blocked,
                    Content::Door(d) if !keys.contains(d) => State::Blocked,
                    _ => State::None,
                },
            )
        })
        .collect();

    state.insert(position, State::VisitedAll(0));
    let mut cursors = vec![Cursor {
        position: from_pos,
        distance: 0,
    }];

    loop {
        print_state(&state, position);
        thread::sleep(Duration::from_millis(200));

        let current_distance = state[&position].get_distance().unwrap();
        let alternative_moves: Vec<_> = get_neighbouring_positions(position)
            .filter(|p| match state[p] {
                State::None => true,
                State::Visited(d) | State::VisitedAll(d) => d > current_distance + 1,
                _ => false,
            })
            .collect();

        println!("Alternative moves: {:?}", alternative_moves);
        if alternative_moves.is_empty() {
            break;
        }

        let single_alternative = alternative_moves.len() == 1;
        for neighbour in alternative_moves {
            match state[&neighbour] {
                State::None | State::Visited(_) | State::VisitedAll(_) => {
                    position = neighbour;
                    let new_state = if single_alternative {
                        State::VisitedAll(distance + 1)
                    } else {
                        State::Visited(distance + 1)
                    };
                    state.insert(position, new_state);
                    break;
                }
                _ => (),
            }
        }
    }
}

fn print_state(state_grid: &Grid<State>, current_pos: Pos) {
    display_grid(state_grid, current_pos, |s| match s {
        Some(State::None) | None => String::from(" "),
        Some(State::Visited(d)) => format!("{}", d % 10),
        Some(State::VisitedAll(d)) => format!("{}", d % 10),
        Some(State::Blocked) => String::from("█"),
    });
}

fn get_neighbouring_positions(pos: Pos) -> NextMoveIterator {
    NextMoveIterator::new(pos)
}

fn display_grid<T>(grid: &Grid<T>, current_pos: Pos, display: impl Fn(Option<&T>) -> String) {
    let x_max = grid.keys().map(|Pos(x, _)| *x).max().unwrap();
    let y_max = grid.keys().map(|Pos(_, y)| *y).max().unwrap();

    for y in 0..y_max + 1 {
        for x in 0..x_max + 1 {
            if Pos(x, y) == current_pos {
                print!("@");
            }
            print!("{}", display(grid.get(&Pos(x, y))));
        }

        println!();
    }
}
