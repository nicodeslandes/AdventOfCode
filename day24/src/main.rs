use array2d::Array2D;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::result::Result;

type MainResult<T> = Result<T, Box<dyn ::std::error::Error>>;

fn main() -> MainResult<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");
    let file = File::open(file_name)?;

    let mut grid = read_grid_from_file(file)?;

    let mut grid_biodiversities: HashSet<i32> = HashSet::new();
    loop {
        println!("Grid:");
        display_grid(&grid);
        let b = calculate_biodiversity(&grid);
        println!("Biodiversity = {}", b);
        if !grid_biodiversities.insert(b) {
            println!("Found duplicate! Result: {}", b);
            break;
        }

        evolve(&mut grid);
    }

    Ok(())
}

type Grid = Array2D<bool>;

fn evolve(grid: &mut Grid) -> () {
    let original = grid.clone();

    let nb_adjacent_bugs = |x: usize, y: usize| {
        let bug_left = x > 0 && *original.get(x - 1, y).unwrap();
        let bug_right = *original.get(x + 1, y).unwrap_or(&false);
        let bug_top = y > 0 && *original.get(x, y - 1).unwrap();
        let bug_bottom = *original.get(x, y + 1).unwrap_or(&false);
        vec![bug_left, bug_right, bug_top, bug_bottom]
            .into_iter()
            .filter(|x| *x)
            .count()
    };

    for x in 0..grid.column_len() {
        for y in 0..grid.row_len() {
            let adjacent_bug_count = nb_adjacent_bugs(x, y);
            let mut bug = *original.get(x, y).unwrap();
            if bug {
                if adjacent_bug_count != 1 {
                    bug = false
                }
            } else {
                if adjacent_bug_count == 1 || adjacent_bug_count == 2 {
                    bug = true
                }
            }

            grid.set(x, y, bug).unwrap();
        }
    }
}

fn display_grid(grid: &Grid) -> () {
    for row in grid.rows_iter() {
        for &el in row {
            print!("{}", if el { "#" } else { "." });
        }
        println!();
    }
}

fn calculate_biodiversity(grid: &Grid) -> i32 {
    let mut result = 0;
    for (i, &el) in grid.elements_row_major_iter().enumerate() {
        if el {
            result += 1 << i;
        }
    }

    result
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
