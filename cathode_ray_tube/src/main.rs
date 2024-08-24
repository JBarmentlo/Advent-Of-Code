use std::{collections::{HashMap, VecDeque}, string};


enum Instruction {
    noop,
    addx(i32),
}

struct Cpu {
    instructions: VecDeque<Vec<i32>>,
    registers: HashMap<String, i32>,
}

fn parse_line(line: &str) -> Option<Instruction> {
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

fn main() {
    let instruction_string = "noop\naddx 3\naddx -5";

    // 1.  Tokenization: Break down the string into parts (instructions)
    // let mut instructions = Vec::new(); 
    let lines = instruction_string.lines(); // For each instruction, you can use `match` for better error handling.

    for line in lines {
        let instruction: Option<Instruction> = parse_line(line);
    }

}

