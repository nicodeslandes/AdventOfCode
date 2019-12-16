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
    let input: Vec<i32> = input
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    println!("Input: {:?}", input);
    Ok(())
}
