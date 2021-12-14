use itermore::IterMore;
use log::{debug, info};
use simplelog::*;
use std::collections::HashMap;
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
    let (mut input, rules) = parse_lines(&file_name)?;
    debug!("Input: {:?}", input);
    debug!("Rules: {:?}", rules);
    for _ in 0..10 {
        let last = input[input.len() - 1];
        input = input
            .iter()
            .windows()
            .flat_map(|[&a, &b]| vec![a, rules[&[a, b]]])
            .collect();
        input.push(last);
        info!("Input: {}", input.len());
    }

    let mut counts: HashMap<char, u32> = HashMap::new();
    for &ch in &input {
        counts.insert(ch, *counts.get(&ch).unwrap_or(&0) + 1);
    }

    let mut min = u32::max_value();
    let mut max = 0;
    for &count in counts.values() {
        min = min.min(count);
        max = max.max(count);
    }
    let part2 = 0;
    println!("Part 1: {}", max - min);
    println!("Part 2: {}", part2);

    Ok(())
}

type Rules = HashMap<[char; 2], char>;

fn parse_lines(file_name: &str) -> Result<(Vec<char>, Rules)> {
    debug!("Reading input from {}", file_name);
    let file = File::open(file_name)?;
    let mut lines = BufReader::new(&file).lines();
    let input = lines.next().unwrap().unwrap();
    let mut rules = Rules::new();
    for l in lines {
        if let [key, value] = l.unwrap().split("->").collect::<Vec<&str>>()[..] {
            if let [a, b] = key.trim().chars().collect::<Vec<_>>()[..] {
                rules.insert([a, b], value.trim().chars().nth(0).unwrap());
            }
        }
    }

    Ok((input.chars().collect(), rules))
}
