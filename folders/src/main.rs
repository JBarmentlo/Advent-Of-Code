use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum OldFsObject { 
    File(u32),
    Folder(HashMap<String, OldFsObject>)
}


#[derive(Debug, Default)]
struct Folder {
    name: String,
    size: Option<u32>,
    contents: HashMap<String, FsObject>
}

impl Folder {
    fn get_nested_folder(&mut self, mut names: impl Iterator<Item=&'a String>) -> &mut Folder {
        match names.next() {
            None => self,
            Some(name) => {
                match self.contents.get_mut(name).expect("Looking for missing folder") {
                    FsObject::Folder(ref mut folder) => {
                        folder.get_nested_folder(names)
                    }
                    _ => panic!("cd into a file"),
                }
            }
        }
    }    
}

#[derive(Debug, Default)]
struct File {
    name: String,
    size: Option<u32>,
}

#[derive(Debug, Default)]
enum FsObject { 
    File(File),
    Folder(Folder)
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
    parse_new(&contents);
    let sum = sum_folders_smaller_than(&root, 100000);
    println!("The first answer is {0}", sum.counted);

    let total_space: i32 = 70000000;
    let required_space: i32  = 30000000;
    let missing_space: i32 = required_space - total_space + sum.total as i32;
    println!("Missing space: {missing_space}");
    // if missing_space < 0 {
    //     panic!("nope");
    // } else {
    //     find_smallest_folder_bigger_than("root".to_string(), &root, missing_space as u32);
    // }
}


fn sum_folders_smaller_than(root: &OldFsObject, max_limit: u32) -> SizeCounter {
    match root {
        OldFsObject::File(size) => {
            SizeCounter{
                total: *size, 
                counted: 0,
            }
        },
        OldFsObject::Folder(map) => {
            let mut out = map.values()
            .map(|f| sum_folders_smaller_than(f, max_limit))
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

fn get_mut_recursive<'a>(mut names: impl Iterator<Item=&'a String>, fuck: &mut OldFsObject) -> &mut HashMap<String, OldFsObject> {
    match names.next() {
        None => {
            if let OldFsObject::Folder(ref mut map) = fuck {
                map
            } else {
                panic!("fuck")
            }
        },
        Some(name) => {
            if let OldFsObject::Folder(map) = fuck {
                get_mut_recursive(names, map.get_mut(name).expect("Fuck"))
            } else {
                panic!("fuck")
            }
        }
    }
}




fn parse_new(text: &String) -> Folder {
    let mut root = Folder::default();
    let mut current_dirs: Vec<String> = Vec::new();

}


fn parse(text: &String) -> OldFsObject {
    let mut root_fuck = OldFsObject::Folder(HashMap::new());
    let mut current_fucks: Vec<String> = Vec::new();
    get_mut_recursive(current_fucks.iter(), &mut root_fuck).insert("root".to_string(), OldFsObject::Folder(HashMap::new()));
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
                    map.entry(arg.to_string()).or_insert(OldFsObject::Folder(HashMap::new()));
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
                        map.insert(file_name.to_string(), OldFsObject::File(file_size));
                    }
                }
            },
            _ => ()
        }
    }
    root_fuck
}
