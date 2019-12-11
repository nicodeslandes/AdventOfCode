use std::env;
use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
type Grid<T> = Vec<Vec<T>>;

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");
    let file = File::open(file_name)?;

    let mut asteroids: Grid<bool> = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c == '#')
                .collect::<Vec<bool>>()
        })
        .collect();

    let grid_x = asteroids[0].len();
    let grid_y = asteroids.len();
    println!("Asteroids grid: {}x{}", grid_x, grid_y);

    Ok(())
}
