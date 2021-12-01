use std::env;
use std::fs::File;
use std::io::Read;
use itermore::IterMore;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");

    println!("Reading input from {}", file_name);

    let mut input = String::new();
    File::open(file_name)?
        .read_to_string(&mut input)
        .expect("Failed to read input file");

    let values = input.split_whitespace().map(|line| line.parse::<i32>().unwrap()).collect::<Vec<_>>();

    let count1 = count_increase_by_window(&values, 1);
    let count2 = count_increase_by_window(&values, 3);

    println!("Part 1: {}", count1);
    println!("Part 2: {}", count2);
    Ok(())
}

fn count_increase_by_window(values: &[i32], window_size: usize) -> usize {
    values
        .windows(window_size)
        .map(|w| w.iter().sum())
        .windows()
        .filter(|[a,b]: &[i32;2]| { a < b })
        .count()
}
