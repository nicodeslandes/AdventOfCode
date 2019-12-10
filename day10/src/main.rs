use ordered_float::OrderedFloat;
use std::env;
use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufRead, BufReader};
extern crate ordered_float;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
type Grid<T> = Vec<Vec<T>>;

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");
    let file = File::open(file_name)?;

    let mut asteroids: Grid<bool> = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c == '#')
                .collect::<Vec<bool>>()
        })
        .collect();

    let grid_x = asteroids[0].len();
    let grid_y = asteroids.len();
    println!("Asteroids grid: {}x{}", grid_x, grid_y);

    let mut count = 0;
    let mut found_asteroid = Coord { x: 0, y: 0 };

    for y in 0..grid_x {
        for x in 0..grid_y {
            if !asteroids[y][x] {
                continue;
            }
            let lines_of_sight =
                compute_line_of_sight_status(&asteroids, grid_x, grid_y, &Coord { x, y });
            let hit_count = get_visible_asteroids(&lines_of_sight, grid_x, grid_y).len();
            //println!("Visible count for {},{}: {}", x + 1, y + 1, hit_count);
            if count < hit_count {
                count = hit_count;
                found_asteroid = Coord { x, y };
            }
        }
    }

    let mut destroyed_asteroids = 0;
    while destroyed_asteroids < 200 {
        let lines_of_sight =
            compute_line_of_sight_status(&asteroids, grid_x, grid_y, &found_asteroid);
        let mut hit_asteroids = get_visible_asteroids(&lines_of_sight, grid_x, grid_y);
        println!("Hit asteroids: {}", hit_asteroids.len());
        if hit_asteroids.is_empty() {
            break;
        }

        hit_asteroids.sort_by_key(|pos| OrderedFloat(angle_between(&found_asteroid, pos)));

        println!("Blasting asteroids from {:?}", found_asteroid);
        for hit in hit_asteroids {
            println!(
                "Destroying asteroid {} ({:?}, angle: {})",
                destroyed_asteroids + 1,
                hit,
                angle_between(&found_asteroid, &hit) * 180.0 / PI
            );
            asteroids[hit.y][hit.x] = false;
            destroyed_asteroids += 1;
            if destroyed_asteroids == 200 {
                println!("Found 200th asteroid: {:?}", hit);
                break;
            }
        }
    }

    Ok(())
}

fn angle_between(a: &Coord, b: &Coord) -> f64 {
    let theta = (b.y as f64 - a.y as f64).atan2(b.x as f64 - a.x as f64);
    let alpha = theta + PI / 2.0;
    let alpha_mod = if alpha < 0.0 { alpha + 2.0 * PI } else { alpha };
    //println!("theta: {}; alpha: {}; modulo, {}", theta, alpha, alpha_mod);
    alpha_mod
}

#[test]
fn check() {
    let pos1 = Coord { x: 5, y: 5 };
    let pos2 = Coord { x: 6, y: 5 };
    println!(
        "Angle between {:?} and {:?}: {}",
        pos1,
        pos2,
        angle_between(&pos1, &pos2) * 180.0 / PI
    )
}

#[test]
fn check2() {
    let pos1 = Coord { x: 5, y: 5 };
    let pos2 = Coord { x: 5, y: 3 };
    println!(
        "Angle between {:?} and {:?}: {} rad , {} deg",
        pos1,
        pos2,
        angle_between(&pos1, &pos2),
        angle_between(&pos1, &pos2) * 180.0 / PI
    )
}

#[test]
fn check3() {
    let pos1 = Coord { x: 5, y: 5 };
    let pos2 = Coord { x: 3, y: 3 };
    println!(
        "Angle between {:?} and {:?}: {} rad , {} deg",
        pos1,
        pos2,
        angle_between(&pos1, &pos2),
        angle_between(&pos1, &pos2) * 180.0 / PI
    )
}

fn get_visible_asteroids(
    grid: &Grid<LineOfSightStatus>,
    grid_x: usize,
    grid_y: usize,
) -> Vec<Coord> {
    let mut result = vec![];
    for y in 0..grid_x {
        for x in 0..grid_y {
            if let LineOfSightStatus::AsteroidVisible = grid[y][x] {
                result.push(Coord { x, y });
            }
        }
    }

    result
}

fn compute_line_of_sight_status(
    grid: &Grid<bool>,
    grid_x: usize,
    grid_y: usize,
    asteroid: &Coord,
) -> Grid<LineOfSightStatus> {
    let mut result = Grid::<LineOfSightStatus>::new();
    for _ in 0..grid_y {
        result.push(vec![LineOfSightStatus::Empty; grid_x]);
    }

    // For all the asteroid in the grid
    for y in 0..grid_y {
        for x in 0..grid_x {
            if (x, y) == (asteroid.x, asteroid.y) || !grid[y][x] {
                continue;
            }

            // if the asteroid is already hidden, skip
            match result[y][x] {
                LineOfSightStatus::LineOfSightHidden => continue,
                _ => {
                    // otherwise, mark the position as visible, and hide all the
                    // positions on the line [orig, asteroid)
                    result[y][x] = LineOfSightStatus::AsteroidVisible;
                    let (vector_x, vector_y) =
                        (x as i32 - asteroid.x as i32, y as i32 - asteroid.y as i32);
                    let (dx, dy): (f64, f64) = match vector_x {
                        0 => {
                            // Only move the y coordinate
                            (0.0, if vector_y > 0 { 1.0 } else { -1.0 })
                        }
                        _ => {
                            let dx = if vector_x > 0 { 1.0 } else { -1.0 };
                            (dx, dx as f64 * vector_y as f64 / vector_x as f64)
                        }
                    };

                    let mut n = 0;
                    let mut next_pos = || {
                        n += 1;
                        (x as f64 + (n as f64) * dx, y as f64 + (n as f64) * dy)
                    };
                    let (mut pos_x, mut pos_y) = next_pos();
                    while pos_x >= 0.0
                        && pos_x < grid_x as f64
                        && pos_y >= 0.0
                        && pos_y < grid_y as f64
                    {
                        // if asteroid.x == 4 && asteroid.y == 7 && vector_y == 0 {
                        //     println!("YO");
                        // }

                        // ignore any coordinates that are not on the grid
                        if pos_x.fract() == 0.0 && pos_y.fract() == 0.0 {
                            // Hide any asteroid in that position
                            if grid[pos_y as usize][pos_x as usize] {
                                result[pos_y as usize][pos_x as usize] =
                                    LineOfSightStatus::LineOfSightHidden;
                            }
                        }

                        let (px, py) = next_pos();
                        pos_x = px;
                        pos_y = py;
                    }
                }
            }
        }
    }

    // if asteroid.x == 4 && asteroid.y == 7 {
    //     for y in 0..grid_y {
    //         for x in 0..grid_x {
    //             print!(
    //                 "{}",
    //                 if x == asteroid.x && y == asteroid.y {
    //                     "A"
    //                 } else {
    //                     match result[y][x] {
    //                         LineOfSightStatus::LineOfSightHidden => "O",
    //                         LineOfSightStatus::AsteroidVisible => "X",
    //                         _ => " ",
    //                     }
    //                 }
    //             );
    //         }
    //         println!();
    //     }
    // }

    result
}

#[derive(Clone)]
enum LineOfSightStatus {
    Empty,
    AsteroidVisible,
    LineOfSightHidden,
}
