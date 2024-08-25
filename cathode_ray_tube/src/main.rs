use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const MAX_EXE_TIME: usize = 2;


#[derive(Hash, Eq, PartialEq, Debug)]
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
    instructions: VecDeque<Vec<Instruction>>,
    registers: HashMap<String, i32>,
}

impl Cpu {
    fn Execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::noop => {println!("Noop");},
            Instruction::addx(v) => {
                let new_val = self.registers
                .entry("x".to_string())
                .and_modify(|x| *x += v)
                .or_insert(v);
                println!("Addx {}. X: {}", &v, new_val);
            },
        }
    }

    fn Cycle(&mut self) {
        if let Some(instructions) = self.instructions.pop_front() {
            instructions.into_iter().for_each(|i| self.Execute(i));
            self.instructions.push_back(Vec::new());
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
    let input_lines = input_lines.iter();

    let mut cpu = Cpu {
        instructions: VecDeque::with_capacity(MAX_EXE_TIME),
        registers: HashMap::from([("x".to_string(), 1)]),
    };
    for _ in 0..MAX_EXE_TIME {
        cpu.instructions.push_front(Vec::new());
    }

    for line in input_lines {
        // dbg!(&line);
        if let Some(instruction) = parse_line(&line) {
            if let Some(vec) = cpu.instructions.get_mut(instruction.get_execution_time() - 1) {
                println!("Pushing instruction {:?} to {}", &instruction, instruction.get_execution_time() - 1);
                vec.push(instruction);
            }
        }
        cpu.Cycle();
        // dbg!(&cpu);
    }

    for _ in 0..(MAX_EXE_TIME - 1) {
        cpu.Cycle();
    }

    dbg!(&cpu);
    Ok(())
}

