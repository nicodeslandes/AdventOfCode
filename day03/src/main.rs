use std::collections::{HashMap, HashSet};
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

    //println!("Line1: {}", line1);
    //println!("Line2: {}", line2);

    let line1_pos = read_line_positions(line1.trim_end().split(",").collect());
    let line2_pos = read_line_positions(line2.trim_end().split(",").collect());

    //println!("Line1 pos: {:?}", line1_pos);
    //println!("Line2 pos: {:?}", line2_pos);

    let line1_positions: HashSet<(i32, i32)> = line1_pos.keys().map(|x| *x).collect();
    let line2_positions: HashSet<(i32, i32)> = line2_pos.keys().map(|x| *x).collect();

    let closest_intersection = line1_positions
        .intersection(&line2_positions)
        .min_by_key(|pos| line1_pos[*pos] + line2_pos[*pos]);

    match closest_intersection {
        Some((x, y)) => {
            let pos = (*x, *y);
            println!(
                "Closest intersection: {},{}; distance: {}",
                pos.0,
                pos.1,
                line1_pos[&pos] + line2_pos[&pos]
            )
        }
        _ => println!("No intersection found!"),
    }

    Ok(())
}

fn distance_to_origin(pos: (i32, i32)) -> i32 {
    pos.0.abs() + pos.1.abs()
}
fn read_line_positions<'a>(moves: Vec<&'a str>) -> HashMap<(i32, i32), i32> {
    let mut positions = HashMap::new();
    let mut current_pos: (i32, i32) = (0, 0);
    let mut current_length = 0;

    for mov in moves {
        let mut chars = mov.chars();
        let direction = chars.next().expect("Empty move");
        let movement_length: i32 = chars.as_str().parse().expect("Failed to parse move");
        let movement: fn((i32, i32)) -> (i32, i32) = match direction {
            'U' => |(x, y)| (x, y + 1),
            'D' => |(x, y)| (x, y - 1),
            'R' => |(x, y)| (x + 1, y),
            'L' => |(x, y)| (x - 1, y),
            _ => panic!("Unexpected direction"),
        };

        add_positions(
            &mut positions,
            &mut current_pos,
            &mut current_length,
            movement_length,
            movement,
        );
    }
    positions
}

fn add_positions(
    positions: &mut HashMap<(i32, i32), i32>,
    current_pos: &mut (i32, i32),
    current_length: &mut i32,
    length: i32,
    movement: fn((i32, i32)) -> (i32, i32),
) {
    for _ in 0..length {
        *current_pos = movement(*current_pos);
        *current_length += 1;
        if !positions.contains_key(current_pos) {
            positions.insert(*current_pos, *current_length);
        }
    }
}
