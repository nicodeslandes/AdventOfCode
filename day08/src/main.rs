use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");

    let mut pixels = String::new();
    File::open(file_name)?
        .read_to_string(&mut pixels)
        .expect("Failed to read input file");

    let pixels: Vec<i32> = pixels
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    const LAYER_LEN: usize = 25 * 6;

    println!(
        "{:?}",
        pixels
            .chunks(LAYER_LEN)
            .map(|l| count_digits(l))
            .min_by_key(|counts| get_count(counts, 0))
            .map(|counts| get_count(&counts, 1) * get_count(&counts, 2))
    );
    Ok(())
}

fn get_count(counts: &HashMap<i32, u32>, digit: i32) -> u32 {
    counts.get(&digit).map(|x| *x).unwrap_or_default()
}

fn count_digits(layer: &[i32]) -> HashMap<i32, u32> {
    let mut digit_counts: HashMap<i32, u32> = HashMap::new();
    for d in layer {
        match digit_counts.get_mut(d) {
            Some(count) => *count += 1,
            None => {
                digit_counts.insert(*d, 1);
            }
        }
    }

    println!("Count for layer {:?}: {:?}", layer, digit_counts);
    digit_counts
}
