use log::debug;
use simplelog::*;
use std::fs::File;
use std::io::Read;
use std::{cmp, env};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;
    let file_name = env::args().nth(1).expect("Enter a file name");

    debug!("Reading input from {}", file_name);
    let mut file = File::open(file_name)?;
    let mut line = String::new();
    file.read_to_string(&mut line)?;

    let mut positions: Vec<_> = line.trim().split(',').map(|s| s.parse().unwrap()).collect();
    positions.sort();
    debug!(
        "Positions:{}",
        positions
            .iter()
            .fold(String::new(), |acc, v| format!("{} {}", acc, v))
    );

    println!("Part 1: {}", calc_min_fuel1(&positions));
    Ok(())
}
fn calc_min_fuel1(positions: &Vec<usize>) -> usize {
    let mut prev = 0;
    let mut moves: usize = positions.iter().sum();

    let n = positions.len();
    let mut min_value = moves;
    for (i, &v) in positions.iter().enumerate() {
        // Move all predecessors up to v
        moves += i * (v - prev);

        // Move all successors down to v
        moves -= (n - i) * (v - prev);

        min_value = cmp::min(min_value, moves);
        debug!("Moves: {}", moves);
        prev = v;
    }

    min_value
}
