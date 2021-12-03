use std::io::BufRead;
use std::io::BufReader;
use std::env;
use std::fs::File;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");

    println!("Reading input from {}", file_name);
    let file = File::open(file_name)?;
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());
    let bits : Vec<_> =  lines.map(|line| line.chars().map(|ch| if ch == '0' {0} else {1}).collect::<Vec<_>>()).collect();

    let mut bit_counts = vec![0;bits[0].len()];

    for x in &bits {
        for (i, b) in x.iter().enumerate(){
            bit_counts[i] += b;
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for c in bit_counts.iter() {
        let b = if *c < (bits.len() / 2) {0} else{1};
        gamma <<=1;
        gamma += b;

        epsilon <<= 1;
        epsilon += 1-b;
    }
    
    println!("Gamma: {}, Epsilon: {}", gamma, epsilon);
    println!("Part 1: {}", gamma * epsilon);
    Ok(())
}