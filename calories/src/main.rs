use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("File unreadable");

    let max: u64 = contents.split("\n\n").map(|line_group| {
        line_group.lines()
        .map(|line| {
            line.trim().parse::<u64>().expect("Unparsable line: {line}")
        })
        .sum()
    })
    .max()
    .expect("Should be a max");

                                    
    println!("{max}");
}
