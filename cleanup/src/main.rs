use std::fs;
use std::ops::RangeInclusive;

use cleanup::{
    to_range,
    overlap,
    included
};

fn main() {
    let contents = fs::read_to_string("data.txt").expect("The file is static and is always parsable");
    first_half(&contents);
    second_half(&contents);
}

fn compute(contents: &str, criterium: fn(&RangeInclusive<u32>, &RangeInclusive<u32>) -> bool) {
    let total = contents.lines().filter(|line| {
        let mut pairs = line.split(',');
        let first  = to_range(pairs.next().expect("Exists"));
        let second = to_range(pairs.next().expect("Exists"));
        
        criterium(&first, &second) || criterium(&second, &first)
    })
    .count();
    println!("{total}");
}

fn first_half(contents: &str) {
    compute(&contents, included)
}

fn second_half(contents: &str) {
    compute(&contents, overlap)
}

