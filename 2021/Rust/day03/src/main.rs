use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Seek;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");

    println!("Reading input from {}", file_name);
    let mut file = File::open(file_name)?;

    let width = BufReader::new(&file)
        .lines()
        .nth(0)
        .unwrap()
        .unwrap()
        .chars()
        .count();
    file.rewind()?;
    let lines = BufReader::new(&file).lines().map(|line| line.unwrap());

    let numbers: Vec<_> = lines
        .map(|line| {
            line.chars()
                .fold(0, |acc, ch| (acc << 1) + if ch == '0' { 0 } else { 1 })
        })
        .collect();

    let result1 = get_result1(&numbers, width);
    let result2 = get_result2(&numbers, width);

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
    Ok(())
}

fn get_result1(numbers: &Vec<u32>, width: usize) -> u32 {
    let count = numbers.len() as u32;
    let gamma = (0..width).rev().fold(0, |acc, i| {
        let mask = 1 << i;
        let count_1: u32 = numbers.iter().map(|n| (n & mask) >> i).sum();
        (acc << 1) + if count_1 < count / 2 { 0 } else { 1 }
    });

    let epsilon = (!gamma) & ((1 << width) - 1);
    return gamma * epsilon;
}

fn get_result2(numbers: &Vec<u32>, width: usize) -> u32 {
    let mut remaining_set: HashSet<_> = numbers.iter().copied().collect();

    let mut i = (width - 1) as isize;
    while remaining_set.len() > 1 {
        let mask = 1 << i;
        let count_1: u32 = remaining_set.iter().map(|n| (n & mask) >> i).sum();

        let remaining_count = remaining_set.len() as u32;
        let keep_value = if 2 * count_1 >= remaining_count {
            mask
        } else {
            0
        };

        let removed: Vec<_> = remaining_set
            .iter()
            .copied()
            .filter(|&x| x & mask != keep_value)
            .collect();
        removed.iter().for_each(|x| {
            remaining_set.remove(&x);
        });
        i -= 1;
    }
    let gamma = remaining_set.iter().nth(0).unwrap();
    let mut remaining_set: HashSet<_> = numbers.iter().copied().collect();

    i = (width - 1) as isize;
    while remaining_set.len() > 1 {
        let mask = 1 << i;
        let count_1: u32 = remaining_set.iter().map(|n| (n & mask) >> i).sum();

        let remaining_count = remaining_set.len() as u32;
        let keep_value = if 2 * count_1 < remaining_count {
            mask
        } else {
            0
        };

        let removed: Vec<_> = remaining_set
            .iter()
            .copied()
            .filter(|&x| x & mask != keep_value)
            .collect();
        removed.iter().for_each(|x| {
            remaining_set.remove(&x);
        });
        i -= 1;
    }
    let epsilon = remaining_set.iter().nth(0).unwrap();
    println!("Gamma: {}, Epsilon: {}", gamma, epsilon);
    return gamma * epsilon;
}
