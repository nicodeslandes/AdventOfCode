use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
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
    fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.z = 0;
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Body {
    position: Coord,
    velocity: Coord,
    acceleration: Coord,
}

impl Body {
    fn new(position: Coord) -> Body {
        Body {
            position,
            velocity: Coord { x: 0, y: 0, z: 0 },
            acceleration: Coord { x: 0, y: 0, z: 0 },
        }
    }

    fn apply_gravity(&mut self, other: &mut Body) {
        fn apply(
            vel1: &mut i32,
            vel2: &mut i32,
            acc1: &mut i32,
            acc2: &mut i32,
            pos1: i32,
            pos2: i32,
        ) {
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
            &mut self.velocity.x,
            &mut other.velocity.x,
            &mut self.acceleration.x,
            &mut other.acceleration.x,
            self.position.x,
            other.position.x,
        );
        apply(
            &mut self.velocity.y,
            &mut other.velocity.y,
            &mut self.acceleration.y,
            &mut other.acceleration.y,
            self.position.y,
            other.position.y,
        );
        apply(
            &mut self.velocity.z,
            &mut other.velocity.z,
            &mut self.acceleration.z,
            &mut other.acceleration.z,
            self.position.z,
            other.position.z,
        );
    }

    fn apply_velocity(&mut self) {
        self.position.add_mut(&self.velocity);
    }

    fn apply_acceleration(&mut self) {
        self.velocity.add_mut(&self.acceleration);
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
        write!(
            f,
            "pos={}, vel={}, acc={}",
            self.position, self.velocity, self.acceleration
        )
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

    let mut position_hashes: HashSet<Vec<Body>> = HashSet::new();
    let mut hasher = DefaultHasher::new();

    display(&bodies);
    let mut i = 0;
    loop {
        //println!("Step {}", i);
        if !position_hashes.insert(bodies.clone()) {
            println!("Found it! Step: {}", i);
            display(&bodies);
            break;
        }
        step(&mut bodies);

        if i % 1_000_000 == 0 {
            println!("Step {}", i);
            display(&bodies);
        }
        i += 1;
    }

    // let energy = energy(&bodies);
    // println!("Energy: {}", energy);
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
    for i in 0..bodies.len() {
        bodies[i].acceleration.reset();
    }

    for i in 0..bodies.len() - 1 {
        for j in i + 1..bodies.len() {
            let (v1, v2) = bodies.split_at_mut(i + 1);
            let body1 = &mut v1[i];
            let body2 = &mut v2[j - i - 1];
            body1.apply_gravity(body2);
        }
    }

    for i in 0..bodies.len() {
        bodies[i].apply_acceleration();
        bodies[i].apply_velocity();
    }
}

fn get_hash(bodies: &Vec<Body>) -> u64 {
    let mut hasher = DefaultHasher::new();
    for b in bodies {
        hash(&b.position, &mut hasher);
    }

    hasher.finish()
}

fn hash<T: Hash>(t: &T, hasher: &mut DefaultHasher) {
    t.hash(hasher);
}
