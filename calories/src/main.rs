use std::fs;


fn main() {
    first_half();
    second_half();
}

// TODO: look for elegant immutable solution
fn second_half() {
    let contents = fs::read_to_string("data.txt").expect("The file is static and is always parsable");

    let mut sorted: Vec<u32> = contents.split("\n\n").map(|line_group| {
        line_group.lines()
        .map(|line| {
            line.trim().parse::<u32>().expect("All the lines are parsable")
        })
        .sum()
    })
    .collect();
    
    sorted.sort();
    let total: u32 = sorted.iter().rev().take(3).sum();

    println!("top three: {total}");
}


fn first_half() {
    let contents = fs::read_to_string("data.txt").expect("The file is static and is always parsable");

    let max: u64 = contents.split("\n\n").map(|line_group| {
        line_group.lines()
        .map(|line| {
            line.trim().parse::<u64>().expect("All the lines are parsable")
        })
        .sum()
    })
    .max()
    .expect("Fixed input file garantees that there is a max");
            
    println!("max: {max}");
}
