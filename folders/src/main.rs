use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::fs;

#[derive(Debug)]
enum FsObject { 
    File(u32),
    Folder(HashMap<String, FsObject>)
}


#[derive(Debug, Default)]
struct SizeCounter {
    total: u32,
    counted: u32
}

fn main() { 
    println!("Hello, world!");
    let contents = fs::read_to_string("data.txt").expect("The file is static and is always parsable");
    let root = parse(&contents);
    let sum = sum_larger(&root, 100000);
    dbg!(sum);
}




fn sum_larger(root: &FsObject, max_limit: u32) -> SizeCounter {
    match root {
        FsObject::File(size) => {
            SizeCounter{
                total: *size, 
                counted: 0,
            }
        },
        FsObject::Folder(map) => {
            let mut out = map.values()
            .map(|f| sum_larger(f, max_limit))
            .fold(
                SizeCounter::default(), 
                |a, b| SizeCounter{
                    total: a.total + b.total,
                    counted: a.counted + b.counted,
                }
            );
            
            if out.total < max_limit {
                out.counted += out.total;
            }

            out
        }
    }
}

fn get_mut_recursive<'a>(mut names: impl Iterator<Item=&'a String>, fuck: &mut FsObject) -> &mut HashMap<String, FsObject> {
    match names.next() {
        None => {
            if let FsObject::Folder(ref mut map) = fuck {
                map
            } else {
                panic!("fuck")
            }
        },
        Some(name) => {
            if let FsObject::Folder(map) = fuck {
                get_mut_recursive(names, map.get_mut(name).expect("Fuck"))
            } else {
                panic!("fuck")
            }
        }
    }
}

fn parse(text: &String) -> FsObject {
    let mut root_fuck = FsObject::Folder(HashMap::new());
    let mut current_fucks: Vec<String> = Vec::new();
    get_mut_recursive(current_fucks.iter(), &mut root_fuck).insert("root".to_string(), FsObject::Folder(HashMap::new()));
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
                    map.entry(arg.to_string()).or_insert(FsObject::Folder(HashMap::new()));
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
                        map.insert(file_name.to_string(), FsObject::File(file_size));
                    }
                }
            },
            _ => ()
        }
    }
    root_fuck
}
