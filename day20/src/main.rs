#![allow(dead_code)]

use crate::iterators::*;
use num_format::{Locale, ToFormattedString};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::result::Result;
use std::time::Instant;

mod iterators;

type MainResult<T> = Result<T, Box<dyn ::std::error::Error>>;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Pos(usize, usize);

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Pos3D {
    pos: Pos,
    level: usize,
}

impl Pos3D {
    fn new(pos: Pos, level: usize) -> Pos3D {
        Pos3D { pos, level }
    }
}

#[derive(Debug)]
enum Content {
    Wall,
    Passage,
    Portal(String),
}

type StateGrid = Grid<State>;
type ContentGrid = Grid<Content>;
type Grid<T> = HashMap<Pos, T>;

type StateGrid3D = HashMap<usize, StateGrid>;

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
        let mut chars = vec![character_grid[&pos1], character_grid[&pos2]];
        chars.sort();
        String::from_iter(chars.into_iter())
    };

    let mut portals: Vec<(String, Pos)> = vec![];

    let mut gen_portal = |pos1: Pos, pos2: Pos| {
        let name = read_portal_name(pos1, pos2);
        portals.push((name.clone(), pos1));
        Content::Portal(name)
    };

    let mut try_read_portal = |pos| {
        // A portal position should contain an alphabetic character, and be adjacent to a passage
        match character_grid.get(&pos) {
            None => None,
            Some(ch) => {
                let Pos(x, y) = pos;

                let is_passage = |x, y| match character_grid.get(&Pos(x, y)) {
                    Some('.') => true,
                    _ => false,
                };

                if !ch.is_alphabetic() {
                    None
                } else {
                    if is_passage(x - 1, y) {
                        Some(gen_portal(pos, Pos(x + 1, y)))
                    } else if is_passage(x + 1, y) {
                        Some(gen_portal(pos, Pos(x - 1, y)))
                    } else if is_passage(x, y - 1) {
                        Some(gen_portal(pos, Pos(x, y + 1)))
                    } else if is_passage(x, y + 1) {
                        Some(gen_portal(pos, Pos(x, y - 1)))
                    } else {
                        None
                    }
                }
            }
        }
    };
    for y in 1..y_max {
        for x in 1..x_max {
            let pos = Pos(x, y);
            let content = match character_grid.get(&pos) {
                Some('.') => Content::Passage,
                Some('#') => Content::Wall,
                Some(ch) if ch.is_alphabetic() => {
                    if let Some(portal) = try_read_portal(pos) {
                        portal
                    } else {
                        continue;
                    }
                }
                _ => continue,
            };

            grid.insert(pos, content);
        }
    }

    display_content_grid(&grid, None);
    let start = Instant::now();

    let current = grid
        .iter()
        .find(|(_, v)| match v {
            Content::Portal(s) => s == "AA",
            _ => false,
        })
        .map(|(pos, _)| {
            NextMoveIterator::new(*pos).find(|p| match grid.get(p) {
                Some(Content::Passage) => true,
                _ => false,
            })
        })
        .unwrap()
        .unwrap();

    let current = Pos3D::new(current, 0);

    println!("Start position: {:?}", current);

    let distance = get_distance_to_exit(current, &grid, (x_max, y_max), &portals);
    println!(
        "Min distance found in {} ms: {}",
        (Instant::now() - start)
            .as_millis()
            .to_formatted_string(&Locale::en),
        distance
    );
    Ok(())
}

