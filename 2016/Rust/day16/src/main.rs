#![feature(generators, generator_trait)]

mod bits;
mod curve;

use bits::Bits;
use curve::Curve;

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
    println!("Data: {}, rev: {}", d, d.rev());
    println!("Data: {}, rev: {}", d.to_string(), d.rev().to_string());

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
    let target_size = 800;

    let b = Bits::parse("10010");
    println!("Bits: {}; reversed: {}", b, b.rev());
    //let mut bits: Vec<char> = "01110110101001000".chars().collect();
    let mut c = Curve::new("100001");
    println!("Curve: {}", c);

    c.expand(5500);
    println!("After expansion: {}", c);
}
