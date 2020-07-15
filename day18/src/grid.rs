use crate::Key;
use crate::MainResult;
use crate::Pos;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub enum Content {
    Wall,
    Key(char),
    Door(char),
    Passage,
}

impl Content {
    pub fn get_key(&self) -> Key {
        match *self {
            Content::Key(c) => c,
            _ => panic!("Invalid content type"),
        }
    }
}

pub type Grid<T> = HashMap<Pos, T>;
pub type ContentGrid = Grid<Content>;

pub fn parse_grid(file_name: &str) -> MainResult<(ContentGrid, Pos)> {
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);
    let mut grid: ContentGrid = ContentGrid::new();

    let mut y = 0;
    let mut current_pos = Pos(0, 0);
    loop {
        let mut line = String::new();
        let read = reader.read_line(&mut line)?;
        if read == 0 {
            break;
        }
        for (x, ch) in line.chars().enumerate() {
            let pos = Pos(x, y);
            match ch {
                '#' => {
                    grid.insert(pos, Content::Wall);
                }
                '.' => {
                    grid.insert(pos, Content::Passage);
                }
                '@' => {
                    current_pos = pos;
                    grid.insert(pos, Content::Passage);
                }
                x if x.is_lowercase() => {
                    grid.insert(pos, Content::Key(x));
                }
                x if x.is_uppercase() => {
                    grid.insert(pos, Content::Door(x));
                }
                _ => (),
            }
        }
        y += 1;
    }

    Ok((grid, current_pos))
}
