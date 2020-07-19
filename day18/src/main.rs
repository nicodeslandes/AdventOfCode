#![allow(dead_code)]

use crate::grid::*;
use crate::iterators::*;
use linked_hash_set::LinkedHashSet;
use log::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::result::Result;

mod grid;
mod iterators;

type MainResult<T> = Result<T, Box<dyn ::std::error::Error>>;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Pos(usize, usize);

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Bottom,
    Left,
}

/// Represents a path between 2 keys, potentially with doors
/// between them
#[derive(Debug)]
struct KeyPath {
    from: Key,
    to: Key,
    distance: u32,
    doors: HashSet<Key>,
}

struct WalkState {
    obtained_keys: LinkedHashSet<Key>,
    reachable_keys: HashSet<Key>,
    blocked_keys: HashSet<Key>,
    total_distance: u32,
    current_position: Key,
}

fn main() -> MainResult<()> {
    simple_logger::init().unwrap();
    log::set_max_level(LevelFilter::Info);
    let file_name = env::args().nth(1).expect("Enter a file name");

    let (grid, initial_pos) = parse_grid(&file_name)?;
    display_content_grid(&grid, Some(initial_pos));

    let keys = get_keys(&grid);

    let key_count = keys.len();

    let mut path_map: HashMap<Key, Vec<KeyPath>> = HashMap::new();
    for (pos, key) in keys {
        let paths = get_all_paths_to_keys_from(&grid, pos);
        debug!("Paths from {}: {:?}", key, paths);
        path_map.insert(key, paths);
    }

    if log_enabled!(Level::Info) {
        for (k, paths) in &path_map {
            info!("From key {}", k);
            for p in paths {
                info!(
                    "  {}->{} ({}); Doors: {:?}",
                    p.from, p.to, p.distance, p.doors
                );
            }
        }
    }

    // Start with a single choice: '@', with a distance of 0
    let mut mementos: Vec<Memento> = vec![Memento::new(vec!['@'].into_iter().collect(), 0u32)];
    let mut min_total_distance = u32::max_value();
    let mut count = 0;
    while !mementos.is_empty() {
        count += 1;
        // Get all the
        // println!("Current pos: {:?}", current_pos);
        // let next_moves = get_neighbouring_positions(current_pos).filter(not_wall_position);

        // let moves: Vec<_> = next_moves.collect();
        // println!("Moves: {:?}", moves);

        let mut memento = mementos.pop().unwrap();
        let keys = &mut memento.keys;

        if memento.distance_from_origin >= min_total_distance {
            continue;
        }

        if keys.len() == key_count {
            // We have path with all keys
            debug!(
                "All keys found! Distance = {}",
                memento.distance_from_origin
            );
            min_total_distance = memento.distance_from_origin;
            info!(
                "New min path found: {:?}; Distance = {}",
                keys, min_total_distance
            );
            continue;
        }

        if count > 1_000_000 {
            info!(
                "Memento keys: {:?}, distance: {}",
                keys, memento.distance_from_origin
            );
            count = 0;
        } else {
            debug!(
                "Memento keys: {:?}, distance: {}",
                keys, memento.distance_from_origin
            );
        }
        // Find out which keys are reachable from the current position and the
        // set of keys we have
        let current_key = *keys.back().expect("No key found on memento!");
        let keys = &memento.keys;

        let mut reachable_keys: Vec<_> = path_map[&current_key]
            .iter()
            .filter(|key_path| {
                !keys.contains(&key_path.to)
                    && key_path.doors.iter().all(|door| keys.contains(door))
            })
            .collect();

        // if log_enabled!(Level::Debug) {
        //     let v: Vec<_> = reachable_keys.map(|kp| (kp.to, kp.distance)).collect();
        //     debug!("Reachable keys: {:?}", v);
        // }

        // if reachable_keys.is_empty() {
        //     info!(
        //         "No more reachable keys. Keys: {:?}, Total Distance: {}",
        //         memento.keys, memento.distance_from_origin
        //     );
        // }

        // Now we can choose to continue with any of the reachable keys
        reachable_keys.sort_by_key(|k| u32::max_value() - k.distance);
        for key_path in reachable_keys {
            let total_distance = key_path.distance + memento.distance_from_origin;
            if total_distance < min_total_distance {
                let mut memento_keys = keys.clone();
                memento_keys.insert(key_path.to);
                mementos.push(Memento::new(memento_keys, total_distance))
            }
        }
    }

    info!("Min distance: {}", min_total_distance);

    Ok(())
}

fn get_keys(grid: &ContentGrid) -> Vec<(Pos, Key)> {
    grid.iter()
        .filter(|x| match x {
            (_, Content::Key(_)) => true,
            _ => false,
        })
        .map(|x| match x {
            (pos, Content::Key(k)) => (*pos, *k),
            _ => panic!("Invalid match"),
        })
        .collect()
}

struct Memento {
    keys: LinkedHashSet<Key>,
    distance_from_origin: u32,
}

impl Memento {
    fn new(keys: LinkedHashSet<Key>, distance_from_origin: u32) -> Memento {
        Memento {
            keys,
            distance_from_origin,
        }
    }
}

type Key = char;

fn get_reachable_keys(
    _grid: &ContentGrid,
    _keys: &LinkedHashSet<Key>,
    _pos: Pos,
) -> HashMap<Key, (Pos, u32)> {
    HashMap::<_, _>::new()
}

