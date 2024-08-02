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

impl Point {
    fn follow(self, other: Point) -> Point {
        let diff = other - self;

        if diff.x.abs() > 1 || diff.y.abs() > 1 {
            self + Point {
                    x: diff.x.signum(),
                    y: diff.y.signum()
                }
        } else {
            self
        }
    }
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
struct Rope<const T: usize> {
    knots: [Point; T]
}

impl<const T: usize> Rope<T> {
    fn new() -> Self {
        Rope {
            knots: [Point { x: 0, y: 0 }; T],
        }
    }
}


impl<const T: usize> Add<Direction> for Rope<T> {
    type Output = Self;

    fn add(self, direction: Direction) -> Self {
        let mut new_points = self.knots;
    
        new_points[0] = self.knots[0] + direction;
        for i in 1..new_points.len() {
            new_points[i] = new_points[i].follow(new_points[i - 1]);
        }

        Rope {
            knots: new_points
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
    let mut rope: Rope<10> = Rope::<10>::new();

    println!("Starting position: {:?}", rope);

    for (i, &direction) in directions.iter().enumerate() {
        rope = rope + direction;

        println!("Direction: {:?}. New Position {:?}", direction, rope);
        tail_positions.insert(rope.knots[9]);

    }

    // dbg!(&tail_positions);
    println!("Number of tail positions {}", tail_positions.len());
    println!("Number of moves {}", directions.len());
    Ok(())
}