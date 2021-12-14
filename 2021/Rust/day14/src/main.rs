use itermore::IterMore;
use itertools::Itertools;
use log::{debug, info};
use simplelog::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;
    let file_name = env::args().nth(1).expect("Enter a file name");
    let (input, rules) = parse_lines(&file_name)?;
    debug!("Input: {:?}", input);
    debug!("Rules: {:?}", rules);

    println!("Part 1: {}", get_result(&input, &rules, 10));
    println!("Part 2: {}", get_result(&input, &rules, 40));

    Ok(())
}

fn get_result(input: &Vec<char>, rules: &Rules, iterations: usize) -> usize {
    let mut cache: HashMap<(char, char, usize), Counts> = HashMap::new();
    let pairs = input.iter().windows().map(|[&a, &b]| (a, b));

    let mut ccounts = Counts::new();
    for (a, b) in pairs {
        debug!("Looking for count map for {},{}", a, b);
        get_counts(a, b, iterations, &rules, &mut cache);
        let count_map = cache.get(&(a, b, iterations)).unwrap();
        debug!("Result: {:?}", count_map);

        for (&ch, c) in count_map {
            let c = *ccounts.get(&ch).unwrap_or(&0) + c;
            ccounts.insert(ch, if ch == b { c - 1 } else { c });
        }
    }

    if let Some(&last) = input.last() {
        ccounts.insert(last, *ccounts.get(&last).unwrap_or(&0) + 1);
    }

    debug!("Result: {:?}", ccounts);

    let ordered: Vec<_> = ccounts
        .iter()
        .sorted_by(|(_, c1), (_, c2)| Ord::cmp(c1, c2))
        .collect();

    return ordered.last().unwrap().1 - ordered.first().unwrap().1;
}

type Counts = HashMap<char, usize>;

fn get_counts<'a>(
    a: char,
    b: char,
    iterations: usize,
    rules: &Rules,
    cache: &'a mut HashMap<(char, char, usize), Counts>,
) {
    if cache.contains_key(&(a, b, iterations)) {
        return;
    }

    debug!("Computing count map for {},{}, iter: {}", a, b, iterations);
    let key = (a, b, iterations);
    let mut counts = Counts::new();

    if iterations == 0 {
        if a == b {
            counts.insert(a, 2);
        } else {
            counts.insert(a, 1);
            counts.insert(b, 1);
        }
        cache.insert(key, counts);
    } else {
        let &new_char = rules.get(&(a, b)).unwrap();
        get_counts(a, new_char, iterations - 1, rules, cache);
        get_counts(new_char, b, iterations - 1, rules, cache);

        for (&ch, count) in cache.get(&(a, new_char, iterations - 1)).unwrap() {
            let mut c = counts.get(&ch).unwrap_or(&0) + count;
            if ch == new_char {
                c -= 1;
            }
            counts.insert(ch, c);
        }

        for (&ch, count) in cache.get(&(new_char, b, iterations - 1)).unwrap() {
            let mut c = counts.get(&ch).unwrap_or(&0) + count;
            // if ch == new_char {
            //     c -= 1;
            // }
            counts.insert(ch, c);
        }
        cache.insert(key, counts);
    }

    debug!(
        "Resulting count map for {},{}, iter: {}: {:?}",
        a,
        b,
        iterations,
        cache.get(&key).unwrap()
    );
}

fn run_polymerisation(input: &Vec<char>, rules: &Rules, iterations: usize) -> u32 {
    let mut input = input.clone();
    for _ in 0..iterations {
        let last = input[input.len() - 1];
        input = input
            .iter()
            .windows()
            .flat_map(|[&a, &b]| vec![a, rules[&(a, b)]])
            .collect();
        input.push(last);
        info!("Input: {}", input.len());
        //debug!("Input: {}", input.iter().collect::<String>());

        let mut counts: HashMap<char, u32> = HashMap::new();
        for &ch in &input {
            counts.insert(ch, *counts.get(&ch).unwrap_or(&0) + 1);
        }
        debug!(
            "Counts: {:?}",
            counts
                .iter()
                .sorted_by(|(_, c1), (_, c2)| Ord::cmp(c1, c2))
                .collect::<Vec<_>>()
        );
    }

    let mut counts: HashMap<char, u32> = HashMap::new();
    for &ch in &input {
        counts.insert(ch, *counts.get(&ch).unwrap_or(&0) + 1);
    }

    let mut min = u32::max_value();
    let mut max = 0;
    for &count in counts.values() {
        min = min.min(count);
        max = max.max(count);
    }

    return (max - min).try_into().unwrap();
}
type Rules = HashMap<(char, char), char>;

fn parse_lines(file_name: &str) -> Result<(Vec<char>, Rules)> {
    debug!("Reading input from {}", file_name);
    let file = File::open(file_name)?;
    let mut lines = BufReader::new(&file).lines();
    let input = lines.next().unwrap().unwrap();
    let mut rules = Rules::new();
    for l in lines {
        if let [key, value] = l.unwrap().split("->").collect::<Vec<&str>>()[..] {
            if let [a, b] = key.trim().chars().collect::<Vec<_>>()[..] {
                rules.insert((a, b), value.trim().chars().nth(0).unwrap());
            }
        }
    }

    Ok((input.chars().collect(), rules))
}
