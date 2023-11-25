use std::fs;

use rock_paper_scissors::{
    compute_plan_score,
    compute_plan_score_two,
};

fn main() {
    let contents = fs::read_to_string("data.txt").expect("The file is static and is always parsable");
    
    let score   = compute_plan_score(&contents);
    let score_2 = compute_plan_score_two(&contents);

    println!("Score 1: {score}");
    println!("Score 2: {score_2}");
}