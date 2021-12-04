use itermore::IterMore;
use log::debug;
use simplelog::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

type Row = HashSet<u32>;
type Col = HashSet<u32>;
type IndexSet = HashSet<usize>;

#[derive(Debug)]
struct Grid {
    numbers_to_rows: HashMap<u32, IndexSet>,
    numbers_to_columns: HashMap<u32, IndexSet>,
    rows: Vec<Row>,
    cols: Vec<Col>,
}

impl Grid {
    fn new(rows: Vec<Vec<u32>>) -> Grid {
        let mut numbers_to_rows: HashMap<u32, IndexSet> = HashMap::new();
        let mut numbers_to_cols: HashMap<u32, IndexSet> = HashMap::new();
        let mut grid_rows: Vec<Row> = vec![];
        let mut columns: Vec<Col> = (0..5).map(|_| HashSet::new()).collect();

        for (r_i, row) in rows.iter().enumerate() {
            let mut new_row: Row = HashSet::new();
            for (c_i, &n) in row.iter().enumerate() {
                new_row.insert(n);
                columns[c_i].insert(n);

                if !numbers_to_rows.contains_key(&n) {
                    let r = IndexSet::new();
                    numbers_to_rows.insert(n, r);
                }
                numbers_to_rows.get_mut(&n).unwrap().insert(r_i);

                if !numbers_to_cols.contains_key(&n) {
                    let c = IndexSet::new();
                    numbers_to_cols.insert(n, c);
                }
                numbers_to_cols.get_mut(&n).unwrap().insert(c_i);
            }

            grid_rows.push(new_row);
        }

        Grid {
            numbers_to_rows: numbers_to_rows,
            numbers_to_columns: numbers_to_cols,
            rows: grid_rows,
            cols: columns,
        }
    }

    fn punch(&mut self, n: u32) -> u32 {
        if let Some(row_indices) = self.numbers_to_rows.get(&n) {
            for &r_i in row_indices.iter() {
                let row = self.rows.get_mut(r_i).unwrap();
                row.remove(&n);
                if row.len() == 0 {
                    return n * self.sum_all_numbers();
                }
            }
        }

        if let Some(col_indices) = self.numbers_to_columns.get(&n) {
            for &c_i in col_indices.iter() {
                let col = self.cols.get_mut(c_i).unwrap();
                col.remove(&n);
                if col.len() == 0 {
                    return n * self.sum_all_numbers();
                }
            }
        }

        return 0;
    }

    fn sum_all_numbers(&self) -> u32 {
        self.rows.iter().flat_map(|r| r).sum()
    }
}

fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;
    let file_name = env::args().nth(1).expect("Enter a file name");

    debug!("Reading input from {}", file_name);
    let file = File::open(file_name)?;

    let mut lines = BufReader::new(&file).lines();
    let line = lines.nth(0).unwrap()?;
    let draw: Vec<u32> = line.split(',').map(|s| s.parse().unwrap()).collect();
    debug!("Draw: {:?}", &draw);

    let grids_data: Vec<Vec<Vec<u32>>> = lines
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

    let punch_all_grids = || {
        let mut grids: Vec<_> = grids_data.iter().map(|d| Grid::new(d.clone())).collect();
        debug!("Grids: {:?}", grids);

        for &n in &draw {
            for g in grids.iter_mut() {
                let n = g.punch(n);
                if n > 0 {
                    return n;
                }
            }
        }
        return 0;
    };

    let punch_until_last_grid = || {
        let mut grids: Vec<_> = grids_data.iter().map(|d| Grid::new(d.clone())).collect();
        let mut remaining_grids: HashSet<_> = (0..grids.len()).collect();
        for &n in &draw {
            let grid_indices: Vec<_> = remaining_grids.iter().copied().collect();
            for g_i in grid_indices {
                let g = &mut grids[g_i];
                let n = g.punch(n);
                if n > 0 && remaining_grids.remove(&g_i) && remaining_grids.is_empty() {
                    return n;
                }
            }
        }
        return 0;
    };

    println!("Part 1: {}", punch_all_grids());
    println!("Part 2: {}", punch_until_last_grid());
    Ok(())
}
