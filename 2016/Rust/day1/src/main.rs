use simplelog::{TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};
use std::fs::File;
use std::io::Read;
use log::info;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()>  {
    
    TermLogger::init(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)?;

    let result = get_distance("data/input.txt")?;
    println!("Result: {}", result);
    Ok(())
}

struct Pos {
    x: i32,
    y: i32,
    d: Dir,
}

impl Pos {
    fn new() -> Pos {
        Pos { x:0, y:0, d: Dir::N }
    }

    fn distance(&self) -> u32{
        (self.x.abs() + self.y.abs()).try_into().unwrap()
    }

    fn apply_move(mut self, m: &Move) -> Self {
        self.d = match m.r {
            LeftRight::Left => NEXT_DIRS_LEFT[self.d as usize],
            LeftRight::Right => NEXT_DIRS_RIGHT[self.d as usize],
        };

        match self.d {
            Dir::N => self.y += m.d,
            Dir::E => self.x += m.d,
            Dir::S => self.y -= m.d,
            Dir::W => self.x -= m.d,
        }

        info!("New position: {:?}", (self.x, self.y));

        self
    }
}

#[derive(Copy, Clone)]
struct Move {
    r: LeftRight,
    d: i32,
}

static NEXT_DIRS_LEFT: [Dir; 4] = [Dir::S, Dir::W, Dir::N, Dir::E];
static NEXT_DIRS_RIGHT: [Dir; 4] = [Dir::N, Dir::E, Dir::S, Dir::W];

#[derive(Copy, Clone)]
enum Dir { W=0, N, E, S }

#[derive(Copy, Clone)]
enum LeftRight { Left, Right }

fn get_distance(file_name: &str) -> Result<u32> {
    let mut input = String::new();
    File::open(file_name)?
        .read_to_string(&mut input)
        .expect("Failed to read input file");

    let moves = input
        .split(", ")
        .map(|x| {
            let rotation = match x.chars().nth(0) {
                Some('L') => LeftRight::Left,
                Some('R') => LeftRight::Right,
                v => panic!("Unhandled rotation: {}", v.unwrap_or(' '))
            };

            let distance = x[1..].parse().unwrap();
            Move { r: rotation, d: distance }
        });

    let result = moves.fold(Pos::new(), |curr, m| { curr.apply_move(&m) }).distance();
    Ok(result)
}

#[test]
fn test1() -> Result<()> {
    assert_eq!(5, get_distance("data/test1.txt")?);
    Ok(())
}

#[test]
fn test2() -> Result<()> {
    assert_eq!(2, get_distance("data/test2.txt")?);
    Ok(())
}

#[test]
fn test3() -> Result<()> {
    assert_eq!(12, get_distance("data/test3.txt")?);
    Ok(())
}
