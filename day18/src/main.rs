use crate::grid::*;
use crate::iterators::*;
use linked_hash_set::LinkedHashSet;
use log::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fmt::{Debug, Display};
use std::rc::Rc;
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
#[derive(Clone)]
struct KeyPath {
    from: Key,
    to: Key,
    distance: u32,
    doors: HashSet<Key>,
}

impl Debug for KeyPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}->{} ({})", self.from, self.to, self.distance)
    }
}

impl Display for KeyPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}->{} ({})", self.from, self.to, self.distance)
    }
}

type PathMap = HashMap<Key, HashMap<Key, Rc<RefCell<KeyPath>>>>;

struct State {
    min_total_distance: u32,
    min_path: LinkedHashSet<Key>,
    current_distance: u32,
    keys: LinkedHashSet<Key>,
    key_count: usize,
    path_map: HashMap<Key, HashMap<Key, Rc<RefCell<KeyPath>>>>,
    iteration_count: u32,
}

impl State {
    fn new(path_map: HashMap<Key, HashMap<Key, Rc<RefCell<KeyPath>>>>) -> State {
        let key_count = path_map.len();
        State {
            min_total_distance: u32::max_value(),
            min_path: LinkedHashSet::new(),
            current_distance: 0,
            keys: LinkedHashSet::new(),
            key_count,
            path_map,
            iteration_count: 0,
        }
    }
}

struct Statics {
    target_keys_to_keypath: HashMap<Key, Vec<Rc<RefCell<KeyPath>>>>,
    doors_to_keypath: HashMap<Key, Vec<Rc<RefCell<KeyPath>>>>,
}

fn main() -> MainResult<()> {
    simple_logger::init().unwrap();
    log::set_max_level(LevelFilter::Info);
    let file_name = env::args().nth(1).expect("Enter a file name");

    let (grid, initial_pos) = parse_grid(&file_name)?;
    display_content_grid(&grid, Some(initial_pos));

    let paths_info = compute_paths(&grid);
    print_keys(&paths_info.path_map);

    let statics = Statics {
        target_keys_to_keypath: paths_info.target_keys_to_keypath,
        doors_to_keypath: paths_info.doors_to_keypath,
    };
    let mut state = State::new(paths_info.path_map);

    info!("Key count: {}", state.key_count);

    // Start with a single choice: '@', with a distance of 0
    let distance = get_min_distance(&statics, &mut state, '@', 0);

    info!("Min distance: {}", distance);
    info!("Path: {:?}", state.min_path);
    Ok(())
}

type KeyPathRefMaps = HashMap<Key, Vec<Rc<RefCell<KeyPath>>>>;

struct PathsInfo {
    path_map: PathMap,
    target_keys_to_keypath: KeyPathRefMaps,
    doors_to_keypath: KeyPathRefMaps,
}

fn compute_paths(grid: &ContentGrid) -> PathsInfo {
    let keys = get_keys(&grid);

    let mut path_map: PathMap = HashMap::new();
    let mut target_keys_to_keypath = KeyPathRefMaps::new();
    let mut doors_to_keypath = KeyPathRefMaps::new();

    for (pos, key) in &keys {
        let paths = get_all_paths_to_keys_from(&grid, *pos);
        debug!("Paths from {}: {:?}", key, paths);

        let mut key_paths = HashMap::new();
        for p in paths {
            let to = p.to;
            let key_path_ref = Rc::from(RefCell::from(p));
            key_paths.insert(to, key_path_ref.clone());
            match target_keys_to_keypath.get_mut(&to) {
                None => {
                    target_keys_to_keypath.insert(to, vec![key_path_ref.clone()]);
                }
                Some(kp) => kp.push(key_path_ref.clone()),
            };

            for &door in &key_path_ref.borrow().doors {
                match doors_to_keypath.get_mut(&door) {
                    None => {
                        doors_to_keypath.insert(door, vec![key_path_ref.clone()]);
                    }
                    Some(kp) => kp.push(key_path_ref.clone()),
                }
            }
        }

        path_map.insert(*key, key_paths);
    }

    PathsInfo {
        path_map,
        target_keys_to_keypath,
        doors_to_keypath,
    }
}

