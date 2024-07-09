use grid::Grid;
use std::fs::File;
use std::io::{self, Read};

use std::error::Error;
use std::{fmt, usize};

#[derive(Debug)]
struct InvalidDataError(String);

impl fmt::Display for InvalidDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for InvalidDataError {}



fn main() -> Result<(), InvalidDataError> {
    println!("Hello, world!");
    // let data = read_file_content("./data.txt");
    let data = read_file_content("./test_data.txt");
    match data {
        Err(err) => {
            println!("Error: {}", err);
            Err(InvalidDataError("Couldn't read input".to_string()))
        },
        Ok(res) => {
            initialize(res)
        }
    }
}


fn read_file_content(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn get_grid_size(data: &String) -> Result<usize, InvalidDataError> {
    // let height = data.lines().count() + 1;
    let width = data.lines().next().ok_or(InvalidDataError("No lines to read".to_string())).map(|line| line.len())?;

    return Ok(width)
}

fn initialize(data: String) -> Result<(), InvalidDataError> {
    let width = get_grid_size(&data)?;
    let data: Vec<u32> = data.chars().filter_map(|c| c.to_digit(10)).collect();
    let heights = Grid::from_vec(data, width);
    dbg!(&heights);
    let visibles = compoot(heights);
    let total = visibles.flatten().iter().filter(|v| **v).count();
    dbg!(total);
    Ok(())
}

fn compoot(heights: Grid<u32>) -> Grid<bool> {
    let mut visibles = Grid::init(heights.rows(), heights.cols(), false);
    for i in 0..heights.rows() {
        let mut max = None::<u32>;
        let mut last_col = 0;

        for j in 0..heights.cols() {
            match max {
                None => {
                    visibles[(i, j)] = true;
                    max = Some(heights[(i, j)]);
                },
                Some(val) => {
                    if heights[(i, j)] > val {
                        visibles[(i, j)] = true;
                        max = Some(heights[(i, j)]);
                        last_col = j;
                    }
                }
            }
        }
        let mut max = None::<u32>;
        for j in (last_col..heights.cols()).rev() {
            match max {
                None => {
                    visibles[(i, j)] = true;
                    max = Some(heights[(i, j)]);
                },
                Some(val) => {
                    if heights[(i, j)] > val {
                        visibles[(i, j)] = true;
                        max = Some(heights[(i, j)]);
                    }
                }
            }
        }
    }
    
    for j in 0..heights.cols() {
        let mut max = None::<u32>;
        let mut last_row = 0;
        for i in 0..heights.rows() {
            match max {
                None => {
                    visibles[(i, j)] = true;
                    max = Some(heights[(i, j)]);
                },
                Some(val) => {
                    if heights[(i, j)] > val {
                        visibles[(i, j)] = true;
                        max = Some(heights[(i, j)]);
                        last_row = i;
                    }
                }
            }
        }
        let mut max = None::<u32>;
        for i in (last_row..heights.rows()).rev() {
            match max {
                None => {
                    visibles[(i, j)] = true;
                    max = Some(heights[(i, j)]);
                },
                Some(val) => {
                    if heights[(i, j)] > val {
                        visibles[(i, j)] = true;
                        max = Some(heights[(i, j)]);
                    }
                }
            }
        }
    }
    dbg!(&visibles);
    visibles
}
