use std::fs;

struct Crate {
    pub name: char
}

struct Pile {
    crates: Vec<Crate>
}

fn main() {
    let contents = fs::read_to_string("test_data.txt").expect("The file is static and is always parsable");

    let crates     = get_crates_str(&contents);
    let operations = get_operations_str(&contents);
    let n_stacks = get_number_of_stacks(&contents);

    for line in contents.lines() {
        println!("{line}");
        for l in line.replace("    ", "[0] ").trim().split_whitespace() {
            let c = l.chars().nth(1).expect("Fixed size");
            println!("{c}");
        }
    }
}

fn get_number_of_stacks(data: &str) -> usize {
    (data.lines().next().expect("input isnt empty").len() + 1) / 4
}

fn get_crates_str(data: &str) -> String {
    data.lines().filter(|l| l.contains('[')).fold(String::new(), |a, b| a + b + "\n")
}

fn get_operations_str(data: &str) -> String {
    data.lines().filter(|l| l.contains("move")).fold(String::new(), |a, b| a + b + "\n")
}
