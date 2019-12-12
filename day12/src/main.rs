extern crate num;

use num::integer;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Body {
    position: i32,
    velocity: i32,
    acceleration: i32,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct AxisState {
    positions: Vec<i32>,
    velocities: Vec<i32>,
}

impl AxisState {
    fn new(bodies: &Vec<Vec<Body>>, axis: usize) -> AxisState {
        AxisState {
            positions: bodies.iter().map(|b| b[axis].position).collect(),
            velocities: bodies.iter().map(|b| b[axis].velocity).collect(),
        }
    }
}

impl Body {
    fn new(position: i32) -> Body {
        Body {
            position,
            velocity: 0,
            acceleration: 0,
        }
    }

    fn apply_gravity(&mut self, other: &mut Body) {
        fn apply(acc1: &mut i32, acc2: &mut i32, pos1: i32, pos2: i32) {
            match pos1.cmp(&pos2) {
                Ordering::Less => {
                    *acc1 += 1;
                    *acc2 -= 1;
                }
                Ordering::Greater => {
                    *acc1 -= 1;
                    *acc2 += 1;
                }
                Ordering::Equal => (),
            }
        }

        apply(
            &mut self.acceleration,
            &mut other.acceleration,
            self.position,
            other.position,
        );
    }

    fn apply_velocity(&mut self) {
        self.position += self.velocity;
    }

    fn apply_acceleration(&mut self) {
        self.velocity += self.acceleration;
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pos={}, vel={}, acc={}",
            self.position, self.velocity, self.acceleration
        )
    }
}

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");
    let file = File::open(file_name)?;

    let mut bodies: Vec<Vec<Body>> = BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = &line[1..(line.len() - 1)];
            let coords: Vec<i32> = line
                .split(", ")
                .map(|c| c.split('=').nth(1).unwrap().parse().unwrap())
                .collect();
            vec![
                Body::new(coords[0]),
                Body::new(coords[1]),
                Body::new(coords[2]),
            ]
        })
        .collect();

    let mut cycle_length_per_axis: Vec<u32> = vec![];

    for axis in 0..3 {
        let mut positions: HashSet<AxisState> = HashSet::new();
        let mut i = 0;
        loop {
            //println!("Step {}", i);
            let state = AxisState::new(&bodies, axis);

            if !positions.insert(state) {
                println!("Found it! Step: {}", i);
                display(&bodies, axis);
                cycle_length_per_axis.push(i);
                break;
            }
            step(&mut bodies, axis);

            if i != 0 && i % 1_000_000 == 0 {
                println!("Step {}", i);
                println!("Hash set size: {}", positions.len());
                display(&bodies, axis);
            }
            i += 1;
        }
    }
    println!(
        "Result: {}",
        cycle_length_per_axis
            .into_iter()
            .map(|x| x as u64)
            .fold(1 as u64, |a, b| integer::lcm(a, b))
    );
    Ok(())
}

fn display(bodies: &Vec<Vec<Body>>, axis: usize) {
    print!("pos: ");
    for body in bodies {
        print!("{} ", body[axis].position);
    }

    print!("vel: ");
    for body in bodies {
        print!("{} ", body[axis].velocity);
    }

    println!();
}

fn step(bodies: &mut Vec<Vec<Body>>, axis: usize) {
    for i in 0..bodies.len() {
        bodies[i][axis].acceleration = 0;
    }

    for i in 0..bodies.len() - 1 {
        for j in i + 1..bodies.len() {
            let (v1, v2) = bodies.split_at_mut(i + 1);
            let body1 = &mut v1[i][axis];
            let body2 = &mut v2[j - i - 1][axis];
            body1.apply_gravity(body2);
        }
    }

    for i in 0..bodies.len() {
        bodies[i][axis].apply_acceleration();
        bodies[i][axis].apply_velocity();
    }
}
