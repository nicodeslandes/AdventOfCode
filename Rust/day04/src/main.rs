//use core::cmp::{Eq, PartialEq};
//use core::hash::Hash;
//use std::collections::{HashMap, HashSet};
use std::env;
//use std::fs::File;
//use std::io::{BufRead, BufReader};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let range_start: i32 = env::args().nth(1).unwrap().parse().expect("Enter range start and end");
    let range_end: i32 = env::args().nth(2).unwrap().parse().expect("Enter range start and end");

    println!("Looking for passwords from {} to {}", range_start, range_end);
    test_password(147888);
    test_password(114888);
    test_password(146788);
    test_password(344444);
    test_password(555888);

    let mut count = 0;
    for p in range_start..range_end + 1 {
        if test_password(p) {
            count += 1;
        }
    }

    println!("Password count: {}", count);

    Ok(())
}

fn test_password(password: i32) -> bool {
    if is_valid_password(format!("{}", password).as_str()) {
        println!("{} is a valid password", password);
        true
    } else {
        //println!("{} is not a valid password", password);
        false }
}

fn is_valid_password(str: &str) -> bool {
    let digits: Vec<u32> = str.chars().map(|c| c.to_digit(10).unwrap()).collect();
    //println!("Digits: {:?}", digits);

    let mut has_adjacent_duplicate = false;
    for i in 1..digits.len() {
        if digits[i] < digits[i - 1] { return false; } // Digits must not decrease
        if digits[i] == digits[i - 1]
            && !(i < digits.len() - 1 && digits[i] == digits[i + 1])
            && !(i > 1 && digits[i - 1] == digits[i - 2]) {
            has_adjacent_duplicate = true;
        }
    }

    has_adjacent_duplicate
}