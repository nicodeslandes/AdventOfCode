use log::debug;
use simplelog::*;
use std::env;
use std::fs::File;
use std::io::Read;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;
    let file_name = env::args().nth(1).expect("Enter a file name");

    debug!("Reading input from {}", file_name);
    let mut file = File::open(file_name)?;
    let mut line = String::new();
    file.read_to_string(&mut line)?;

    let mut fishes = vec![0_u64; 9];
    debug!("Line: {}", line);
    for n in line.trim().split(',').map(|s| s.parse::<usize>().unwrap()) {
        fishes[n] += 1;
    }

    let mut gen_0_index = 0;
    for _ in 0..80 {
        debug!(
            "Counts: {} [{}]",
            fishes.iter().sum::<u64>(),
            (gen_0_index..gen_0_index + 9).fold(String::new(), |acc, v| if acc.is_empty() {
                format!("{}", fishes[v % 9])
            } else {
                format!("{};{}", acc, fishes[v % 9])
            })
        );
        let old_gen0 = gen_0_index;
        gen_0_index += 1;
        gen_0_index %= 9;

        fishes[(gen_0_index + 6) % 9] += fishes[old_gen0];
    }
    let result: u64 = fishes.iter().sum();
    println!("Part 1: {}", result);
    Ok(())
}
