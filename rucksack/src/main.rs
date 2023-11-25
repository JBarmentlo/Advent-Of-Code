use std::fs;

// Fairly ugly
fn main() {
    let contents = fs::read_to_string("data.txt").expect("The file is static and is always parsable");
    first_half(&contents);
}

fn first_half(text: &str) {
    let alphabet = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut total_priority = 0;

    'line_loop: for line in text.lines() {
        assert!(line.len() % 2 == 0);
        let first_half = &line[0..(line.len() / 2)];
        let second_half = &line[(line.len() / 2)..line.len()];

        for c in first_half.chars() {
            if second_half.chars().filter(|elt| *elt == c).count() > 0 {
                total_priority += alphabet.find(c).expect("fixed formatting input");
                continue 'line_loop;
            }
        }
    }

    println!("Prio: {total_priority}");
}

fn second_half(text: &str) {
    let alphabet = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    for line_group in text.split("\n\n") {
        let mut lines = line_group.lines();
        let first  = lines.next().expect("Fixed input format");
        let second = lines.next().expect("Fixed input format");
        let third  = lines.next().expect("Fixed input format");

        first.chars().filter(|c| second.contains(*c) && third.contains(*c)).next().expect("Exists as per exercise rules");
    }
}