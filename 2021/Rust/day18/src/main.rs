use log::debug;
use simplelog::*;
use std::env;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use Number::*;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;
    let file_name = env::args().nth(1).expect("Enter a file name");
    let numbers = parse_lines(&file_name)?;
    debug!("Input: {:?}", numbers);

    // println!("Part 1: {}", get_result(&input, &rules, 10));
    // println!("Part 2: {}", get_result(&input, &rules, 40));

    Ok(())
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Number<'a> {
    Regular(u32),
    Pair(&'a Number<'a>, &'a Number<'a>),
}

impl<'a> Debug for Number<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Regular(v) => v.fmt(f),
            Pair(a, b) => f.debug_tuple("").field(a).field(b).finish(),
        }
    }
}

fn parse_lines(file_name: &str) -> Result<Vec<Number>> {
    debug!("Reading input from {}", file_name);
    let file = File::open(file_name)?;
    let mut lines = BufReader::new(&file).lines();
    let input = lines.next().unwrap().unwrap();
    let mut results: Vec<Number> = vec![];
    results.iter().filter(|o| true);
    for l in lines {
        results.push(parse_number(&l?)?);
    }

    Ok(results)
}

fn parse_number<'a, 'b>(input: &'a str) -> Result<Number<'b>> {
    fn parse_from_index<'a, 'b>(index: usize, input: &'a str) -> Result<Number<'b>> {
        let find_first_index = |start: usize, pred: &dyn Fn(char) -> bool| {
            while start < input.len() && !pred(input.char_at(start)) {
                start += 1;
            }
            start
        };

        if input.chars()[index].is_digit() {
            let end = find_first_index(index, |ch| !ch.is_digit());
            return Regular(input[index..end].parse::<u32>());
        }

        return Ok(Regular(22));
    }

    let chars = input.chars();
    chars.next()
    parse_from_index(0, input)
}
