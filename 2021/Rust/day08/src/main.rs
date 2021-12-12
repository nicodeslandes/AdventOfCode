use log::{debug, info};
use simplelog::*;
use std::collections::HashMap;
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
        .flat_map(|[_, l]| l)
        .filter(|word| known_sizes.contains(&word.len()))
        .count();

    println!("Part 1: {}", part1);

    let part2: u32 = lines
        .iter()
        .map(|[signals, display]| {
            let signals: Vec<_> = signals.iter().map(|s| parse_segments(s)).collect();
            let signal_map = find_segment_associations(&signals);
            display.iter().fold(0, |acc, digit| {
                let digit_segments = parse_segments(digit);
                let value = signal_map[&digit_segments];
                acc * 10 + value
            })
        })
        .sum();
    println!("Part 2: {}", part2);

    Ok(())
}

type Segments = u8;

fn find_segment_associations(signals: &Vec<Segments>) -> HashMap<Segments, u32> {
    let mut result = HashMap::<Segments, u32>::new();
    let mut known_numbers = HashMap::<u32, Segments>::new();

    // First step, identify 1,7,4,9
    for &v in signals {
        let value = match v.count_ones() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        };

        if let Some(x) = value {
            result.insert(v, x);
            known_numbers.insert(x, v);
        }
    }

    let one = known_numbers[&1];
    let four = known_numbers[&4];
    let eight = known_numbers[&8];
    let mut locate_digit =
        |unknown: &mut HashSet<Segments>, value: u32, predicate: &dyn Fn(&u8) -> bool| {
            let found: Vec<_> = unknown.iter().copied().filter(predicate).take(2).collect();
            if found.len() != 1 {
                panic!("Wrong match count for value {}: {:?}", value, found);
            }
            let found = found[0];
            result.insert(found, value);
            known_numbers.insert(value, found);
            unknown.remove(&found);
        };

    // next: find 6-segment digits: 0,6,9
    let mut unknown: HashSet<_> = signals
        .iter()
        .copied()
        .filter(|&v| v.count_ones() == 6)
        .collect();

    locate_digit(&mut unknown, 6, &|&v| v & one != one);
    locate_digit(&mut unknown, 9, &|&v| v & four == four);
    locate_digit(&mut unknown, 0, &|_| true);

    // finally: find 5-segment digits: 2,3,5
    unknown = signals
        .iter()
        .copied()
        .filter(|&v| v.count_ones() == 5)
        .collect();

    locate_digit(&mut unknown, 3, &|&v| v & one == one);
    locate_digit(&mut unknown, 2, &|&v| v | four == eight);
    locate_digit(&mut unknown, 5, &|_| true);

    debug!("Maps: {:?}", result);

    result
}

fn parse_segments(v: &str) -> Segments {
    v.chars().fold(0, |acc, ch| {
        acc + match ch {
            'a' => 1,
            'b' => 2,
            'c' => 4,
            'd' => 8,
            'e' => 16,
            'f' => 32,
            'g' => 64,
            _ => panic!("Nope: {}", ch),
        }
    })
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
