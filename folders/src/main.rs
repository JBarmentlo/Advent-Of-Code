use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::fs;
use std::io::Cursor;
use std::iter::Filter;

fn main() { 
    println!("Hello, world!");
    let contents = fs::read_to_string("test_data.txt").expect("The file is static and is always parsable");
    // let contents = fs::read_to_string("data.txt").expect("The file is static and is always parsable");
    let mut root = parse(&contents);
    let mut target_list = vec!["root".to_string(), "a".to_string()];
    let target_name = target_list.pop().expect("");
    let root = get_mut_recursive(target_list.iter(), &mut root).get(&target_name).expect("msg");
    dbg!(target_name, &root);
    let sum = sum_larger(&root, 100000);
    dbg!(sum);
}

#[derive(Debug)]
enum Fuck { 
    File(u32),
    Folder(HashMap<String, Fuck>)
}

fn println_recurse(depth: u32, text: &String) {
    for i in [0..depth] {
        print!("  ")
    }
    println!("{text}");
}

#[derive(Debug, Default)]
struct SizeCounter {
    total: u32,
    counted: u32
}

fn sum_larger(root: &Fuck, max_limit: u32) -> SizeCounter {
    match root {
        Fuck::File(size) => {
            SizeCounter{
                total: *size, 
                counted: 0
            }
        },
        Fuck::Folder(map) => {
            let mut out = map.iter()
            .map(|(name, f)| sum_larger(f, max_limit))
            .fold(
                SizeCounter::default(), 
                |a, b| SizeCounter{
                    total: a.total + b.total,
                    counted: a.counted + b.counted,
                }
            );
            if out.total < max_limit {
                out.counted = out.counted + out.total;
            }

            out
        }
    }
}

fn get_mut_recursive<'a>(mut names: impl Iterator<Item=&'a String>, fuck: &mut Fuck) -> &mut HashMap<String, Fuck> {
    match names.next() {
        None => {
            if let Fuck::Folder(ref mut map) = fuck {
                map
            } else {
                panic!("fuck")
            }
        },
        Some(name) => {
            if let Fuck::Folder(map) = fuck {
                get_mut_recursive(names, map.get_mut(name).expect("Fuck"))
            } else {
                panic!("fuck")
            }
        }
    }
}

fn parse(text: &String) -> Fuck {
    let mut root_fuck = Fuck::Folder(HashMap::new());
    // let mut current_fucks: Vec<&Fuck> = Vec::new();
    // current_fucks.push(&mut root_fuck);
    let mut current_fucks: Vec<String> = Vec::new();
    get_mut_recursive(current_fucks.iter(), &mut root_fuck).insert("root".to_string(), Fuck::Folder(HashMap::new()));
    current_fucks.push("root".to_string());


    let blocks = text.split("$ ")
        .skip(2);

    for block in blocks {
        let mut lines = block.lines();
        let command_line = lines.next().expect("every block has a first line").trim();
        let mut command_line_words = command_line.split_whitespace();
        let cmd = command_line_words.next().expect("always here");
        let arg = command_line_words.next();
        let respones_lines = lines;

        match cmd {
            "cd" => {
                let arg = arg.expect("always arg for cd");
                if arg == ".." {
                    current_fucks.pop();
                } else if arg == "/" {
                    current_fucks = vec!["root".to_string()];
                } else {
                    let map = get_mut_recursive(current_fucks.iter(), &mut root_fuck);
                    map.entry(arg.to_string()).or_insert(Fuck::Folder(HashMap::new()));
                    current_fucks.push(arg.to_string());
                }
            },
            
            "ls" => {
                for line in respones_lines {
                    let trimmed_line = line.trim();
                    
                    if !trimmed_line.starts_with("dir") {
                        let mut words = trimmed_line.split_whitespace();
                        let file_size: u32 = words.next().unwrap().parse().expect("Std format expected");
                        let file_name = words.next().unwrap();
                        let map = get_mut_recursive(current_fucks.iter(), &mut root_fuck);
                        map.insert(file_name.to_string(), Fuck::File(file_size));
                    }
                }
            },
            _ => ()
        }
    }
    root_fuck
}
