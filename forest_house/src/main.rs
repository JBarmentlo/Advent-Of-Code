use std::fs::File;
use std::io::{BufRead, BufReader};
use grid::Grid;

fn parse_grid(filename: &str) -> Grid<u32> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut data = Vec::new();
    let mut width = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if width == 0 {
            width = line.len();
        }
        data.extend(line.chars().map(|c| c.to_digit(10).unwrap()));
    }

    Grid::from_vec(data, width)
}

fn count_visible_trees(heights: &Grid<u32>) -> usize {
    let rows = heights.rows();
    let cols = heights.cols();
    let mut visible = Grid::init(rows, cols, false);

    // Mark edge trees as visible
    for i in 0..rows {
        visible[(i, 0)] = true;
        visible[(i, cols - 1)] = true;
    }
    for j in 0..cols {
        visible[(0, j)] = true;
        visible[(rows - 1, j)] = true;
    }

    // Check visibility from left and right
    for i in 1..rows - 1 {
        let mut max_left = heights[(i, 0)];
        let mut max_right = heights[(i, cols - 1)];

        for j in 1..cols - 1 {
            if heights[(i, j)] > max_left {
                visible[(i, j)] = true;
                max_left = heights[(i, j)];
            }
            if heights[(i, cols - 1 - j)] > max_right {
                visible[(i, cols - 1 - j)] = true;
                max_right = heights[(i, cols - 1 - j)];
            }
        }
    }

    // Check visibility from top and bottom
    for j in 1..cols - 1 {
        let mut max_top = heights[(0, j)];
        let mut max_bottom = heights[(rows - 1, j)];

        for i in 1..rows - 1 {
            if heights[(i, j)] > max_top {
                visible[(i, j)] = true;
                max_top = heights[(i, j)];
            }
            if heights[(rows - 1 - i, j)] > max_bottom {
                visible[(rows - 1 - i, j)] = true;
                max_bottom = heights[(rows - 1 - i, j)];
            }
        }
    }

    // Count visible trees
    visible.iter().filter(|&&x| x).count()
}

fn calculate_scenic_score(heights: &Grid<u32>, row: usize, col: usize) -> usize {
    let rows = heights.rows();
    let cols = heights.cols();
    let height = heights[(row, col)];

    let mut left = 0;
    for j in (0..col).rev() {
        left += 1;
        if heights[(row, j)] >= height {
            break;
        }
    }

    let mut right = 0;
    for j in col + 1..cols {
        right += 1;
        if heights[(row, j)] >= height {
            break;
        }
    }

    let mut up = 0;
    for i in (0..row).rev() {
        up += 1;
        if heights[(i, col)] >= height {
            break;
        }
    }

    let mut down = 0;
    for i in row + 1..rows {
        down += 1;
        if heights[(i, col)] >= height {
            break;
        }
    }

    left * right * up * down
}

fn highest_scenic_score(heights: &Grid<u32>) -> usize {
    let rows = heights.rows();
    let cols = heights.cols();
    let mut max_score = 0;

    for i in 0..rows {
        for j in 0..cols {
            let score = calculate_scenic_score(heights, i, j);
            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}

fn main() {
    let heights = parse_grid("data.txt");
    let visible_trees = count_visible_trees(&heights);
    println!("Number of visible trees: {}", visible_trees);

    let max_scenic_score = highest_scenic_score(&heights);
    println!("Highest scenic score: {}", max_scenic_score);
}