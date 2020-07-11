#![allow(unused_variables)]
#![allow(dead_code)]

use crate::iterators::*;
use linked_hash_set::LinkedHashSet;
use log::*;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::result::Result;

mod iterators;

type MainResult<T> = Result<T, Box<dyn ::std::error::Error>>;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Pos(usize, usize);

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
    Key(char),
    Door(char),
}

impl State {
    #[allow(dead_code)]
    fn is_blocked(&self) -> bool {
        match *self {
            State::Blocked => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn get_distance(&self) -> Option<u32> {
        match *self {
            State::Visited(d) => Some(d),
            _ => None,
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
    simple_logger::init().unwrap();
    log::set_max_level(LevelFilter::Info);
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

    let key_count = count_keys(&grid);

    //println!("Walls: {:?}", walls);
    //println!("State: {:?}", state);
    // display_grid(&grid, Some(current_pos), |s| match s {
    //     Some(Content::Wall) => String::from("#"),
    //     Some(Content::Key(v)) => format!("{}", v),
    //     Some(Content::Door(v)) => format!("{}", v),
    //     Some(Content::Passage) => ".".to_string(),
    //     _ => " ".to_string(),
    // });

    let not_wall_position = |p: &Pos| {
        let s = grid.get(p);
        match s {
            Some(Content::Wall) | None => false,
            _ => true,
        }
    };

    let current_pos = current_pos;
    let mut mementos: Vec<Memento> = vec![Memento::new(LinkedHashSet::<_>::new(), current_pos, 0)];

    let mut min_total_distance = u32::max_value();
    while !mementos.is_empty() {
        // println!("Current pos: {:?}", current_pos);
        // let next_moves = get_neighbouring_positions(current_pos).filter(not_wall_position);

        // let moves: Vec<_> = next_moves.collect();
        // println!("Moves: {:?}", moves);

        let memento = mementos.pop().unwrap();
        if memento.distance_from_origin >= min_total_distance {
            continue;
        }
        let keys = &memento.keys;
        debug!("Keys: {:?}", keys);

        // Find out which keys are reachable from the current position and the
        // set of keys we have
        let keys_distances = get_reachable_keys(&grid, &keys, memento.position);
        debug!("Key distances: {:?}", keys_distances);

        if keys_distances.is_empty() {
            info!(
                "No more reachable keys. Keys: {:?}, Total Distance: {}",
                memento.keys, memento.distance_from_origin
            );
        }

        if keys_distances.len() == 1 && keys.len() == key_count - 1 {
            let (key, (_, key_distance)) = keys_distances.iter().take(1).last().unwrap();
            let total_distance = key_distance + memento.distance_from_origin;
            debug!("All keys found! Distance = {}", total_distance);
            if total_distance < min_total_distance {
                min_total_distance = min(min_total_distance, total_distance);
                let mut keys = memento.keys.clone();
                keys.insert(*key);
                info!(
                    "New min path found: {:?}; Distance = {}",
                    keys, total_distance
                );
            }
            continue;
        }
        // Now we can choose to continue with any of the reachable keys
        let mut ordered_keys: Vec<_> = keys_distances.keys().collect();
        ordered_keys.sort_by_key(|k| u32::max_value() - keys_distances[k].1);
        for &key in ordered_keys {
            let (position, distance) = keys_distances[&key];
            let total_distance = distance + memento.distance_from_origin;
            if total_distance < min_total_distance {
                let mut memento_keys = keys.clone();
                memento_keys.insert(key);
                mementos.push(Memento::new(memento_keys, position, total_distance))
            }
        }

        if mementos.is_empty() {
            break;
        }
    }

    info!("Min distance: {}", min_total_distance);

    Ok(())
}

fn count_keys(grid: &ContentGrid) -> usize {
    grid.values()
        .filter(|x| match x {
            Content::Key(_) => true,
            _ => false,
        })
        .count()
}

struct Memento {
    keys: LinkedHashSet<Key>,
    position: Pos,
    distance_from_origin: u32,
}

impl Memento {
    fn new(keys: LinkedHashSet<Key>, position: Pos, distance_from_origin: u32) -> Memento {
        Memento {
            keys,
            position,
            distance_from_origin,
        }
    }
}

type Key = char;

fn get_reachable_keys(
    grid: &ContentGrid,
    keys: &LinkedHashSet<Key>,
    pos: Pos,
) -> HashMap<Key, (Pos, u32)> {
    let mut result: HashMap<_, _> = HashMap::new();
    visit_all_from(grid, keys, pos, |key: Key, pos: Pos, distance: u32| {
        let existing_distance = result.get(&key);
        match existing_distance {
            Some(&(_, d)) if d > distance => (),
            _ => {
                result.insert(key, (pos, distance));
            }
        }
    });

    result
}

#[derive(Debug)]
struct Cursor {
    position: Pos,
    distance: u32,
}

impl Cursor {
    fn new(pos: Pos, distance: u32) -> Cursor {
        Cursor {
            position: pos,
            distance,
        }
    }
}

fn visit_all_from(
    grid: &ContentGrid,
    keys: &LinkedHashSet<Key>,
    from_pos: Pos,
    mut on_key_reached: impl FnMut(Key, Pos, u32) -> (),
) {
    let mut state: Grid<State> = grid
        .iter()
        .map(|(k, v)| {
            (
                *k,
                match v {
                    Content::Wall => State::Blocked,
                    Content::Key(k) if !keys.contains(k) => State::Key(*k),
                    Content::Door(d) if !keys.contains(&d.to_ascii_lowercase()) => State::Door(*d),
                    _ => State::None,
                },
            )
        })
        .collect();

    state.insert(from_pos, State::Visited(0));
    let mut cursors = vec![Cursor {
        position: from_pos,
        distance: 0,
    }];

    print_state(&state, None);

    // clear();
    while !cursors.is_empty() {
        // set_cursor_position(0, 0);
        if log_enabled!(Level::Trace) {
            print_state(&state, None);
        }
        // println!();
        // for c in &cursors {
        //     println!("{:?}", c);
        // }

        // for _ in 0..10 {
        //     println!("                                                      ");
        // }

        //thread::sleep(Duration::from_millis(10));

        let mut next_cursors = vec![];
        for c in &cursors {
            // For each cursor,
            // See where we can go
            let next_moves: Vec<_> = get_neighbouring_positions(c.position)
                .filter(|p| match state[p] {
                    State::None => true,
                    State::Key(k) => {
                        on_key_reached(k, *p, c.distance + 1);
                        false
                    }
                    State::Visited(d) => d > c.distance + 1,
                    _ => false,
                })
                .collect();
            for m in next_moves {
                state.insert(m, State::Visited(c.distance + 1));
                next_cursors.push(Cursor::new(m, c.distance + 1));
            }
        }

        cursors = next_cursors;
    }
}

fn print_state(state_grid: &Grid<State>, current_pos: Option<Pos>) {
    display_grid(state_grid, current_pos, |s| match s {
        Some(State::None) | None => String::from("  "),
        Some(State::Visited(d)) => format!("{} ", d % 10),
        Some(State::Key(k)) => format!("{} ", k),
        Some(State::Door(k)) => format!("{} ", k),
        Some(State::Blocked) => String::from("██"),
    });
}

fn get_neighbouring_positions(pos: Pos) -> NextMoveIterator {
    NextMoveIterator::new(pos)
}

fn display_grid<T>(
    grid: &Grid<T>,
    current_pos: Option<Pos>,
    display: impl Fn(Option<&T>) -> String,
) {
    if !log_enabled!(Level::Debug) {
        return;
    }
    let x_max = grid.keys().map(|Pos(x, _)| *x).max().unwrap();
    let y_max = grid.keys().map(|Pos(_, y)| *y).max().unwrap();

    for y in 0..y_max + 1 {
        for x in 0..x_max + 1 {
            if let Some(p) = current_pos {
                if p == Pos(x, y) {
                    print!("@");
                }
            }
            print!("{}", display(grid.get(&Pos(x, y))));
        }

        println!();
    }
    println!();
}

extern crate kernel32;
extern crate winapi;

#[cfg(windows)]
use winapi::wincon::CONSOLE_SCREEN_BUFFER_INFO;
#[cfg(windows)]
use winapi::wincon::COORD;
#[cfg(windows)]
use winapi::wincon::SMALL_RECT;
#[cfg(windows)]
use winapi::DWORD;
#[cfg(windows)]
use winapi::HANDLE;
#[cfg(windows)]
use winapi::WORD;

#[cfg(windows)]
static mut CONSOLE_HANDLE: Option<HANDLE> = None;

#[cfg(windows)]
fn get_output_handle() -> HANDLE {
    unsafe {
        if let Some(handle) = CONSOLE_HANDLE {
            handle
        } else {
            let handle = kernel32::GetStdHandle(winapi::STD_OUTPUT_HANDLE);
            CONSOLE_HANDLE = Some(handle);
            handle
        }
    }
}

#[cfg(windows)]
fn get_buffer_info() -> winapi::CONSOLE_SCREEN_BUFFER_INFO {
    let handle = get_output_handle();
    if handle == winapi::INVALID_HANDLE_VALUE {
        panic!("NoConsole")
    }
    let mut buffer = CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: COORD { X: 0, Y: 0 },
        dwCursorPosition: COORD { X: 0, Y: 0 },
        wAttributes: 0 as WORD,
        srWindow: SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 0,
            Bottom: 0,
        },
        dwMaximumWindowSize: COORD { X: 0, Y: 0 },
    };
    unsafe {
        kernel32::GetConsoleScreenBufferInfo(handle, &mut buffer);
    }
    buffer
}

#[cfg(windows)]
fn clear() {
    let handle = get_output_handle();
    if handle == winapi::INVALID_HANDLE_VALUE {
        panic!("NoConsole")
    }

    let screen_buffer = get_buffer_info();
    let console_size: DWORD = screen_buffer.dwSize.X as u32 * screen_buffer.dwSize.Y as u32;
    let coord_screen = COORD { X: 0, Y: 0 };

    let mut amount_chart_written: DWORD = 0;
    unsafe {
        kernel32::FillConsoleOutputCharacterW(
            handle,
            32 as winapi::WCHAR,
            console_size,
            coord_screen,
            &mut amount_chart_written,
        );
    }
    set_cursor_position(0, 0);
}

#[cfg(windows)]
fn set_cursor_position(y: i16, x: i16) {
    let handle = get_output_handle();
    if handle == winapi::INVALID_HANDLE_VALUE {
        panic!("NoConsole")
    }
    unsafe {
        kernel32::SetConsoleCursorPosition(handle, COORD { X: x, Y: y });
    }
}
