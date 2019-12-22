extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::result::Result;

type MainResult<T> = Result<T, Box<dyn ::std::error::Error>>;
type Deck = Vec<i32>;

#[derive(Debug)]
enum Operation {
    DealWithIncrement(usize),
    DealIntoNewStack,
    Cut(i32),
}

fn main() -> MainResult<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");
    let operations = read_operations(&file_name)?;

    //println!("Operations: {:?}", operations);

    let mut deck: Deck = vec![];
    for i in 0..10007 {
        deck.push(i);
    }

    for op in operations {
        apply_operation(&op, &mut deck);
    }

    //println!("Resulting deck: {:?}", deck);
    let index = deck
        .iter()
        .enumerate()
        .find(|(_, x)| **x == 2019)
        .unwrap()
        .0;
    println!("Result: {}", index);
    Ok(())
}

fn apply_operation(op: &Operation, deck: &mut Deck) {
    match *op {
        Operation::DealIntoNewStack => {
            deck.reverse();
        }
        Operation::Cut(n) => {
            let index = if n >= 0 { n } else { n + deck.len() as i32 } as usize;
            let mut new_deck = vec![0; deck.len()];
            let mut dest_index = 0;
            for i in index..deck.len() {
                new_deck[dest_index] = deck[i];
                dest_index += 1;
            }

            for i in 0..index {
                new_deck[dest_index] = deck[i];
                dest_index += 1;
            }

            deck.copy_from_slice(&new_deck);
        }
        Operation::DealWithIncrement(incr) => {
            let mut new_deck = vec![-1; deck.len()];
            let mut dest_index = 0;
            for i in 0..deck.len() {
                while new_deck[dest_index] != -1 {
                    dest_index += 1;
                    dest_index %= deck.len();
                }

                new_deck[dest_index] = deck[i];
                dest_index += incr;
                dest_index %= deck.len();
            }

            deck.copy_from_slice(&new_deck);
        }
    }
}
fn read_operations(file_name: &str) -> MainResult<Vec<Operation>> {
    let file = File::open(file_name)?;
    let mut operations: Vec<Operation> = vec![];

    let mut reader = BufReader::new(&file);

    let re = Regex::new(
        r#"(?x)(?:deal\s+with\s+increment\s+(\d+))|
        (?:cut\s+(-?\d+))|
        (?:deal into new stack)"#,
    )?;

    loop {
        let mut line = String::new();
        let read = reader.read_line(&mut line)?;
        line = line.trim().to_string();
        if read == 0 {
            break;
        }

        let op = {
            if line == "deal into new stack" {
                Operation::DealIntoNewStack
            } else {
                let capture = re.captures(&line).unwrap();
                if let Some(increment) = capture.get(1) {
                    Operation::DealWithIncrement(increment.as_str().parse().unwrap())
                } else if let Some(n) = capture.get(2) {
                    Operation::Cut(n.as_str().parse().unwrap())
                } else {
                    panic!("Invalid input")
                }
            }
        };

        operations.push(op);
    }

    Ok(operations)
}
