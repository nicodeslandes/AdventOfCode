use std::env;
use std::fs::File;
use std::io::Read;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");

    println!("Reading input from {}", file_name);

    let mut input = String::new();
    File::open(file_name)?
        .read_to_string(&mut input)
        .expect("Failed to read input file");

    let values = input.split_whitespace().map(|line| line.parse::<i32>().unwrap()).collect::<Vec<_>>();

    let mut count = 0;
    for (i,&v) in values.iter().enumerate().skip(1) {
        let previous = values[i-1];
        if v > previous {count += 1;}
    }

    println!("Result: {}", count);
    Ok(())
}
