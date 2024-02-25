use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::fs;
use std::io::Cursor;
use std::iter::Filter;

fn main() { 
    println!("Hello, world!");
    let contents = fs::read_to_string("test_data.txt").expect("The file is static and is always parsable");
    // let contents = fs::read_to_string("data.txt").expect("The file is static and is always parsable");
    parse(&contents);
}

enum Fuck { 
    File(u32),
    Folder(HashMap<String, Fuck>)
}

fn get_mut_recursive(mut names: impl Iterator<Item=&String>, fuck: &Fuck) -> &mut HashMap<String, Fuck> {
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
                get_mut_recursive(names, map.get(name).expect("Fuck"))
            } else {
                panic!("fuck")
            }
        }
    }
}

fn parse(text: &String) {
    let mut root_fuck = Fuck::Folder(HashMap::new());
    // let mut current_fucks: Vec<&Fuck> = Vec::new();
    // current_fucks.push(&mut root_fuck);
    let mut current_fucks: Vec<String> = Vec::new();
    current_fucks.push("root".to_string());
    get_mut_recursive(current_fucks.iter(), &root_fuck).insert("root".to_string(), Fuck::Folder(HashMap::new()));


    let blocks = text.split("$ ")
        .skip(2);

    for block in blocks {
        let mut lines = block.lines();
        let command_line = lines.next().expect("every block has a first line").trim();
        let mut command_line_words = command_line.split_whitespace();
        let cmd = command_line_words.next().expect("always here");
        let arg = command_line_words.next();
        let respones_lines = lines;

        println!("{command_line}");
        match cmd {
            "cd" => {
                let arg = arg.expect("always arg for cd");
                if arg == ".." {
                    current_fucks.pop();
                } else if arg == "/" {
                    current_fucks = vec!["root".to_string()];
                } else {
                    
                    // TODO: handle alread exists
                    if let Fuck::Folder(ref mut map) = root_fuck {
                        map.insert("root".to_string(), Fuck::Folder(HashMap::new()));
                    }

                }
            },
            
            "ls" => {
                for line in respones_lines {
                    println!("\t{line}");
                    let trimmed_line = line.trim();
                    
                    if !trimmed_line.starts_with("dir") {
                        let mut words = trimmed_line.split_whitespace();
                        let file_size: u32 = words.next().unwrap().parse().expect("Std format expected");
                        let file_name = words.next().unwrap();

                        match current_fucks.last_mut().expect("Made it myself") {
                            Fuck::File(size) => panic!(),
                            Fuck::Folder(map) => map.insert(file_name.to_string(), Fuck::File(file_size)),
                        };
                    } else {
                        // let mut words = trimmed_line.split_whitespace();
                        
                        // let dir_name = words.nth(1).unwrap();

                        // match current_fucks.last_mut().expect("Made it myself") {
                        //     Fuck::File(size) => panic!(),
                        //     Fuck::Folder(mut map) => map.insert(dir_name.to_string(), Fuck::Folder(HashMap::new())),
                        // };
                    }
                }
            },

            _ => ()
        }
        println!();
        println!();
    }
}

enum FsContent {
    File(String, u32),
    Folder(String, Vec<FsContent>)
}

fn parse_file_map(files: HashMap<String, u32>) -> Vec<FsContent> {
    // Folder(files.iter().filter)
    let mut keys: Vec<&String> = files.keys().collect();
    keys.sort();
    for key in keys {
        let value = files.get(key).unwrap();
        let count = key.split("/").count() - 1;
        println!("{count} : {key} : {value}");
    }
    dbg!(&files);
    create_folder(files, 2)
    // files.iter()
}

fn is_dir(filename: &String) -> bool {
    let count = filename.split("/").count();
    println!("name: {filename}: {count}");
    
    filename.split("/").count() > 1
}

// process_iterable<T: IntoIterator<Item = (String, u32)>>(iterable: T)
fn create_folder(filenames: HashMap<String, u32>, i: u32) -> Vec<FsContent> {
// fn create_folder<T: Iterator<Item = (String, u32)> + Clone>(mut filenames: T) -> Vec<FsContent> {
    if i == 0 {
        let mut r: Vec<FsContent> = Vec::new();
        r.push(FsContent::File("end".to_string(), 0));
        return r
    }
    println!();
    println!("Folder from: {i}");
    dbg!(&filenames);
    filenames.clone().drain().map(
        |(name, size)| {
            if !is_dir(&name) {
                FsContent::File(name.clone(), size)
            } else {
                let folder_name = name.split_once("/").unwrap().0;
                let folder_filenames = filenames.clone().drain().filter(|(name, size)| name.starts_with(folder_name)).collect();
                FsContent::Folder(
                    name.clone(), 
                    create_folder(folder_filenames, i - 1)
                )
            }
        }
    ).collect()
}
