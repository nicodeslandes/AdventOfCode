extern crate num;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::result::Result;

type MainResult<T> = Result<T, Box<dyn ::std::error::Error>>;

fn main() -> MainResult<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");
    let file = File::open(file_name)?;

    let mut input = String::new();
    BufReader::new(file).read_to_string(&mut input)?;
    let mut input: Vec<i32> = input
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    println!("Input: {:?}", input);

    for _ in 0..100 {
        input = calculate_iteration(&input);
    }
    println!("Input: {:?}", input);
    Ok(())
}

struct Pattern {
    current_index: usize,
    repeat_len: usize,
    pattern_values: Vec<i32>,
}

impl Pattern {
    fn new(repeat_len: usize) -> Pattern {
        Pattern {
            current_index: 0,
            repeat_len,
            pattern_values: vec![0, 1, 0, -1],
        }
    }
}

impl Iterator for Pattern {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.current_index += 1;
        Some(
            self.pattern_values[(self.current_index / self.repeat_len) % self.pattern_values.len()],
        )
    }
}

fn calculate_iteration(input: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for i in 0..input.len() {
        let pattern = Pattern::new(i + 1);

        let r: i32 = input.iter().zip(pattern).map(|(v, p)| v * p).sum();
        result.push((r % 10).abs());
    }

    result
}
