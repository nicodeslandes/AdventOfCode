use std::fs::File;
use std::io::Read;
use std::{env, io};

fn main() {
    let file_name = env::args().nth(1).expect("Enter a file name");

    println!("Reading input from {}", file_name);

    let mut input = String::new();
    File::open(file_name)
        .unwrap()
        .read_to_string(&mut input)
        .expect("Failed to read input file");

    let values = input.split_whitespace().collect::<Vec<_>>();

    let mut sum: i32 = 0;

    for v in &values {
        let v: i32 = v.parse().expect("Failed to parse value");
        print!("Value: {}", v);

        let fuel = calculate_fuel(v);
        println!(", mass: {}", fuel);
        sum += fuel;
    }

    println!("Result: {}", sum);
}

fn calculate_fuel(mass: i32) -> i32 {
    let mut total_fuel = 0;
    let mut current_mass = mass;
    loop {
        current_mass = current_mass / 3 - 2;
        if current_mass <= 0 {
            break;
        }
        total_fuel += current_mass;
    }

    return total_fuel;
}
