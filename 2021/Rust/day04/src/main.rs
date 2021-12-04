use itermore::IterMore;
use log::debug;
use simplelog::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Seek;
use std::rc::Rc;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

type Row = HashSet<u32>;
type Col = HashSet<u32>;
type IndexSet = HashSet<usize>;

struct Grid {
    numbers_to_rows: HashMap<u32, IndexSet>,
    numbers_to_columns: HashMap<u32, IndexSet>,
    rows: Vec<Row>,
    cols: Vec<Col>,
}

impl Grid {
    fn new(rows: Vec<Vec<u32>>) -> Grid {
        let numbers_to_rows: HashMap<u32, IndexSet> = HashMap::new();
        let numbers_to_columns: HashMap<u32, IndexSet> = HashMap::new();

        let columns: Vec<Col> = (0..5).map(|_| HashSet::new()).collect();

        for (r_i, row) in rows.iter().enumerate() {
            let mut new_row: Row = HashSet::new();

            for (c_i, &n) in row.iter().enumerate() {
                new_row.insert(n);
                columns[c_i].insert(n);

                let mut rows = match numbers_to_rows.get_mut(&n) {
                    Some(r) => r,
                    None => {
                        let r = HashSet::new();
                        numbers_to_rows.insert(n, r);
                        &r
                    }
                };
                let rows: Vec<Vec<Rc<u32>>> = rows
                    .iter()
                    .map(|r| Rc::new(r.iter().copied().collect()))
                    .collect();
            }
        }

        Grid {
            numbers_to_rows: HashMap::new(),
            numbers_to_columns: HashMap::new(),
        }
    }
}

fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;
    let file_name = env::args().nth(1).expect("Enter a file name");

    debug!("Reading input from {}", file_name);
    let mut file = File::open(file_name)?;

    let mut lines = BufReader::new(&file).lines();
    let line = lines.nth(0).unwrap()?;
    let draw: Vec<u32> = line.split(',').map(|s| s.parse().unwrap()).collect();
    debug!("Draw: {:?}", &draw);

    let grids: Vec<Vec<Vec<u32>>> = lines
        .map(|l| l.unwrap())
        .chunks()
        .map(|grid_lines: [String; 6]| {
            grid_lines
                .iter()
                .skip(1)
                .map(|l| {
                    l.split(' ')
                        .into_iter()
                        .filter(|s| s.len() != 0)
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect()
        })
        .collect();
    debug!("Grids: {:?}", grids);

    let result1 = 1;

    println!("Part 1: {}", result1);
    Ok(())
}
