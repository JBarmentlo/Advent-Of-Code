use std::{fs, ops::Range};

fn main() {
    let contents = fs::read_to_string("data.txt").expect("The file is static and is always parsable");
    first_half(&contents);
}

fn first_half(contents: &str) {
    let total = contents.lines().filter(|line| {
        let mut pairs = line.split(',');
        let first  = to_range(pairs.next().expect("Exists"));
        let second = to_range(pairs.next().expect("Exists"));
        ((first.start <= second.start) && (first.end >= second.end))
        || ((first.start >= second.start) && (first.end <= second.end))
    })
    .count();
    println!("{total}");
}

fn to_range(pair: &str) -> Range<u32> {
    let mut bounds = pair.split('-');
    let lower_bound = bounds.next().expect("Exists").parse::<u32>().expect("Fixed input");
    let upper_bound = bounds.next().expect("Exists").parse::<u32>().expect("Fixed input");

    lower_bound..upper_bound
}