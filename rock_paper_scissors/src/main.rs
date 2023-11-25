use std::fs;

enum Shape {
    Rock,
    Paper,
    Scissor,
}

enum Outcome {
    Win,
    Loss,
    Draw,
}



fn main() {
    let contents = fs::read_to_string("test_data.txt").expect("The file is static and is always parsable");
    
    let mut total_score = 0;
    for line in contents.lines() {
        let mut words = line.split_whitespace();
        match (words.next(), words.next()) {
            (Some(him), Some(me)) => {
                let me = convert_me(me);
                let him = convert_him(him);

                total_score += score(compute_outcome(&me, &him), me);
            },
            _ => panic!("All lines should be parsable"),
        }
    }
    println!("Total score {total_score}");
}

fn convert_him(letter: &str) -> Shape {
    match letter {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissor,
        _   => panic!("Incorrect letter used")
    }
} 
  

fn convert_me(letter: &str) -> Shape {
    match letter {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissor,
        _   => panic!("Incorrect letter used")
    }
} 
  


fn compute_outcome(me: &Shape, him: &Shape) -> Outcome{
    match me {
        Shape::Rock    => {
            match him {
                Shape::Rock    => Outcome::Draw,
                Shape::Paper   => Outcome::Loss,
                Shape::Scissor => Outcome::Win,
            }
        },

        Shape::Paper   => {
            match him {
                Shape::Rock    => Outcome::Win,
                Shape::Paper   => Outcome::Draw,
                Shape::Scissor => Outcome::Loss,
            }
        },

        Shape::Scissor => {
            match him {
                Shape::Rock    => Outcome::Loss,
                Shape::Paper   => Outcome::Win,
                Shape::Scissor => Outcome::Draw,
            }
        },
    }
}


fn score(outcome: Outcome, shape: Shape) -> u32 {
    score_outcome(outcome) + score_shape(shape)
}

fn score_outcome(outcome: Outcome) -> u32 {
    match outcome {
        Outcome::Win  => 6,
        Outcome::Draw => 3,
        Outcome::Loss => 0,
    }
}

fn score_shape(shape: Shape) -> u32 {
    match shape {
        Shape::Rock    => 1,
        Shape::Paper   => 2,
        Shape::Scissor => 3,
    }
}