fn get_distance_to_exit(
    start: Pos3D,
    grid: &ContentGrid,
    dim: (usize, usize),
    portals: &Vec<(String, Pos)>,
) -> u32 {
    // Connect each portal to its destination
    let mut portals_by_key: HashMap<String, Vec<Pos>> = HashMap::new();
    for (portal_name, pos) in portals {
        match portals_by_key.get_mut(portal_name) {
            None => {
                portals_by_key.insert(portal_name.clone(), vec![*pos]);
            }
            Some(v) => v.push(*pos),
        }
    }

    let get_portal_destination = |name: &String, from: Pos| {
        let portal_positions = &portals_by_key[name];
        let other_end = if portal_positions[0] == from {
            portal_positions[1]
        } else {
            portal_positions[0]
        };
        NextMoveIterator::new(other_end)
            .find(|p| match grid.get(p) {
                Some(Content::Passage) => true,
                _ => false,
            })
            .unwrap()
    };
    let mut base_state: StateGrid = StateGrid::new();
    let (x_max, y_max) = dim;
    for x in 0..x_max {
        for y in 0..y_max {
            let pos = Pos(x, y);
            base_state.insert(
                pos,
                match grid.get(&pos) {
                    Some(Content::Wall) => State::Wall,
                    Some(Content::Portal(s)) if s == "AA" => State::Origin,
                    Some(Content::Portal(s)) if s == "ZZ" => State::Exit,
                    Some(Content::Portal(name)) => {
                        let destination = get_portal_destination(name, pos);
                        let name = name.clone();
                        if x < 2 || y < 2 || x >= x_max - 3 || y >= y_max - 2 {
                            State::OuterPortal(name, destination)
                        } else {
                            State::InnerPortal(name, destination)
                        }
                    }
                    _ => State::None,
                },
            );
        }
    }

    //display_state_grid(&state, Some(start));

    let mut cursors = vec![start];
    let mut distance = 0;

    let mut state = StateGrid3D::new();

    while !cursors.is_empty() {
        let mut new_cursors = vec![];

        //println!("Cursors: {:?}", cursors);
        for c in &cursors {
            let level_state = match state.get_mut(&c.level) {
                Some(s) => s,
                _ => {
                    state.insert(c.level, base_state.clone());
                    state.get_mut(&c.level).unwrap()
                }
            };

            match level_state.get(&c.pos) {
                Some(State::Visited(d)) if *d <= distance => continue,
                _ => (),
            }

            level_state.insert(c.pos, State::Visited(distance));

            //set_cursor_position(0, 0);
            //display_state_grid(&state, None);

            for m in NextMoveIterator::new(c.pos) {
                match level_state.get(&m) {
                    Some(State::None) => {
                        new_cursors.push(Pos3D::new(m, c.level));
                    }
                    Some(State::InnerPortal(_, p)) => {
                        let portal_dest = p.clone();
                        new_cursors.push(Pos3D::new(portal_dest, c.level + 1));
                    }
                    Some(State::OuterPortal(_, p)) => {
                        if c.level > 0 {
                            let portal_dest = p.clone();
                            new_cursors.push(Pos3D::new(portal_dest, c.level - 1));
                        }
                    }
                    Some(State::Exit) => {
                        if c.level == 0 {
                            return distance;
                        }
                    }
                    _ => (),
                }
            }
        }

        cursors = new_cursors;
        distance += 1;
    }

    return distance;
}

fn display_state_grid(grid: &StateGrid, current_pos: Option<Pos>) {
    display_grid(grid, current_pos, |_pos, s| match s {
        Some(State::None) | None => String::from("  "),
        Some(State::Visited(d)) => format!("{:2}", d % 100),
        Some(State::InnerPortal(name, _)) => name.to_lowercase(),
        Some(State::OuterPortal(name, _)) => name.clone(),
        Some(State::Origin) => String::from("AA"),
        Some(State::Exit) => String::from("ZZ"),
        Some(State::Wall) => String::from("██"),
    });
}

fn display_content_grid(grid: &ContentGrid, current_pos: Option<Pos>) {
    display_grid(grid, current_pos, |_pos, s| match s {
        Some(Content::Passage) | None => String::from("  "),
        Some(Content::Portal(p)) => p.clone(),
        Some(Content::Wall) => String::from("██"),
    });
}

fn display_grid<T>(
    grid: &Grid<T>,
    current_pos: Option<Pos>,
    display: impl Fn(Pos, Option<&T>) -> String,
) {
    // if !log_enabled!(Level::Info) {
    //     return;
    // }
    let x_max = grid.keys().map(|Pos(x, _)| *x).max().unwrap();
    let y_max = grid.keys().map(|Pos(_, y)| *y).max().unwrap();

    for y in 0..=y_max {
        for x in 0..=x_max {
            if let Some(p) = current_pos {
                if p == Pos(x, y) {
                    print!("@ ");
                    continue;
                }
            }
            let pos = Pos(x, y);
            print!("{}", display(pos, grid.get(&pos)));
        }

        println!();
    }
    println!();
}

#[derive(Clone, Debug)]
enum State {
    Wall,
    None,
    InnerPortal(String, Pos),
    OuterPortal(String, Pos),
    Origin,
    Exit,
    Visited(u32),
}

extern crate kernel32;
extern crate winapi;

use winapi::wincon::COORD;
use winapi::HANDLE;

static mut CONSOLE_HANDLE: Option<HANDLE> = None;

fn get_output_handle() -> HANDLE {
    unsafe {
        if let Some(handle) = CONSOLE_HANDLE {
            return handle;
        } else {
            let handle = kernel32::GetStdHandle(winapi::STD_OUTPUT_HANDLE);
            CONSOLE_HANDLE = Some(handle);
            return handle;
        }
    }
}

fn set_cursor_position(y: i16, x: i16) {
    let handle = get_output_handle();
    if handle == winapi::INVALID_HANDLE_VALUE {
        panic!("NoConsole")
    }
    unsafe {
        kernel32::SetConsoleCursorPosition(handle, COORD { X: x, Y: y });
    }
}
