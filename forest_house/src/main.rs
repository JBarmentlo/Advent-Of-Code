use grid::Grid;
use std::fs::File;
use std::io::{self, Read};

use std::error::Error;
use std::fmt;

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
    let data = read_file_content("./test_data.txt");
    match data {
        Err(err) => {
            println!("Error: {}", err);
            Err(InvalidDataError("Couldn't read input".to_string()))
        },
        Ok(res) => {
            do_the_thang(res)
        }
    }
}


fn read_file_content(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn get_grid_size(data: &String) -> Result<(usize, usize), InvalidDataError> {
    let height = data.lines().count() + 1;
    let width = data.lines().next().ok_or(InvalidDataError("No lines to read".to_string())).map(|line| line.len())?;

    return Ok((height, width))
}

fn do_the_thang(data: String) -> Result<(), InvalidDataError> {
    let (height, width) = get_grid_size(&data)?;
    let data: Vec<u32> = data.chars().filter_map(|c| c.to_digit(10)).collect();
    let height_grid = Grid::from_vec(data, width);
    dbg!(height_grid);
    Ok(())
}
