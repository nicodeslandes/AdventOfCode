use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
type Grid<T> = Vec<Vec<T>>;
struct Coord {
    x: usize,
    y: usize,
}

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");
    let file = File::open(file_name)?;

    let asteroids: Grid<bool> = BufReader::new(file)
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
    println!("Asteroids grid: {}x{} {:?}", grid_x, grid_y, asteroids);

    let mut count = 0;
    for y in 0..grid_x {
        for x in 0..grid_y {
            if !asteroids[y][x] {
                continue;
            }
            let lines_of_sight =
                compute_line_of_sight_status(&asteroids, grid_x, grid_y, Coord { x, y });
            let hit_count = count_visible_asteroids(&lines_of_sight, grid_x, grid_y);
            println!("Visible count for {},{}: {}", x + 1, y + 1, hit_count);
            if count < hit_count {
                count = hit_count;
            }
        }
    }

    println!("Result: {}", count);
    Ok(())
}

fn count_visible_asteroids(grid: &Grid<LineOfSightStatus>, grid_x: usize, grid_y: usize) -> u32 {
    let mut count = 0;
    for y in 0..grid_x {
        for x in 0..grid_y {
            if let LineOfSightStatus::AsteroidVisible = grid[y][x] {
                count += 1;
            }
        }
    }

    count
}
fn compute_line_of_sight_status(
    grid: &Grid<bool>,
    grid_x: usize,
    grid_y: usize,
    asteroid: Coord,
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
