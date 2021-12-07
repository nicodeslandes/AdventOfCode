use log::debug;
use simplelog::*;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
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

    let now = Instant::now();
    println!("Part 1: {}", calc_min_fuel1(&positions));
    println!("Duration: {}us", now.elapsed().as_micros());

    let now = Instant::now();
    println!("Part 2: {}", calc_min_fuel2(&positions));
    println!("Duration: {}us", now.elapsed().as_micros());
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

fn calc_min_fuel2(positions: &Vec<usize>) -> usize {
    let mut costs: Vec<_> = positions.iter().map(|v| (*v, v * (v + 1) / 2)).collect();
    let mut prev = 0;

    let n = positions.len();
    let mut min_value = usize::max_value();
    debug!("Init: {:?}", costs);

    for (i, &v) in positions.iter().enumerate() {
        debug!("i: {}, v: {}", i, v);
        for k in prev..v {
            let mut sum = 0;
            // Move all predecessors up by 1
            for j in 0..i {
                let (vj, fj) = costs[j];
                costs[j] = (vj + 1, fj + vj + 1);
                sum += costs[j].1;
            }

            // Move all successors down by 1
            for j in i..n {
                let (vj, fj) = costs[j];
                costs[j] = (vj - 1, fj - vj);
                sum += costs[j].1;
            }

            debug!("{} Sum: {} ({:?})", k, sum, costs);
            min_value = min_value.min(sum);
        }
        prev = v;
    }

    min_value
}
