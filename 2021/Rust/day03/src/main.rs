use std::io::BufRead;
use std::io::BufReader;
use std::env;
use std::fs::File;
use std::io::Seek;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");

    println!("Reading input from {}", file_name);
    let mut file = File::open(file_name)?;

    let width = BufReader::new(&file).lines().nth(0).unwrap().unwrap().chars().count();
    file.rewind()?;
    let lines = BufReader::new(&file).lines().map(|line| line.unwrap());

    let numbers : Vec<_> =  lines
        .map(|line| line.chars().fold(0, |acc, ch| (acc << 1) + if ch == '0' {0} else {1}))
        .collect();

    let count = numbers.len();

    println!("Numbers: {:?}", numbers);
    let gamma = (0..width).rev().fold(0, |acc, i|{
        let mask = 1 << i;
        println!("Mask: {}", mask);
        let count_1: usize = numbers.iter().map(|n| (n&mask) >> i).sum();
        println!("Count: {}", count_1);
        (acc << 1) + if count_1 < count/2 {0} else {1}
    });
    
    let epsilon = (!gamma) & ((1<<width) - 1);

    println!("Gamma: {}, Epsilon: {}", gamma, epsilon);
    println!("Part 1: {}", gamma * epsilon);
    Ok(())
}