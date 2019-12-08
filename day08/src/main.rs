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

    let layers: Vec<_> = pixels.chunks(LAYER_LEN).collect();

    for i in 0..LAYER_LEN {
        // Find the 1 layer that doesn't have a transparent pixel
        // at this position
        let pixel = layers.iter().map(|l| l[i]).find(|x| *x != 2).unwrap();
        print!("{}", if pixel == 0 { " " } else { "█" });
        if i != 0 && i % 25 == 0 {
            println!();
        }
    }

    Ok(())
}
