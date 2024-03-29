#![feature(generators, generator_trait)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
mod bits;
mod curve;
mod resolver;

use crate::curve::SingleBitCurve;
use bits::Bits;
use curve::Curve;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

fn get_checksum_string(items: &Vec<char>, size: usize) -> String {
    items
        .iter()
        .take(size)
        .fold(String::new(), |s, &ch| format!("{}{}", s, ch))
}

// fn print_disk_old(items: &Vec<char>, size: usize) {
//     println!("{} ({})", get_checksum_string(items, size), size);
// }

fn print_disk(items: &Vec<char>, size: usize) {
    let d = Bits { bits: 13, len: 4 };
    println!("Data: {}, rev: {}", d, d.rev_inv());
    println!("Data: {}, rev: {}", d.to_string(), d.rev_inv().to_string());

    let it = items.iter().take(size);
    let values: Vec<char> = it.map(|&ch| ch).collect();

    let output = "";
    println!("{} ({})", output, size);
}

fn checksum(items: &mut Vec<char>, size: usize) -> String {
    let mut size = size;
    loop {
        // Size is guaranteed to be even here
        print_disk(items, size);

        let mut index_in = 0;
        let mut index_out = 0;
        while index_in < size {
            items[index_out] = if items[index_in] == items[index_in + 1] {
                '1'
            } else {
                '0'
            };

            index_in += 2;
            index_out += 1;
        }
        size /= 2;
        if size % 2 == 1 {
            break;
        }
    }
    // (10000101000010^2).1000010100001
    print_disk(items, size);
    return get_checksum_string(items, size);
}

fn main() {
    // let target_size = 20;
    // let input = "10000";
    let target_size = 35651584;
    let input = "01110110101001000";
    // let b = Bits::parse("010010");
    // println!("Bits: {}; reversed/inversed: {}", b, b.rev_inv());
    //let mut bits: Vec<char> = "01110110101001000".chars().collect();
    // let mut c = Curve::new(input);
    // println!("Curve: {}", c);

    // c.expand_and_trim(target_size);
    // println!("After expansion: {}", c);

    // let result = c.compress();
    // println!("Compressed: {:?}", result);

    //let sbc = SingleBitCurve::new(1024 * 1024);
    // let sbc_string = sbc.fold(String::new(), |acc, d| acc + &format!("{}", d));
    // println!("SBC: {}", sbc_string);

    let mut results = [0_u8; 17];

    resolver::run(input, &mut results, 1);
    println!(
        "Part 1: {}",
        results
            .iter()
            .fold(String::new(), |acc, c| format!("{}{}", acc, c))
    );
    resolver::run(input, &mut results, 2);
    println!(
        "Part 2: {}",
        results
            .iter()
            .fold(String::new(), |acc, c| format!("{}{}", acc, c))
    );

    const RUN_COUNT: u32 = 1_000_000;
    println!("Starting {} runs", RUN_COUNT);
    let now = Instant::now();
    for _ in 1..RUN_COUNT {
        resolver::run(input, &mut results, 2);
    }

    println!(
        "Average time: {}ns",
        now.elapsed().as_nanos() / (RUN_COUNT as u128)
    );
}
