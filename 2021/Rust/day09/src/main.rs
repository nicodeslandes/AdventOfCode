use itertools::Itertools;
use log::{debug, info};
use simplelog::*;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
type Grid<T> = Vec<Vec<T>>;

fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Warn,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;
    let file_name = env::args().nth(1).expect("Enter a file name");

    debug!("Reading input from {}", file_name);
    let file = File::open(file_name)?;
    let lines = BufReader::new(&file).lines();

    let grid: Grid<_> = lines
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|ch| ch.to_string().parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    debug!("grid:{:?}", grid);

    let mut minimums: Vec<(Pos, u32)> = vec![];
    let height = grid.len();
    let width = grid[0].len();
    for y in 0..height {
        for x in 0..width {
            let v = grid[y][x];
            let mut is_min = true;
            for i in [-1, 0, 1] {
                if (x as i32) + i < 0 || ((x as i32) + i) as usize >= width {
                    continue;
                }
                for j in [-1, 0, 1] {
                    if (y as i32) + j < 0 || ((y as i32) + j) as usize >= height || i == 0 && j == 0
                    {
                        continue;
                    }
                    debug!(
                        "Checking {}, {}",
                        ((x as i32) + i) as usize,
                        ((y as i32) + j) as usize
                    );
                    if grid[((y as i32) + j) as usize][((x as i32) + i) as usize] <= v {
                        is_min = false;
                        break;
                    }
                }
                if !is_min {
                    break;
                } else {
                }
            }

            if is_min {
                info!("Minimum at {},{}: {}", x, y, v);
                minimums.push((Pos { x: x, y: y }, v));
            }
        }
    }

    info!("Minimums: {:?}", minimums);
    println!("Part 1: {}", minimums.iter().map(|v| v.1 + 1).sum::<u32>());

    let c: u32 = minimums
        .iter()
        .map(|(p, _)| walk_through_basin(&grid, *p))
        .sorted_by(|x, y| Ord::cmp(y, x))
        .take(3)
        .product();

    println!("Part 2: {}", c);
    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn add(&self, x: i32, y: i32) -> Pos {
        Pos {
            x: (self.x as i32 + x) as usize,
            y: (self.y as i32 + y) as usize,
        }
    }
}
fn walk_through_basin(grid: &Grid<u32>, from: Pos) -> u32 {
    let mut visited: Grid<bool> = grid
        .iter()
        .map(|row| row.iter().map(|_| false).collect())
        .collect();
    let mut count = 0;

    fn visit(pos: Pos, v: u32, grid: &Grid<u32>, visited: &mut Grid<bool>, count: &mut u32) {
        let new_value = grid[pos.y][pos.x];
        if visited[pos.y][pos.x] || new_value == 9 || new_value < v {
            return;
        }
        debug!("Visiting pos {:?}, value: {}", pos, v);
        visited[pos.y][pos.x] = true;
        *count += 1;
        if pos.y > 0 {
            visit(pos.add(0, -1), new_value, grid, visited, count);
        }
        if pos.x > 0 {
            visit(pos.add(-1, 0), new_value, grid, visited, count);
        }
        if pos.y < grid.len() - 1 {
            visit(pos.add(0, 1), new_value, grid, visited, count);
        }
        if pos.x < grid[0].len() - 1 {
            visit(pos.add(1, 0), new_value, grid, visited, count);
        }
    }

    visit(from, grid[from.y][from.x], &grid, &mut visited, &mut count);
    count
}
