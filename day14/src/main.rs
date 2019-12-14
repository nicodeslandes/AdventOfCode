extern crate num;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");
    let file = File::open(file_name)?;

    let mut reactions: HashMap<char, HashMap<char, u32>> = BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = &line[1..(line.len() - 1)];
            let coords: Vec<i32> = line
                .split(", ")
                .map(|c| c.split('=').nth(1).unwrap().parse().unwrap())
                .collect();
            vec![
                Body::new(coords[0]),
                Body::new(coords[1]),
                Body::new(coords[2]),
            ]
        })
        .collect();

    Ok(())
}
