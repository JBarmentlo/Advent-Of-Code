use std::{fs, str::Lines};

#[derive(Debug)]
struct Crate {
    pub name: char
}


#[derive(Debug)]
struct Pile {
    crates: Vec<Crate>
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
            '0' => (),
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
}




fn main() {
    let contents = fs::read_to_string("test_data.txt").expect("The file is static and is always parsable");

    let n_stacks   = get_number_of_stacks(&contents);
    let mut port   = Port::with_capacity(n_stacks); 
    dbg!(&port);

    let crates     = get_crates_str(&contents);


    for line in crates.lines() {
        println!("{line}");
        for (i, l) in line.replace("    ", "[0] ").trim().split_whitespace().enumerate() {
            let c = l.chars().nth(1).expect("Fixed size");
            let krat = Crate { name: c };
            port.add(krat, i);
        }
        println!("");
    }
    dbg!(&port);

}

// fn build_piles(lines: &mut Lines, piles: &mut Vec<Pile>) {
//     match lines.next() {
//         None => return,
//         Some(line) => {

//         }
//     }
// }

// fn add_line_to_piles(line: &str, piles: &mut Vec<Pile>) {
//     let mut i = 0;
//     let piles = piles.iter_mut();
    
//     for l in line.replace("    ", "[0] ").trim().split_whitespace() {
//         if l.chars().nth(1).expect("Fixed size").is_alphabetic() {
            
//         }
//         i += 1;
//     }
// }



fn get_number_of_stacks(data: &str) -> usize {
    (data.lines().next().expect("input isnt empty").len() + 1) / 4
}

fn get_crates_str(data: &str) -> String {
    data.lines().filter(|l| l.contains('[')).fold(String::new(), |a, b| a + b + "\n")
}

fn get_operations_str(data: &str) -> String {
    data.lines().filter(|l| l.contains("move")).fold(String::new(), |a, b| a + b + "\n")
}
