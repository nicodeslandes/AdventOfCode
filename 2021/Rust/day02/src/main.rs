use std::io::BufRead;
use std::io::BufReader;
use std::env;
use std::fs::File;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

struct State{
    x: i32,
    y: i32,
    aim: i32,
}

impl State {
    fn new() -> State {
        State { x:0, y:0, aim: 0}
    }

    fn apply(mut self, action: Action) -> Self {
        match action {
            Action::Forward(x) => {
                self.x += x;
                self.y += self.aim * x;
            },
            Action::Down(x) => self.aim += x,        
        }

        self
    }

    fn distance(&self) -> i32 {
        self.x * self.y
    }
}

enum Action {
    Forward(i32),
    Down(i32),
}

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");

    println!("Reading input from {}", file_name);

    let file = File::open(file_name)?;
    let state =
        BufReader::new(file).lines()
            .map(|line| match line.unwrap().split_whitespace().collect::<Vec<_>>()[..]{
                ["forward", d] => Action::Forward(d.parse::<i32>().unwrap()),
                ["down", d] => Action::Down(d.parse::<i32>().unwrap()),
                ["up", d] => Action::Down(-d.parse::<i32>().unwrap()),
                _ => panic!("Nope"),
            })
            .fold(State::new(), |state, action| state.apply(action) );

    println!("Part 2: {:?}", state.distance());
    Ok(())
}