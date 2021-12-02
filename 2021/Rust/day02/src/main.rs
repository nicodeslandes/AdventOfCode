use std::io::BufRead;
use std::io::BufReader;
use std::env;
use std::fs::File;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");

    println!("Reading input from {}", file_name);

    let file = File::open(file_name)?;
    let result =
        BufReader::new(file).lines()
            .map(|line| match line.unwrap().split_whitespace().collect::<Vec<_>>()[..]{
                ["forward", d] => (d.parse::<i32>().unwrap(), 0),
                ["down", d] => (0, d.parse::<i32>().unwrap()),
                ["up", d] => (0, -d.parse::<i32>().unwrap()),
                _ => panic!("Nope"),
            })
            .fold((0,0), |(new_x, new_y), (x,y)| (x+new_x, y+new_y) );

    println!("Part 1: {:?}", result.0 * result.1);
    Ok(())
}