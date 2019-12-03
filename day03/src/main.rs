use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");

    println!("Reading input from {}", file_name);

    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);

    let mut line1 = String::new();
    let mut line2 = String::new();
    reader.read_line(&mut line1)?;
    reader.read_line(&mut line2)?;

    println!("Line1: {}", line1);
    println!("Line2: {}", line2);

    let mut line1Pos = read_line_positions(line1.trim_end().split(",").collect());
    let mut line2Pos = read_line_positions(line2.trim_end().split(",").collect());

    println!("Line1 pos: {:?}", line1Pos);
    println!("Line2 pos: {:?}", line2Pos);

    Ok(())
}

fn read_line_positions<'a>(moves: Vec<&'a str>) -> HashSet<(i32, i32)> {
    let mut positions = HashSet::new();
    let mut current_pos: (i32, i32) = (0, 0);
    for mov in moves {
        let mut chars = mov.chars();
        let direction = chars.next().expect("Empty move");
        let movement_length: i32 = chars.as_str().parse().expect("Failed to parse move");
        match direction {
            'U' => current_pos.0 += movement_length,
            'D' => current_pos.0 -= movement_length,
            'R' => current_pos.1 += movement_length,
            'L' => current_pos.1 -= movement_length,
            _ => panic!("Unexpected direction"),
        };

        positions.insert(current_pos);
        println!("pos: {:?}", positions);
    }
    positions
}
