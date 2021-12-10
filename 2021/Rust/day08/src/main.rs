use log::{debug, info};
use simplelog::*;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Warn,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;
    let file_name = env::args().nth(1).expect("Enter a file name");
    let lines = parse_lines(&file_name)?;

    let known_sizes: HashSet<usize> = [2, 4, 3, 7]
        .iter()
        .map(|&x| x.try_into().unwrap())
        .collect();
    info!("sizes: {:?}", known_sizes);
    let part1: usize = lines
        .iter()
        .flat_map(|l| &l[1])
        .filter(|word| known_sizes.contains(&word.len()))
        .count();

    println!("Part 1: {}", part1);
    //println!("Part 2: {}", part2);

    Ok(())
}

fn parse_lines(file_name: &str) -> Result<Vec<[Vec<String>; 2]>> {
    debug!("Reading input from {}", file_name);
    let file = File::open(file_name)?;
    let lines = BufReader::new(&file).lines();

    let result = lines
        .map(|l| {
            l.unwrap()
                .split(" | ")
                .map(|s| {
                    s.split(' ')
                        .map(|ss| ss.to_string())
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|v| v.try_into().unwrap())
        .collect();
    Ok(result)
}
