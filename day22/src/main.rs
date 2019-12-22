extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::result::Result;

type MainResult<T> = Result<T, Box<dyn ::std::error::Error>>;

#[derive(Debug)]
enum Operation {
    DealWithIncrement(u32),
    DealIntoNewStack,
    Cut(i32),
}

fn main() -> MainResult<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");
    let file = File::open(file_name)?;

    let mut operations: Vec<Operation> = vec![];

    let mut reader = BufReader::new(&file);

    let re = Regex::new(
        r#"(?:deal with increment (\d+))|
        (?:cut (\-?\d+))|
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
                } else if let Some(n) = capture.get(1) {
                    Operation::Cut(n.as_str().parse().unwrap())
                } else {
                    panic!("Invalid input")
                }
            }
        };

        operations.push(op);
    }

    println!("Operations: {:?}", operations);

    Ok(())
}
