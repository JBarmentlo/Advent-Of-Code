use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::fs;
use std::iter::Filter;

fn main() { 
    println!("Hello, world!");
    let contents = fs::read_to_string("test_data.txt").expect("The file is static and is always parsable");
    // let contents = fs::read_to_string("data.txt").expect("The file is static and is always parsable");
    parse(&contents);
}

fn parse(text: &String) {
    let mut folder_names: Vec<String> = Vec::new();
    let mut files: HashMap<String, u32> = HashMap::new();
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
                    folder_names.pop();
                } else if arg == "/" {
                    folder_names = Vec::new();
                } else {
                    folder_names.push(arg.to_string());
                }

                let cwd = folder_names.join("/");
                println!("current_folder: {cwd}");
            },
            
            "ls" => {
                let cwd = folder_names.join("/");
                println!("current_folder: {cwd}");
                for line in respones_lines {
                    println!("\t{line}");
                    let trimmed_line = line.trim();
                    if  !trimmed_line.starts_with("dir") {
                        let mut words = trimmed_line.split_whitespace();
                        let file_size: u32 = words.next().unwrap().parse().expect("Std format expected");
                        let file_name = words.next().unwrap();
                        folder_names.push(file_name.to_string());
                        let full_path = folder_names.join("/");
                        folder_names.pop();
                        println!("\t\tAdd: {full_path}: {file_size}");
                        files.insert(full_path, file_size);
                    }
                }
            },

            _ => ()
        }
        println!();
        println!();
    }
    dbg!(&files);
    let total_size: u32 = files.values().sum();
    dbg!(total_size);
    parse_file_map(files);
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
    create_folder(files.iter().map(|(&k, &v)| (k.clone(), v)))
    // files.iter()
}

fn is_dir(filename: &String) -> bool {
    filename.split("/").count() == 1
}
// process_iterable<T: IntoIterator<Item = (String, u32)>>(iterable: T)
// fn create_folder(mut filenames: Filter<Iter<String, u32>, _>) -> Vec<FsContent> {
fn create_folder<T: Iterator<Item = (String, u32)> + Clone>(mut filenames: T) -> Vec<FsContent> {
    filenames.clone().into_iter().map(
        |(name, size)| {
            if !is_dir(&name) {
                FsContent::File(name.clone(), size)
            } else {
                let folder_name = name.split_once("/").unwrap().0;
                FsContent::Folder(
                    name.clone(), 
                    create_folder(filenames.clone().filter(|(name, size)| name.starts_with(folder_name)).into_iter())
                )
            }
        }
    ).collect()
}