fn get_min_distance(
    statics: &Statics,
    state: &mut State,
    next_key: Key,
    distance_to_key: u32,
) -> u32 {
    state.current_distance += distance_to_key;
    if log_enabled!(Level::Debug) {
        state.keys.insert(next_key);
        debug!(
            "Exploring key {}, distance: {}; current path: {:?} (distance: {})",
            next_key, distance_to_key, state.keys, state.current_distance
        );
        state.keys.pop_back();
    }

    state.iteration_count += 1;
    if state.iteration_count == 10_000_000 {
        state.iteration_count = 0;
        state.keys.insert(next_key);
        info!(
            "Current path: {:?} (distance: {}; min_distance: {}",
            state.keys, state.current_distance, state.min_total_distance
        );
        state.keys.pop_back();
    }

    // If this key takes us past the current min path length, don't go further
    if state.current_distance >= state.min_total_distance {
        state.current_distance -= distance_to_key;
        return state.min_total_distance;
    }

    // Otherwise, update the state
    state.keys.insert(next_key);
    if state.keys.len() == state.key_count {
        info!(
            "New min path found! {:?} distance: {}",
            state.keys, state.current_distance
        );

        state.min_total_distance = state.current_distance;
        state.min_path = state.keys.clone();

        // Revert state changes
        state.current_distance -= distance_to_key;
        state.keys.pop_back();
    } else {
        // "Open" the door for the new key, ie update all the paths that contain it and remove
        // the door from them
        if let Some(key_paths) = statics.doors_to_keypath.get(&next_key) {
            for kp in key_paths {
                kp.borrow_mut().doors.remove(&next_key);
            }
        }

        // Remove the paths going to that key: we don't need them during this call
        let mut removed_key_paths = vec![];
        for key_path in &statics.target_keys_to_keypath[&next_key] {
            let from_key_paths = state.path_map.get_mut(&key_path.borrow().from).unwrap();
            if let Some(kp) = from_key_paths.remove(&next_key) {
                removed_key_paths.push(kp);
            }
        }

        // Explore the possible paths

        // Find out which keys are reachable from the current position and the
        // set of keys we have
        let keys = &state.keys;
        let mut reachable_keys: Vec<_> = state.path_map[&next_key]
            .iter()
            .filter(|(_, key_path)| {
                !keys.contains(&key_path.borrow().to)
                    && key_path
                        .borrow()
                        .doors
                        .iter()
                        .all(|door| keys.contains(door))
            })
            .map(|(_to, kp)| kp)
            .cloned()
            .collect();

        // Now we can choose to continue with any of the reachable keys
        reachable_keys.sort_by_key(|k| k.borrow().distance);
        trace!(
            "Reachable keys: {:?}",
            reachable_keys
                .iter()
                .map(|k| k.borrow())
                .collect::<Vec<_>>()
        );

        for key_path in reachable_keys {
            let key_path = key_path.borrow();
            get_min_distance(statics, state, key_path.to, key_path.distance);
        }

        // Before leaving the function, restore the state
        // 1. Close the door again, ie add the door to all the keypath
        if let Some(key_paths) = statics.doors_to_keypath.get(&next_key) {
            for kp in key_paths {
                kp.borrow_mut().doors.insert(next_key);
            }
        }

        // 2. Add back the paths going to that key
        for key_path in removed_key_paths {
            let from_key_paths = state.path_map.get_mut(&key_path.borrow().from).unwrap();
            let to = key_path.borrow().to;
            from_key_paths.insert(to, key_path);
        }

        // 3. Restore the current_distance
        state.current_distance -= distance_to_key;

        // 4. Restore the key set
        state.keys.pop_back();
    }

    state.min_total_distance
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

type Key = char;

#[derive(Debug, Clone)]
struct Cursor {
    position: Pos,
    distance: u32,
    doors: Vec<char>,
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
        debug!("Found key {}", k);
        result.push(KeyPath {
            from: from_key,
            to: k,
            distance: c.distance,
            doors: c.doors.iter().copied().collect(),
        });
    };

    while !cursors.is_empty() {
        if log_enabled!(Level::Trace) {
            print_state(&grid, &state, None);
        }

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
                    Content::Key(k) => {
                        on_key_found(k, &new_cursor);
                        // Also mark the key as a door, as we don't want to consider that path
                        // before reaching this key
                        new_cursor.doors.push(k);
                    }
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

fn print_keys(path_map: &PathMap) {
    if log_enabled!(Level::Info) {
        let mut keys: Vec<_> = path_map.keys().collect();
        keys.sort();
        for k in keys {
            info!("From key {}", k);
            let key_path_map = &path_map[k];
            let mut to_keys: Vec<_> = key_path_map.keys().collect();
            to_keys.sort_by_key(|k| key_path_map[k].borrow().distance);
            for k in to_keys {
                let p = key_path_map[k].borrow();
                info!(
                    "  {}->{} ({}); Doors: {:?}",
                    p.from, p.to, p.distance, p.doors
                );
            }
        }
    }
}
