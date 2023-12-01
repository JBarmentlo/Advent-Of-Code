use std::{fs, fmt};

use itertools::Itertools;

#[derive(Debug)]
struct Crate {
    pub name: char
}

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


#[derive(Debug)]
struct Pile {
    crates: Vec<Crate>
}

impl fmt::Display for Pile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // self.crates.
        let str: String = self.crates.iter().map(|c| c.name).collect();
        write!(f, "{str}", )
    }
}

impl Pile {
    fn new() -> Pile {
        Pile {
            crates: Vec::<Crate>::new()
        }
    }

    fn add(&mut self, krat: Crate) {
        self.crates.push(krat);
    }

    fn pop(&mut self) -> Option<Crate> {
        self.crates.pop()
    }
}


#[derive(Debug)]
struct Port {
    piles: Vec<Pile>
}

impl Port {
    fn with_capacity(number_of_piles: usize) -> Port {
        let mut piles = Vec::<Pile>::with_capacity(number_of_piles);
        for _ in 0..number_of_piles {
            piles.push(Pile::new());
        }

        Port {
            piles,
        }
    }

    fn add(&mut self, krat: Crate, index: usize) {
        match krat.name {
            ' ' => (),
            _  => {
                match self.piles.get_mut(index) {
                    Some(pile) => pile.add(krat),
                    None => panic!("Pushing to a pile that doesnt exist")
                }
            }
        }
    }

    fn pop(&mut self, index: usize) -> Option<Crate> {
        match self.piles.get_mut(index) {
            Some(pile) => pile.pop(),
            None => panic!("Pushing to a pile that doesnt exist")
        }
    }

    fn top_of_piles(&self) -> String {
        self.piles.iter().filter(|p| p.crates.len() > 0).map(|p| p.crates.last().expect("filtered the empty ones").name).collect()
    }
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str: String = self.piles.iter().enumerate().map(|(i, p)| format!("{}: {}", i, p)).join("\n");
        write!(f, "{str}")
    }
}



fn main() {

    let contents = fs::read_to_string("data.txt").expect("The file is static and is always parsable");
    first_part(&contents);
}

fn first_part(contents: &str) {
    let mut port = Port::with_capacity(get_number_of_stacks(&contents)); 

    for line in get_crates_str(&contents).iter().rev() {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .map(|(i, c)| {
                port.add(Crate { name: c }, i)
            })
            .count();
    }

    println!("{port}");

    for line in get_operations_str(&contents).lines() {
        let mut words = line.split_whitespace();
        let number: usize = words.nth(1).expect("fixed format input").parse().expect("fixed format input");
        let src   : usize = words.nth(1).expect("fixed format input").parse().expect("fixed format input");
        let dest  : usize = words.nth(1).expect("fixed format input").parse().expect("fixed format input");

        for _ in 0..number {
            let krat = port.pop(src - 1).expect("Popping a non existant crate");
            port.add(krat, dest - 1);
        }
    }
    println!("\n{port}\n");

    let top = port.top_of_piles();
    println!("TOP: {top}");
}




fn get_number_of_stacks(data: &str) -> usize {
    (data.lines().next().expect("input isnt empty").len() + 1) / 4
}

fn get_crates_str(data: &str) -> Vec<String> {
    data.lines().filter(|l| l.contains('[')).map(|s| format!("{} ", s)).collect::<Vec<String>>()
    // .fold(String::new(), |a, b| a + b + "\n")
}

fn get_operations_str(data: &str) -> String {
    data.lines().filter(|l| l.contains("move")).fold(String::new(), |a, b| a + b + "\n")
}
