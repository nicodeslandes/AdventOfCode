use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::result::Result;

type MainResult<T> = Result<T, Box<dyn ::std::error::Error>>;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Pos(usize, usize);

#[derive(Debug)]
enum Content {
    Wall,
    Passage,
    Portal(String),
}

fn main() -> MainResult<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");
    let file = File::open(file_name)?;

    let mut character_grid: HashMap<Pos, char> = HashMap::new();
    let mut grid: HashMap<Pos, Content> = HashMap::new();

    let mut y = 0;
    loop {
        let mut line = String::new();
        let read = BufReader::new(&file).read_line(&mut line)?;
        if read == 0 {
            break;
        }

        for (x, ch) in line.chars().enumerate() {
            let pos = Pos(x, y);
            character_grid.insert(pos, ch);
        }

        y += 1;
    }

    let x_max = character_grid.keys().map(|Pos(x, _)| *x).max().unwrap();
    let y_max = character_grid.keys().map(|Pos(_, y)| *y).max().unwrap();

    let read_portal_name = |pos1: Pos, pos2: Pos| {
        let chars = vec![character_grid[&pos1], character_grid[&pos2]];
        String::from_iter(chars.into_iter())
    };

    let gen_portal = |pos1: Pos, pos2: Pos| Content::Portal(read_portal_name(pos1, pos2));

    for y in 2..y_max - 1 {
        for x in 2..x_max - 1 {
            let pos = Pos(x, y);
            let content = match character_grid.get(&pos) {
                Some('.') => {
                    if x == 2 {
                        gen_portal(Pos(x - 2, y), Pos(x - 1, y))
                    } else if x == x_max - 1 {
                        gen_portal(Pos(x + 1, y), Pos(x + 2, y))
                    } else if y == 2 {
                        gen_portal(Pos(x, y - 2), Pos(x, y - 1))
                    } else if y == y_max - 1 {
                        gen_portal(Pos(x, y + 1), Pos(x, y + 2))
                    } else {
                        Content::Passage
                    }
                }
                Some('#') => Content::Wall,
                _ => continue,
            };

            grid.insert(pos, content);
        }
    }

    println!("Grid: {:?}", grid);

    Ok(())
}
