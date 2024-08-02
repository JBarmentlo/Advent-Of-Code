use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ops::{Add, Sub, Div, Mul};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Div<i32> for Point {
    type Output = Self;

    fn div(self, other: i32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}




impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, direction: Direction) -> Self {
        self + direction.to_point()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn to_point(self) -> Point {
        match self {
            Direction::Right => Point { x: 1, y: 0 },
            Direction::Up => Point { x: 0, y: 1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Down => Point { x: 0, y: -1 },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Rope {
    tail: Point,
    head: Point,
}

impl Add<Direction> for Rope {
    type Output = Self;

    fn add(self, direction: Direction) -> Self {
        let diff = (self.head + direction - self.tail);
        // dbg!(self.head, self.head + direction, self.tail, &diff);
        if diff.x.abs() > 1 || diff.y.abs() > 1 {
            Self {
                tail: self.tail + Point {
                    x: diff.x.signum(),
                    y: diff.y.signum()
                },
                head: self.head + direction,
            }
        } else {
            Self {
                tail: self.tail,
                head: self.head + direction,
            }
        }

    }
}


fn parse_directions(filename: &str) -> io::Result<Vec<Direction>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut directions = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            continue; // Skip invalid lines
        }

        let direction = match parts[0] {
            "R" => Direction::Right,
            "U" => Direction::Up,
            "L" => Direction::Left,
            "D" => Direction::Down,
            _ => continue, // Skip invalid directions
        };

        let steps: u32 = match parts[1].parse() {
            Ok(n) => n,
            Err(_) => continue, // Skip invalid step counts
        };

        // Add the direction 'steps' number of times
        directions.extend(std::iter::repeat(direction).take(steps as usize));
    }

    Ok(directions)
}

fn main() -> io::Result<()> {
    let directions = parse_directions("data.txt")?;
    let mut tail_positions: HashSet::<Point> = HashSet::new();
    let mut current_position: Rope = Rope {
        tail: Point { x: 0, y: 0 },
        head: Point { x: 0, y: 0 },
    };

    println!("Starting position: {:?}", current_position);

    for (i, &direction) in directions.iter().enumerate() {
        current_position = current_position + direction;

        println!("Direction: {:?}. New Position {:?}", direction, current_position);
        tail_positions.insert(current_position.tail);

    }

    // dbg!(&tail_positions);
    println!("Number of tail positions {}", tail_positions.len());
    println!("Number of moves {}", directions.len());
    Ok(())
}