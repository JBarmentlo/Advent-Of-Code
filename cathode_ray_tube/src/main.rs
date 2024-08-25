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
    queue: VecDeque::<Instruction>,
    instruction: Option<Instruction>,
    waiting_time: Option<usize>,
    x: i32,
    cycle: i32,
}

impl Cpu {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::noop => {println!("Noop ");},
            Instruction::addx(v) => {
                self.x += v;
                println!("Addx {} ", &v);
            },
        }
    }

    fn cycle(&mut self) {
        if let None = self.instruction {
            self.prime();
        }
        if let Some(instruction) = self.instruction {
            match self.waiting_time {
                Some(0) => {
                    panic!("This should never happen");
                }
                Some(1) => {
                    self.execute(instruction);
                    self.instruction = None;
                    self.waiting_time = None;
                },
                Some(v) => {
                    self.waiting_time = Some(v - 1);
                },
                None => {
                    println!("Cycling THE VOID");
                }
            }
        }
        self.cycle += 1;
    }

    fn get_signal_strength(&self) -> i32 {
        self.x * self.cycle
    }

    fn prime(&mut self) {
        match self.instruction {
            None => {
                self.instruction = self.queue.pop_front();
                if let Some(instruction) = self.instruction {
                    println!("Starting {:?}", instruction);
                    self.waiting_time = Some(instruction.get_execution_time());
                }
            },
            
            Some(_) => {
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
    let filename = "test_data_2.txt";
    // let filename = "test_data.txt";
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
    let instructions: VecDeque::<Instruction> = input_lines.filter_map(|l| parse_line(l)).collect();

    let mut signal_strengths: Vec<i32> = Vec::new();

    let mut cpu = Cpu {
        queue: instructions,
        instruction: None,
        waiting_time: None,
        x: 1,
        cycle: 1,
    };
    // cpu.prime();
    // dbg!(&cpu);

    while !cpu.queue.is_empty() {
        println!("Start Cycle {}", cpu.cycle);
        cpu.cycle();
        signal_strengths.push(cpu.get_signal_strength());
        println!("x {}, stren {:?}", cpu.x, cpu.get_signal_strength());
        println!("");
    }
    while let Some(_) = cpu.instruction {
        println!("Start Cycle {}", cpu.cycle);
        cpu.cycle();
        signal_strengths.push(cpu.get_signal_strength());
        println!("x {}, stren {:?}", cpu.x, cpu.get_signal_strength());
        println!("");
    }


    let signal_strengths: Vec<i32> = signal_strengths.iter()
    .enumerate()
    .filter(|(i, _)| (i + 1) % 40 == 20)
    .map(|(_, &val)| val)
    .collect();
    dbg!(&signal_strengths);
    let sum: i32 = signal_strengths.into_iter().sum();
    println!("{}", sum);
    Ok(())
}

