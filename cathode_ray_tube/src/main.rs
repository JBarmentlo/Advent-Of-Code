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
    // registers: HashMap<String, i32>,
    x: i32,
}

impl Cpu {
    fn Execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::noop => {println!("Noop");},
            Instruction::addx(v) => {
                self.x += v;
                println!("Addx {}", &v);
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
    let mut signal_strengths = Vec::new();
    let mut cycle = 1;

    let mut cpu = Cpu {
        instructions: VecDeque::with_capacity(MAX_EXE_TIME),
        x: 1
    };
    for _ in 0..MAX_EXE_TIME {
        cpu.instructions.push_front(Vec::new());
    }

    for line in input_lines {
        signal_strengths.push(cpu.x * cycle);
        println!("Cycle {}, x {}, str {:?}", cycle, cpu.x, signal_strengths.last());
        // dbg!(&line);
        cpu.Cycle();
        cycle += 1;
        if let Some(instruction) = parse_line(&line) {
            if let Some(vec) = cpu.instructions.get_mut(instruction.get_execution_time() - 1) {
                // println!("Pushing instruction {:?} to {}", &instruction, instruction.get_execution_time() - 1);
                vec.push(instruction);
            }
        }
    }

    for _ in 0..(MAX_EXE_TIME - 1) {
        cpu.Cycle();
    }

    let sum: i32 = signal_strengths.iter()
    .enumerate()
    .filter(|(i, _)| (i + 1) % 40 == 20)
    .map(|(_, &val)| val)
    .sum();

    dbg!(&cpu);
    // dbg!(&signal_strengths);
    println!("Sum {}", sum);
    Ok(())
}