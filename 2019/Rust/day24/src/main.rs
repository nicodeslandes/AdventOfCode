use array2d::Array2D;
use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::result::Result;

const DISPLAY_GRIDS: bool = false;
const TOTAL_MINUTES: u32 = 200;

type MainResult<T> = Result<T, Box<dyn ::std::error::Error>>;

fn main() -> MainResult<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");
    let file = File::open(file_name)?;

    let grid = read_grid_from_file(file)?;
    let mut grid_space = GridSpace::from(vec![grid]);

    for i in 0..=TOTAL_MINUTES {
        if DISPLAY_GRIDS {
            println!("\n\nGrid Space after {} minutes:", i);
            display_grid_space(&grid_space);
        }
        println!("Bug count after {} min: {}", i, count_bugs(&grid_space));

        evolve(&mut grid_space);
    }

    Ok(())
}

fn count_bugs(grid_space: &GridSpace) -> usize {
    grid_space
        .iter()
        .map(|g| g.elements_row_major_iter().filter(|&x| *x).count())
        .sum()
}

fn display_grid_space(grid_space: &GridSpace) -> () {
    for (depth, grid) in grid_space.iter().enumerate() {
        println!("Depth {}", depth);
        display_grid(grid);
    }
}
type Grid = Array2D<bool>;
type GridSpace = VecDeque<Grid>;

fn evolve(grid_space: &mut GridSpace) -> () {
    let mut original = grid_space.clone();

    // Start with the inner-most level
    // If it has bug adjacent to the middle cell, we need to add a new inside grid level
    let grid = original.front().unwrap();
    if has_bug_around_middle_cell(grid) {
        original.push_front(Grid::filled_with(false, 5, 5));
        grid_space.push_front(Grid::filled_with(false, 5, 5));
    }

    // If the top-most grid has bugs adjacent to the outside, we need to add a new top-most grid level
    let grid = original.back().unwrap();
    if has_bug_adjacent_to_outside(grid) {
        original.push_back(Grid::filled_with(false, 5, 5));
        grid_space.push_back(Grid::filled_with(false, 5, 5));
    }

    let original = original;
    let max_depth = original.len() - 1;

    let get_bug = |x: usize, y: usize, d: usize| match original.get(d) {
        Some(grid) => *grid.get(y, x).unwrap_or(&false),
        None => false,
    };

    let mut set_bug = |x: usize, y: usize, d: usize, bug: bool| {
        if x == 2 && y == 2 {
            panic!(
                "Attempted to add bug at coordinates ({},{}) at depth {} !",
                x, y, d
            );
        }
        grid_space.get_mut(d).unwrap().set(y, x, bug).unwrap();
    };

    let count_bugs_at = |x: usize, y: usize, d: usize| {
        if get_bug(x, y, d) {
            1usize
        } else {
            0usize
        }
    };

    let nb_adjacent_bugs = |x: usize, y: usize, depth: usize| {
        let bugs_left = match (x, y) {
            (0, _) => {
                if depth == max_depth {
                    0
                } else {
                    count_bugs_at(1, 2, depth + 1)
                }
            }
            (3, 2) => {
                if depth == 0 {
                    0
                } else {
                    (0..5).map(|y| count_bugs_at(4, y, depth - 1)).sum()
                }
            }
            (x, y) => count_bugs_at(x - 1, y, depth),
        };
        let bugs_right = match (x, y) {
            (4, _) => {
                if depth == max_depth {
                    0
                } else {
                    count_bugs_at(3, 2, depth + 1)
                }
            }
            (1, 2) => {
                if depth == 0 {
                    0
                } else {
                    (0..5).map(|y| count_bugs_at(0, y, depth - 1)).sum()
                }
            }
            (x, y) => count_bugs_at(x + 1, y, depth),
        };
        let bugs_top = match (x, y) {
            (_, 0) => {
                if depth == max_depth {
                    0
                } else {
                    count_bugs_at(2, 1, depth + 1)
                }
            }
            (2, 3) => {
                if depth == 0 {
                    0
                } else {
                    (0..5).map(|x| count_bugs_at(x, 4, depth - 1)).sum()
                }
            }
            (x, y) => count_bugs_at(x, y - 1, depth),
        };
        let bugs_bottom = match (x, y) {
            (_, 4) => {
                if depth == max_depth {
                    0
                } else {
                    count_bugs_at(2, 3, depth + 1)
                }
            }
            (2, 1) => {
                if depth == 0 {
                    0
                } else {
                    (0..5).map(|x| count_bugs_at(x, 0, depth - 1)).sum()
                }
            }
            (x, y) => count_bugs_at(x, y + 1, depth),
        };
        vec![bugs_left, bugs_right, bugs_top, bugs_bottom]
            .into_iter()
            .sum()
    };

    for depth in 0..original.len() {
        for x in 0..5 {
            for y in 0..5 {
                if x == 2 && y == 2 {
                    continue;
                }

                let adjacent_bug_count: usize = nb_adjacent_bugs(x, y, depth);
                let mut bug = get_bug(x, y, depth);
                if bug {
                    if adjacent_bug_count != 1 {
                        bug = false
                    }
                } else {
                    if adjacent_bug_count == 1 || adjacent_bug_count == 2 {
                        bug = true
                    }
                }

                set_bug(x, y, depth, bug);
            }
        }
    }
}

fn has_bug_around_middle_cell(grid: &Grid) -> bool {
    for x in 1..4 {
        for y in 1..4 {
            if x != 2 && y != 2 && *grid.get(x, y).unwrap() {
                return true;
            }
        }
    }

    return false;
}

fn has_bug_adjacent_to_outside(grid: &Grid) -> bool {
    for y in 0..5 {
        if *grid.get(0, y).unwrap() || *grid.get(4, y).unwrap() {
            return true;
        }
    }
    for x in 0..5 {
        if *grid.get(x, 0).unwrap() || *grid.get(x, 4).unwrap() {
            return true;
        }
    }

    return false;
}

fn display_grid(grid: &Grid) -> () {
    for y in 0..grid.row_len() {
        for x in 0..grid.column_len() {
            print!(
                "{}",
                if (x, y) == (2, 2) {
                    "?"
                } else {
                    if *grid.get(y, x).unwrap() {
                        "#"
                    } else {
                        "."
                    }
                }
            );
        }
        println!();
    }
}

fn read_grid_from_file(file: File) -> Result<Grid, Box<dyn Error>> {
    let mut reader = BufReader::new(&file);
    let mut parsed_grid: Vec<bool> = vec![];
    loop {
        let mut line = String::new();
        let read = reader.read_line(&mut line)?;
        if read == 0 {
            break;
        }

        for ch in line.chars() {
            if ch != '\n' && ch != '\r' {
                parsed_grid.push(ch == '#');
            }
        }
    }

    return Ok(Grid::from_row_major(&parsed_grid, 5, 5));
}
