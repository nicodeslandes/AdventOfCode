extern crate num;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;
use std::result::Result;

type MainResult<T> = Result<T, Box<dyn ::std::error::Error>>;

fn main() -> MainResult<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");
    let file = File::open(file_name)?;

    let mut input_orig = String::new();
    BufReader::new(file).read_to_string(&mut input_orig)?;
    let mut input_orig: Vec<i32> = input_orig
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    println!("Input: {:?}", input_orig);

    let mut input = input_orig.clone();
    // for _ in 0..100 {
    //     input = calculate_iteration(&input);
    // }
    println!("Result: {:?}", input);

    input = vec![];
    for i in 0..10_000 {
        input.append(&mut input_orig.clone());
    }

    let offset = (0..7)
        .map(|i| input[i])
        .fold(0, |x: i32, i| x.abs() * 10 + i as i32) as usize;
    println!("Index: {}; total size: {}", offset, input.len());

    for _ in 0..100 {
        print!(".");
        io::stdout().flush().unwrap();

        for i in offset..input.len() - 1 {
            let i = input.len() - 2 - i + offset;
            input[i] = (input[i] + input[i + 1]) % 10;
        }
    }

    // for v in &input {
    //     print!("{}", v);
    // }

    println!();

    let result = (0..8).map(|i| input[i + offset]).fold(0, |x, i| x * 10 + i);

    println!("Result: {}", result);
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
