#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

#[derive(Copy, Clone)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

pub fn compute_plan_score(plan: &String) -> u32 {
    plan.lines().map(|line| {
        let mut words = line.split_whitespace();
        match (words.next(), words.next()) {
            (Some(him), Some(me)) => {
                let me = convert_me(me);
                let him = convert_him(him);

                score(compute_outcome(me, him), me)
            },
            _ => panic!("All lines should be parsable"),
        }
    })
    .sum()
}


pub fn compute_plan_score_two(plan: &String) -> u32 {
    plan.lines().map(|line| {
        let mut words = line.split_whitespace();
        match (words.next(), words.next()) {
            (Some(him), Some(me)) => {
                let him = convert_him(him);
                let me = convert_me_two(me, him);

                score(compute_outcome(me, him), me)
            },
            _ => panic!("All lines should be parsable"),
        }
    })
    .sum()
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

fn convert_me_two(letter: &str, him: Shape) -> Shape {
    match letter {
        "X" => match him {
            Shape::Rock    => Shape::Scissor,
            Shape::Paper   => Shape::Rock,
            Shape::Scissor => Shape::Paper,
        },

        "Y" => him,
    
        "Z" => match him {
            Shape::Rock    => Shape::Paper ,
            Shape::Paper   => Shape::Scissor ,
            Shape::Scissor => Shape::Rock ,
        },
        _   => panic!("Incorrect letter used")
    }
} 

fn compute_outcome(me: Shape, him: Shape) -> Outcome{
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