//     let mut result: HashMap<_, _> = HashMap::new();
//     visit_all_from(
//         grid,
//         keys,
//         pos,
//         |c| match c {
//             Content::Wall => false,
//             _ => true,
//         },
//         |content, pos: Pos, distance: u32| match content {
//             Content::Key(key) => {
//                 let existing_distance = result.get(&key);
//                 match existing_distance {
//                     Some(&(_, d)) if d > distance => (),
//                     _ => {
//                         result.insert(key, (pos, distance));
//                     }
//                 }
//             }
//             _ => (),
//         },
//     );

//     result
// }

// fn get_all_paths_to_keys_from(grid: &ContentGrid, pos: Pos) -> HashMap<Key, KeyPath> {
//     let mut result: HashMap<_, _> = HashMap::new();
//     let from_key = grid[&pos].get_key();
//     visit_all_from(
//         grid,
//         &LinkedHashSet::<_>::new(),
//         pos,
//         |c| match c {
//             Content::Wall => false,
//             _ => true,
//         },
//         |key: Key, pos: Pos, distance: u32| {
//             let existing_distance = result.get(&key);
//             match existing_distance {
//                 Some(&(_, d)) if d > distance => (),
//                 _ => {
//                     result.insert(key, (pos, distance));
//                 }
//             }
//         },
//     );

//     result
// }

#[derive(Debug, Clone)]
struct Cursor {
    position: Pos,
    distance: u32,
    doors: Vec<char>,
}

impl Cursor {
    fn new(pos: Pos, distance: u32) -> Cursor {
        Cursor {
            position: pos,
            distance,
            doors: vec![],
        }
    }
}

fn get_all_paths_to_keys_from(grid: &ContentGrid, from_pos: Pos) -> Vec<KeyPath> {
    debug!("Calculating paths from position {:?}", from_pos);
    let mut result = vec![];
    let mut state: Grid<u32> = Grid::new();
    state.insert(from_pos, 0);

    let from_key = match grid[&from_pos] {
        Content::Key(k) => k,
        _ => panic!("Unexpected"),
    };

    let mut cursors = vec![Cursor {
        position: from_pos,
        distance: 0,
        doors: vec![],
    }];

    print_state(&grid, &state, None);

    let mut on_key_found = |k: Key, c: &Cursor| {
        // Ignore paths back to the origin
        if k == '@' {
            return;
        }
        debug!("Found key {}", k);
        result.push(KeyPath {
            from: from_key,
            to: k,
            distance: c.distance,
            doors: c.doors.iter().copied().collect(),
        });
    };

    // clear();
    while !cursors.is_empty() {
        // set_cursor_position(0, 0);
        if log_enabled!(Level::Trace) {
            print_state(&grid, &state, None);
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
        for (i, c) in cursors.iter().enumerate() {
            trace!("Cursor {}: position: {:?}", i, c.position);
            // For each cursor,
            // See where we can go
            let next_moves: Vec<_> = get_neighbouring_positions(c.position)
                .filter(|p| match grid[p] {
                    Content::Wall => false,
                    _ => true,
                })
                .filter(|p| !state.contains_key(p))
                .collect();

            for &m in next_moves.iter() {
                // TODO: How to only clone for the first n-1 cursors?
                let mut new_cursor = c.clone();

                new_cursor.distance += 1;
                new_cursor.position = m;

                match grid[&m] {
                    Content::Key(k) => on_key_found(k, &new_cursor),
                    Content::Door(k) => {
                        let k = k.to_ascii_lowercase();
                        if k != from_key {
                            debug!("Door found: {}", k);
                            new_cursor.doors.push(k);
                        }
                    }
                    _ => (),
                }

                state.insert(m, new_cursor.distance);
                next_cursors.push(new_cursor);
            }
        }

        cursors = next_cursors;
    }

    result
}

fn print_state(grid: &ContentGrid, state_grid: &Grid<u32>, current_pos: Option<Pos>) {
    if !log_enabled!(Level::Debug) {
        return;
    }
    display_grid(grid, current_pos, |pos, s| match s {
        Some(Content::Passage) | None => match state_grid.get(&pos) {
            None => String::from("  "),
            Some(d) => format!("{} ", d % 10),
        },
        Some(Content::Key(k)) => format!("{} ", k),
        Some(Content::Door(k)) => format!("{} ", k),
        Some(Content::Wall) => String::from("██"),
    });
}

fn display_content_grid(grid: &ContentGrid, current_pos: Option<Pos>) {
    display_grid(grid, current_pos, |_pos, s| match s {
        Some(Content::Passage) | None => String::from("  "),
        Some(Content::Key(k)) => format!("{} ", k),
        Some(Content::Door(k)) => format!("{} ", k),
        Some(Content::Wall) => String::from("██"),
    });
}

fn get_neighbouring_positions(pos: Pos) -> NextMoveIterator {
    NextMoveIterator::new(pos)
}

fn display_grid<T>(
    grid: &Grid<T>,
    current_pos: Option<Pos>,
    display: impl Fn(Pos, Option<&T>) -> String,
) {
    if !log_enabled!(Level::Info) {
        return;
    }
    let x_max = grid.keys().map(|Pos(x, _)| *x).max().unwrap();
    let y_max = grid.keys().map(|Pos(_, y)| *y).max().unwrap();

    for y in 0..y_max + 1 {
        for x in 0..x_max + 1 {
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
