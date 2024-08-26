use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Instruction {
    noop,
    addx(i32),
}

impl Instruction {
    fn get_execution_time(&self) -> usize {
        match self {
            Instruction::noop => 1,
            Instruction::addx(_) => 2,
        }
    }
}

#[derive(Debug)]
struct Cpu {
    strengths: Vec<i32>,
    x: i32,
    cycle: i32,
}

impl Cpu {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::noop => {
                self.tick();
                // println!("Noop ");
            },
            Instruction::addx(v) => {
                self.tick();
                self.tick();
                self.x += v;
                // println!("Addx {} ", &v);
            },
        }
    }

    fn tick(&mut self) {
        self.cycle += 1;
        self.strengths.push(self.x * self.cycle);

        // if self.cycle % 40 == 20 {
            // self.strengths.push(self.x * self.cycle)
        // }
    }

}

fn parse_line(line: &String) -> Option<Instruction> {
    let tokens = line.trim().split_whitespace().collect::<Vec<&str>>();  // Split by whitespace (adjust if needed)
    
    if tokens.len() == 0 {
        return None
    }

    match tokens[0] { 
        "addx" => {
            if tokens.len() != 2 {
                println!("Malformded line {}", line);
                return None
            } else {
                return match tokens[1].parse::<i32>() {
                    Ok(parsed_num) => {
                        Some(Instruction::addx(parsed_num))
                    },
                    Err(_) => {
                        println!("Failed to parse string to i32");
                        None
                    } 
                }
            }
        },

        "noop"  => {
            Some(Instruction::noop)
        },

        _  => {
            println!("Malformded line {}", line);
            None
        }
    }
}

fn read_file() -> Result<Vec<String>, io::Error>  {
    // let filename = "test_data_2.txt";
    let filename = "test_data.txt";
    // let filename = "data.txt";
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    Ok(
        reader.lines()
        .filter_map(Result::ok)
        .collect()
    )
}

fn main() -> Result<(), io::Error>{
    let input_lines = read_file()?;
    let input_lines = input_lines.iter();
    // let instructions: VecDeque::<Instruction> = input_lines.filter_map(|l| parse_line(l)).collect();
    
    let mut cpu = Cpu {
        strengths: Vec::new(),
        x: 1,
        cycle: 0,
    };
    input_lines.filter_map(|l| parse_line(l)).for_each(|i| cpu.execute(i));

    let signal_strengths: Vec<i32> = cpu.strengths.iter()
    .enumerate()
    .filter(|(i, _)| (i + 1) % 40 == 20)
    .map(|(_, &val)| val)
    .collect();
    // dbg!(&signal_strengths);
    let sum: i32 = signal_strengths.into_iter().sum();
    println!("{}", sum);
    Ok(())
}

