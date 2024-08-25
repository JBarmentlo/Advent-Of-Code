use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Hash, Eq, PartialEq, Debug)]
enum Instruction {
    noop,
    addx(i32),
}

#[derive(Debug)]
struct Cpu {
    instructions: VecDeque<Vec<i32>>,
    registers: HashMap<String, i32>,
}

impl Cpu {
    fn Execute(mut self, instruction: Instruction) {
        match instruction {
            Instruction::noop => {},
            Instruction::addx(v) => {
                self.registers
                .entry("x".to_string())
                .and_modify(|x| *x += v)
                .or_insert(v);
            },
        }
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
    let filename = "test_data.txt";
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

    let mut instructions: Vec<Instruction> = Vec::new(); 
    for line in input_lines {
        if let Some(instruction) = parse_line(&line) {
            instructions.push(instruction);
        }
    }

    let mut cpu = Cpu {
        instructions: VecDeque::from([vec![], vec![]]),
        registers: HashMap::from([("x".to_string(), 0)]),
    };

    
    dbg!(instructions);
    Ok(())
}

