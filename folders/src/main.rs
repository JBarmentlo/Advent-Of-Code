use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::fs;
use std::io::Cursor;
use std::iter::Filter;

fn main() { 
    println!("Hello, world!");
    let contents = fs::read_to_string("test_data.txt").expect("The file is static and is always parsable");
    // let contents = fs::read_to_string("data.txt").expect("The file is static and is always parsable");
    let root = parse(&contents);
    dbg!(&root);
    let sum = sum_larger(&"root".to_string(), &root, 100000);
    dbg!(sum);
}

#[derive(Debug)]
enum Fuck { 
    File(u32),
    Folder(HashMap<String, Fuck>)
}



fn sum_larger(name: &String, root: &Fuck, max_limit: u32) -> (u32, u32) {
    // if let Fuck::Folder(map) = root {
    //     map.iter().map(|f|)
    // }
    match root {
        Fuck::File(size) => {
            (*size, *size)
        },
        Fuck::Folder(map) => {
            map.iter()
            .map(|(_, f)| sum_larger(f, max_limit))
            // .filter(|(total_size, counted_size)| *total_size > min_limit && *total_size < max_limit)
            .fold((0, 0), |a, b| {
                let total_size = a.0 + b.0;
                let mut counted_size = 0;
                if a.1 < max_limit{
                    counted_size += a.1;
                }
                if b.1 < max_limit{
                    counted_size += b.1;
                }
                (total_size, counted_size)
            })
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

        println!("{command_line}");
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
                    println!("\t{line}");
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
        println!();
        println!();
    }
    root_fuck
}

// enum FsContent {
//     File(String, u32),
//     Folder(String, Vec<FsContent>)
// }

// fn parse_file_map(files: HashMap<String, u32>) -> Vec<FsContent> {
//     // Folder(files.iter().filter)
//     let mut keys: Vec<&String> = files.keys().collect();
//     keys.sort();
//     for key in keys {
//         let value = files.get(key).unwrap();
//         let count = key.split("/").count() - 1;
//         println!("{count} : {key} : {value}");
//     }
//     dbg!(&files);
//     create_folder(files, 2)
//     // files.iter()
// }

// fn is_dir(filename: &String) -> bool {
//     let count = filename.split("/").count();
//     println!("name: {filename}: {count}");
    
//     filename.split("/").count() > 1
// }

// // process_iterable<T: IntoIterator<Item = (String, u32)>>(iterable: T)
// fn create_folder(filenames: HashMap<String, u32>, i: u32) -> Vec<FsContent> {
// // fn create_folder<T: Iterator<Item = (String, u32)> + Clone>(mut filenames: T) -> Vec<FsContent> {
//     if i == 0 {
//         let mut r: Vec<FsContent> = Vec::new();
//         r.push(FsContent::File("end".to_string(), 0));
//         return r
//     }
//     println!();
//     println!("Folder from: {i}");
//     dbg!(&filenames);
//     filenames.clone().drain().map(
//         |(name, size)| {
//             if !is_dir(&name) {
//                 FsContent::File(name.clone(), size)
//             } else {
//                 let folder_name = name.split_once("/").unwrap().0;
//                 let folder_filenames = filenames.clone().drain().filter(|(name, size)| name.starts_with(folder_name)).collect();
//                 FsContent::Folder(
//                     name.clone(), 
//                     create_folder(folder_filenames, i - 1)
//                 )
//             }
//         }
//     ).collect()
// }
