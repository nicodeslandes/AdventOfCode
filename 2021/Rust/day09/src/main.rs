use log::{debug, info};
use simplelog::*;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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
    let file = File::open(file_name)?;
    let lines = BufReader::new(&file).lines();

    let grid: Vec<_> = lines
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|ch| ch.to_string().parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    debug!("grid:{:?}", grid);

    let mut minimums: Vec<u32> = vec![];
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
                minimums.push(v);
            }
        }
    }

    info!("Minimums: {:?}", minimums);
    println!("Part 1: {}", minimums.iter().map(|v| v + 1).sum::<u32>());

    Ok(())
}
