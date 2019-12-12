use std::cmp::Ordering;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    fn add_mut(&mut self, other: &Coord) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    fn sum_abs(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Debug)]
struct Body {
    position: Coord,
    velocity: Coord,
    id: usize,
}

impl Body {
    fn new(position: Coord) -> Body {
        Body {
            id: 0,
            position,
            velocity: Coord { x: 0, y: 0, z: 0 },
        }
    }

    fn apply_gravity(&mut self, other: &mut Body) {
        fn apply(vel1: &mut i32, vel2: &mut i32, pos1: i32, pos2: i32) {
            match pos1.cmp(&pos2) {
                Ordering::Less => {
                    *vel1 += 1;
                    *vel2 -= 1;
                }
                Ordering::Greater => {
                    *vel1 -= 1;
                    *vel2 += 1;
                }
                Ordering::Equal => (),
            }
        }

        apply(
            &mut self.velocity.x,
            &mut other.velocity.x,
            self.position.x,
            other.position.x,
        );
        apply(
            &mut self.velocity.y,
            &mut other.velocity.y,
            self.position.y,
            other.position.y,
        );
        apply(
            &mut self.velocity.z,
            &mut other.velocity.z,
            self.position.z,
            other.position.z,
        );
    }

    fn apply_velocity(&mut self) {
        self.position.add_mut(&self.velocity);
    }

    fn energy(&self) -> i32 {
        let pot = self.position.sum_abs();
        let kin = self.velocity.sum_abs();
        println!("pot: {}, vel: {}", pot, kin);
        pot * kin
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos={}, vel={}", self.position, self.velocity)
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<x={:3}, y={:3}, z={:3}>", self.x, self.y, self.z)
    }
}

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");
    let file = File::open(file_name)?;

    let mut bodies: Vec<Body> = BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = &line[1..(line.len() - 1)];
            let coords: Vec<i32> = line
                .split(", ")
                .map(|c| c.split('=').nth(1).unwrap().parse().unwrap())
                .collect();
            let coord = Coord {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            };

            Body::new(coord)
        })
        .collect();

    display(&bodies);
    for _ in 0..1000 {
        step(&mut bodies);
        //display(&bodies);
    }

    let energy = energy(&bodies);
    println!("Energy: {}", energy);
    Ok(())
}

fn energy(bodies: &Vec<Body>) -> i32 {
    bodies.iter().map(|b| b.energy()).sum()
}
fn display(bodies: &Vec<Body>) {
    for body in bodies {
        println!("{}", body);
    }

    println!();
}

fn step(bodies: &mut Vec<Body>) {
    for i in 0..bodies.len() - 1 {
        for j in i + 1..bodies.len() {
            let (v1, v2) = bodies.split_at_mut(i + 1);
            let body1 = &mut v1[i];
            let body2 = &mut v2[j - i - 1];
            body1.apply_gravity(body2);
        }
    }

    for i in 0..bodies.len() {
        bodies[i].apply_velocity();
    }
}
