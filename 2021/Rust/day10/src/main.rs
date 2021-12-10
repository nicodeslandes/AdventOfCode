use log::{debug, info};
use simplelog::*;
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

    let part1: usize = lines
        .iter()
        .map(|l| {
            let mut s: Vec<char> = vec![];
            for ch in l.chars() {
                match ch {
                    '(' | '[' | '<' | '{' => s.push(ch),
                    _ => {
                        let opening = s.pop().unwrap();
                        let expected = match opening {
                            '(' => ')',
                            '[' => ']',
                            '{' => '}',
                            '<' => '>',
                            _ => panic!("What??"),
                        };
                        if ch != expected {
                            return match ch {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' => 25137,
                                _ => panic!("Huh?"),
                            };
                        }
                    }
                }
            }
            0
        })
        .sum();

    let mut scores: Vec<_> = lines
        .iter()
        .map(|l| {
            info!("Line: {}", l);
            let mut s: Vec<char> = vec![];
            for ch in l.chars() {
                match ch {
                    '(' | '[' | '<' | '{' => s.push(ch),
                    _ => {
                        let opening = s.pop().unwrap();
                        let expected = match opening {
                            '(' => ')',
                            '[' => ']',
                            '{' => '}',
                            '<' => '>',
                            _ => panic!("What??"),
                        };
                        if ch != expected {
                            return 0;
                        }
                    }
                }
            }
            s.reverse();
            debug!("Left chars: {:?}", s);
            let score = s.iter().fold(0_u64, |score, ch| {
                score * 5
                    + match ch {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!("Huh? Unkonwn char: {}", ch),
                    }
            });
            info!("Score: {}", score);
            score
        })
        .filter(|&s| s != 0)
        .collect();
    scores.sort();
    let part2 = scores[scores.len() / 2];
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn parse_lines(file_name: &str) -> Result<Vec<String>> {
    debug!("Reading input from {}", file_name);
    let file = File::open(file_name)?;
    let lines = BufReader::new(&file).lines();

    let result = lines.map(|l| l.unwrap()).collect();
    Ok(result)
}
