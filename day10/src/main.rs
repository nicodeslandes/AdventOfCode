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
                    let (mut pos_x, mut pos_y) = (x as i32 + vector_x, y as i32 + vector_y);
                    while pos_x >= 0 && pos_x < grid_x as i32 && pos_y >= 0 && pos_y < grid_y as i32
                    {
                        // Hide any asteroid in that position
                        if grid[pos_y as usize][pos_x as usize] {
                            result[pos_y as usize][pos_x as usize] =
                                LineOfSightStatus::LineOfSightHidden;
                        }
                        pos_x += vector_x;
                        pos_y += vector_y;
                    }
                }
            }
        }
    }

    result
}

#[derive(Clone)]
enum LineOfSightStatus {
    Empty,
    AsteroidVisible,
    LineOfSightHidden,
}
